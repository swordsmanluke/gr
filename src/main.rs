use std::error::Error;
use gr_tui::Tui;
use gr_git::Git;
use gr_tui::string_helpers::Colorize;

fn main() -> Result<(), Box<dyn Error>> {
    let mut tui = Tui::new();

    // Read the arguments from the command line
    let mut args = std::env::args().collect::<Vec<String>>();
    args.reverse();
    tui.println(format!("ARGS: {:?}", args));
    args.pop(); // Remove the first argument, which is the name of the program

    let res = match args.pop() {
        Some(arg) => handle_argument(arg, args, &mut tui),
        None => { println!("No argument provided"); Ok(()) },
    };


    res
}

fn handle_argument(command: String, args: Vec<String>, tui: &mut Tui) -> Result<(), Box<dyn Error>>{
    match command.as_str() {
        "bco" => {
            let branch = select_branch(tui)?;
            let git = Git::new();
            git.switch(&branch)?;
            Ok(tui.println(format!("Checked out branch: {}", branch.green())))
        },
        _ => { println!("Unknown command: {}", command); Ok(()) },
    }
}


fn select_branch(tui: &mut Tui) -> Result<String, Box<dyn Error>>{
    let git = Git::new();
    let branches = git.branch("")?;
    tui.println(format!("BRANCHES: {}", branches.clone().lines().count()));

    let options = branches.lines().map(|s| s.to_string()).collect();

    let selected_options = tui.select(options, Some("Select a branch".to_string()), false)?;

    match selected_options {
        Some(b) => Ok(b.first().expect("Expected a branch name!").to_string()),
        None => Err("No branch selected".into()),
    }
}
