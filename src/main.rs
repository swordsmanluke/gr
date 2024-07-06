mod gr;
mod config;

use std::error::Error;
use gr_tui::Tui;
use gr_git::{ExecGit, Git};
use gr_tui::string_helpers::{Colorize, GrString};
use gr::{initialize_gr, move_relative};

fn main() -> Result<(), Box<dyn Error>> {
    let mut tui = Tui::new();

    // Read the arguments from the command line
    let mut args = std::env::args().collect::<Vec<String>>();
    args.reverse();
    args.pop(); // Remove the first argument, which is the name of the program

    let res = match args.pop() {
        Some(command) => process_command(command, args, &mut tui),
        None => { println!("No argument provided"); Ok(()) },
    };

    tui.exit_raw_mode();  // Not guaranteed to be in raw mode, but it's a good idea, just in case.
    println!(); // ...and we're done - return the result and exit.
    res
}

fn process_command(command: String, args: Vec<String>, tui: &mut Tui) -> Result<(), Box<dyn Error>>{
    let git = Git::new();
    match command.as_str() {
        "bco" | "switch" => {
            let branch = args.first().unwrap_or(&select_branch(tui)?).to_owned();
            let git = Git::new();
            git.switch(&branch)?;
            tui.println(GrString::from("Checked out branch: ") + branch.green());
            tui.println(git.status()?.green());
        },
        "bc" | "create" => {
            let git = Git::new();
            let cur_branch = args.first().unwrap_or(&git.current_branch()?).to_owned();
            let branch = tui.prompt("Branch name:".green() + " ")?;
            git.checkout(&format!("-t {} -b {}", cur_branch, branch))?;
            tui.println("Created branch: ".default() + branch.green());
        },
        "init" => {
            initialize_gr(tui)?;
            tui.println("Initialized gr config".into());
        },
        "top" | "up" | "down" | "bottom" | "bu" | "bd" => {
            move_relative(tui, &command)?;
            tui.println(GrString::from("Checked out branch: ") + git.current_branch()?.green());
            let egit = ExecGit::new();
            tui.exit_alt_screen();
            tui.exit_raw_mode();
            println!();  // Clear space to the next line.
            egit.status()?; // Exits gr and hands control to git
        },
        "cc" | "commit" => {
            let git = ExecGit::new();
            let mut new_args = Vec::new();
            args.into_iter().for_each(|s| new_args.insert(0, s.to_owned()));
            tui.exit_alt_screen();
            tui.exit_raw_mode();
            println!();  // Clear space to the next line.

            git.commit(new_args)?;
            // ExecGit should take over the process - we won't return here.
        }
        _ => { println!("Unknown command: {}", command) },
    }
    Ok(())
}


fn select_branch(tui: &mut Tui) -> Result<String, Box<dyn Error>>{
    let git = Git::new();
    let branches = git.branch("")?;
    let options = branches.lines().map(|s| s.to_string()).collect();

    let selection = tui.select_one("Select a branch".to_string(), options)?;

    match selection {
        Some(b) => Ok(b),
        None => Err("No branch selected".into()),
    }
}
