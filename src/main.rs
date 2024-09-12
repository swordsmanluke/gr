mod gr;
mod config;
mod indent;

use anyhow::{anyhow, Result};
use colored::Colorize;
use candy::candy::{Candy};
use candy::events::CandyEvent;
use candy::events::CandyEvent::Select;
use gr_git::{BranchType, ExecGit, Git};
use gr::{initialize_gr, move_relative};
use crate::gr::{merge, sync, reviews, submit, log, help, split};
use gr::submit::get_commit_message;
use help::{show_usage, show_help};

#[tokio::main]
async fn main() -> Result<()> {
    // Read the arguments from the command line
    let mut args = std::env::args().collect::<Vec<String>>();
    let candy = Candy::new();
    args.reverse();
    args.pop(); // Remove the first argument, which is the name of the program

    let res = match args.pop() {
        Some(command) => process_command(command, &mut args).await,
        None => {
            println!("No argument provided");
            Ok(())
        }
    };

    println!(); // ...and we're done - return the result and exit.

    if let Err(e) = res {
        println!("Error occurred: {}", e.to_string().red());
        return Err(e);
    }
    Ok(())
}

async fn process_command(command: String, args: &mut Vec<String>) -> Result<()> {
    let git = Git::new();
    let candy = Candy::new();

    match command.as_str() {
        "bco" | "switch" => {
            let branch = args.first().unwrap_or(&select_branch()?).to_owned();
            git.switch(&branch)?;
            println!("Checked out branch: {}", branch.green());
            println!("{}", git.status()?.green());
        }
        "bc" | "create" => {
            let cur_branch = git.current_branch()?;
            let branch = match args.pop() {
                Some(b) => b,
                None => match candy.edit_line("Branch name: ", None) {
                    CandyEvent::Submit(b) => b,
                    _ => return Err(anyhow!("No branch name provided")),
                },
            };

            git.checkout(vec!["-t", &cur_branch, "-b", &branch])?;
            println!("Created branch: {}", branch.green());
        }
        "cc" | "commit" => {
            let git = ExecGit::new();
            let mut new_args = Vec::new();
            args.into_iter().for_each(|s| new_args.insert(0, s.to_owned()));
            git.commit(new_args)?;
            // ExecGit should take over the process - we won't return here.
        }
        "help" => {
            match args.first() {
                Some(arg) => show_help(arg),
                None => show_usage(),
            }
        }
        "init" => {
            initialize_gr()?;
            println!("Initialized gr config");
        }
        "log" => {
            log()?;
        }
        "merge" => {
            let conf = &config::read_config()?;
            merge(&conf.code_review_tool, &conf.origin).await?;
        }
        "rv" | "reviews" => {
            let config = config::read_config()?;
            let revs = reviews(&config.code_review_tool).await?;

            for r in revs
            {
                let url = match r.url {
                    Some(url) => url.to_string(),
                    None => "".to_string(),
                };

                println!("{} {}", r.title, format!("({})", r.state).yellow());
                println!("  {} -> {}", r.branch.cyan(), r.base.magenta());
                println!("  {}", url);
            }
        }
        "split" => {
            split()?;
        }
        "submit" => {
            let cfg = config::read_config()?;
            submit(&cfg.code_review_tool, &cfg.origin).await?;
        }
        "sync" => {
            println!("{}", "Syncing current stack...".green());
            sync()?;
            println!("{}", "Complete".green());
        }
        "top" | "up" | "down" | "bottom" | "bu" | "bd" => {
            move_relative(&command)?;
            println!("Checked out branch: {}", git.current_branch()?.green());
            let egit = ExecGit::new();
            egit.status()?; // Exits gr and hands control to git
        }
        "debug" => {
            // Secret command menu
            let next = args.pop();
            match next {
                Some(arg) => {
                    match arg.as_str() {
                        "submit" => {
                            println!("Debugging 'submit'.");
                            let cur_branch = git.current_branch()?;
                            let parent = git.parent_of(&cur_branch, BranchType::Local)?;
                            let msg = get_commit_message(&cur_branch, parent.unwrap().as_str())?;
                            println!("Commit Message\n======\n{}", msg.join("\n"));

                            let title = match msg.get(0) {
                                Some(t) => t.to_string(),
                                None => "".to_string(),
                            };
                            println!("\n\nCommit title: {}", title);
                        }
                        "prompt" => {
                            let prompt = candy.edit_line("Enter a prompt: ", None);
                            match prompt {
                                CandyEvent::Submit(s) => { println!("You entered: {}", s) }
                                _ => { println!("Canceled") }
                            }
                        }
                        _ => { println!("Unknown debug command: {}", arg) }
                    }
                }
                None => { println!("Missing debug command") }
            }
        }
        _ => { println!("FWD TO GIT: Unknown command: {}", command) }
    }
    Ok(())
}


fn select_branch() -> Result<String> {
    let git = Git::new();
    let candy = Candy::new();
    let branches = git.branch(vec![])?;
    let options = branches.lines().map(|s| s.to_string()).collect();

    let selection = candy.choose_option("Select a branch", options, None,false);

    match selection {
        Select(v) => Ok(v[0].clone()),
        _ => Err(anyhow!("No branch selected")),
    }
}
