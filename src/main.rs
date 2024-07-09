mod gr;
mod config;

use std::error::Error;
use colored::Colorize;
use gr_tui::TuiWidget;
use gr_git::{ExecGit, Git};
use gr::{initialize_gr, move_relative};
use crate::gr::restack;

fn main() -> Result<(), Box<dyn Error>> {
    let mut tui = TuiWidget::new();

    // Read the arguments from the command line
    let mut args = std::env::args().collect::<Vec<String>>();
    args.reverse();
    args.pop(); // Remove the first argument, which is the name of the program

    let res = match args.pop() {
        Some(command) => process_command(command, &mut args, &mut tui),
        None => { println!("No argument provided"); Ok(()) },
    };

    tui.exit_raw_mode();  // Not guaranteed to be in raw mode, but it's a good idea, just in case.
    println!(); // ...and we're done - return the result and exit.
    res
}

fn process_command(command: String, mut args: &mut Vec<String>, tui: &mut TuiWidget) -> Result<(), Box<dyn Error>>{
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
        "top" | "up" | "down" | "bottom" | "bu" | "bd" => {
            move_relative(tui, &command)?;
            println!("Checked out branch: {}", git.current_branch()?.green());
            let egit = ExecGit::new();
            egit.status()?; // Exits gr and hands control to git
        },
        "sync" => {
            println!("{}", "Syncing current stack...".green());
            restack()?;
            println!("{}", "Complete".green());
        }
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
