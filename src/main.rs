mod init;
mod config;

use std::error::Error;
use gr_tui::Tui;
use gr_git::Git;
use gr_tui::string_helpers::{Colorize, GrString};
use crate::init::initialize_gr;

fn main() -> Result<(), Box<dyn Error>> {
    let mut tui = Tui::new();

    // Read the arguments from the command line
    let mut args = std::env::args().collect::<Vec<String>>();
    args.reverse();
    args.pop(); // Remove the first argument, which is the name of the program
    args.push("init".to_string());

    let res = match args.pop() {
        Some(command) => process_command(command, args, &mut tui),
        None => { println!("No argument provided"); Ok(()) },
    };

    tui.exit_raw_mode();  // Not guaranteed to be in raw mode, but it's a good idea, just in case.
    println!(); // ...and we're done - return the result and exit.
    res
}

fn process_command(command: String, args: Vec<String>, tui: &mut Tui) -> Result<(), Box<dyn Error>>{
    match command.as_str() {
        "bco" | "switch" => {
            let branch = args.first().unwrap_or(&select_branch(tui)?).to_owned();
            let git = Git::new();
            git.switch(&branch)?;
            Ok(tui.println((GrString::from("Checked out branch: ") + branch.green())))
        },
        "bc" | "create" => {
            let git = Git::new();
            let cur_branch = args.first().unwrap_or(&git.current_branch()?).to_owned();
            let branch = tui.prompt("Branch name:".green() + " ")?;
            git.checkout(&format!("-t {} -b {}", cur_branch, branch))?;
            Ok(tui.println(("Created branch: ".default() + branch.green())))
        },
        "init" => {
            initialize_gr(tui)?;
            Ok(tui.println("Initialized gr config".into()))
        }
        _ => { println!("Unknown command: {}", command); Ok(()) },
    }
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
