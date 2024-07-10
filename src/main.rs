mod gr;
mod config;

use std::error::Error;
use colored::Colorize;
use gr_tui::TuiWidget;
use gr_git::{ExecGit, Git};
use gr_reviews::review_service_for;
use gr::{initialize_gr, move_relative};
use url::Url;
use crate::gr::restack;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut tui = TuiWidget::new();

    // Read the arguments from the command line
    let mut args = std::env::args().collect::<Vec<String>>();
    args.reverse();
    args.pop(); // Remove the first argument, which is the name of the program

    let res = match args.pop() {
        Some(command) => process_command(command, &mut args, &mut tui).await,
        None => { println!("No argument provided"); Ok(()) },
    };

    tui.exit_raw_mode();  // Not guaranteed to be in raw mode, but it's a good idea, just in case.
    println!(); // ...and we're done - return the result and exit.

    if let Err(e) = res {
        println!("Error occurred: {}", e.to_string().red());
        return Err(e);
    } else {
        println!("{}", "Done".green());
    }
    Ok(())
}

async fn process_command(command: String, mut args: &mut Vec<String>, tui: &mut TuiWidget<'_>) -> Result<(), Box<dyn Error>>{
    let git = Git::new();
    match command.as_str() {
        "bco" | "switch" => {
            let branch = args.first().unwrap_or(&select_branch(tui)?).to_owned();
            git.switch(&branch)?;
            println!("Checked out branch: {}", branch.green());
            println!("{}", git.status()?.green());
        },
        "bc" | "create" => {
            let cur_branch = git.current_branch()?;
            let branch = match args.pop() {
                Some(b) => b,
                None => tui.prompt("Branch name: ")?,
            };

            git.checkout(vec!["-t", &cur_branch, "-b", &branch])?;
            println!("Created branch: {}", branch.green());
        },
        "cc" | "commit" => {
            let git = ExecGit::new();
            let mut new_args = Vec::new();
            args.into_iter().for_each(|s| new_args.insert(0, s.to_owned()));
            git.commit(new_args)?;
            // ExecGit should take over the process - we won't return here.
        }
        "init" => {
            initialize_gr(tui)?;
            println!("Initialized gr config");
        },
        "rv" | "reviews" => {
            let config = config::read_config()?;
            let service = review_service_for(&config.code_review_tool)?;

            if service.is_none() {
                println!("No review service configured");
                return Ok(());
            }

            let reviews = service.unwrap().reviews().await?;
            for r in reviews
            {
                let url =match r.url {
                    Some(url) => url.to_string(),
                    None => "".to_string(),
                };

                println!("{}: {}", r.id.cyan().bold(), r.state.to_string().yellow());
                println!("  {}", r.title.blue());
                println!("  {}", url);
            }
        }
        "submit" => {
            println!("Not implemented");
        },
        "sync" => {
            println!("{}", "Syncing current stack...".green());
            restack()?;
            println!("{}", "Complete".green());
        }
        "top" | "up" | "down" | "bottom" | "bu" | "bd" => {
            move_relative(tui, &command)?;
            println!("Checked out branch: {}", git.current_branch()?.green());
            let egit = ExecGit::new();
            egit.status()?; // Exits gr and hands control to git
        },
        _ => { println!("Unknown command: {}", command) },
    }
    Ok(())
}


fn select_branch(tui: &mut TuiWidget) -> Result<String, Box<dyn Error>>{
    let git = Git::new();
    let branches = git.branch("")?;
    let options = branches.lines().map(|s| s.to_string()).collect();

    let selection = tui.select_one("Select a branch".to_string(), options)?;

    match selection {
        Some(b) => Ok(b),
        None => Err("No branch selected".into()),
    }
}
