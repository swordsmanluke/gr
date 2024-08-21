mod gr;
mod config;
mod indent;

use anyhow::{anyhow, Result};
use colored::Colorize;
use candy::candy::Candy;
use candy::events::CandyEvent;
use candy::events::CandyEvent::Select;
use gr_git::{ExecGit, Git};
use gr::{initialize_gr, move_relative};
use crate::gr::{merge, restack, reviews, submit, log};

#[tokio::main]
async fn main() -> Result<()> {
    // Read the arguments from the command line
    let mut args = std::env::args().collect::<Vec<String>>();
    let candy = Candy::new();
    args.reverse();
    args.pop(); // Remove the first argument, which is the name of the program

    let res = match args.pop() {
        Some(command) => process_command(command, &mut args, &candy).await,
        None => {
            println!("No argument provided");
            Ok(())
        }
    };

    println!(); // ...and we're done - return the result and exit.

    if let Err(e) = res {
        println!("Error occurred: {}", e.to_string().red());
        return Err(e);
    } else {
        println!("{}", "Done".green());
    }
    Ok(())
}

async fn process_command(command: String, args: &mut Vec<String>, candy: &Candy) -> Result<()> {
    let git = Git::new();
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
        "submit" => {
            let cfg = config::read_config()?;
            submit(&cfg.code_review_tool, &cfg.origin).await?;
            println!("{}", "Done".green());
        }
        "sync" => {
            println!("{}", "Syncing current stack...".green());
            restack()?;
            println!("{}", "Complete".green());
        }
        "top" | "up" | "down" | "bottom" | "bu" | "bd" => {
            move_relative(&command)?;
            println!("Checked out branch: {}", git.current_branch()?.green());
            let egit = ExecGit::new();
            egit.status()?; // Exits gr and hands control to git
        }
        _ => { println!("Unknown command: {}", command) }
    }
    Ok(())
}


fn select_branch() -> Result<String> {
    let git = Git::new();
    let candy = Candy::new();
    let branches = git.branch(vec![])?;
    let options = branches.lines().map(|s| s.to_string()).collect();

    let selection = candy.choose_option("Select a branch", options, false);

    match selection {
        Select(v) => Ok(v[0].clone()),
        _ => Err(anyhow!("No branch selected")),
    }
}
