use std::error::Error;
use gr_git::Git;
use gr_tui::string_helpers::Colorize;
use gr_tui::Tui;

pub fn move_relative(tui: &mut Tui, command: &str) -> Result<(), Box<dyn Error>> {
    let git = Git::new();
    match command {
        "bu" | "up" => { move_up(tui, &git)?; },
        "bd" | "down" => { move_down(tui, &git)?; },
        "top" => {
            let mut cur_branch = git.current_branch()?;
            while git.children_of(&cur_branch)?.len() > 0 {
                move_up(tui, &git)?;
                cur_branch = git.current_branch()?
            }
        },
        "bottom" => {
            let mut cur_branch = git.current_branch()?;
            while git.parent_of(&cur_branch)?.is_some() {
                move_down(tui, &git)?;
                cur_branch = git.current_branch()?
            }
        },
        _ => { println!("Unknown command: {}", command) },
    }

    Ok(())
}

fn move_down(tui: &mut Tui, git: &Git) -> Result<(), Box<dyn Error>> {
    let cur_branch = git.current_branch()?;

    let parent = git.parent_of(&cur_branch)?;
    match parent {
        None => { tui.println("You are already at the bottom of the stack".green()) },
        Some(p) => { git.checkout(&p)? }
    }
    Ok(())
}

fn move_up(tui: &mut Tui, git: &Git) -> Result<(), Box<dyn Error>> {
    let cur_branch= git.current_branch()?;

    let children = git.children_of(&cur_branch)?;
    match children.len() {
        0 => { tui.println("You are already at the top of the stack".green()) },
        1 => { git.checkout(&children[0])? }
        _ => {
            let child = tui.select_one("Which branch do you want to go up to?".into(), children)?;
            match child {
                None => {}, // canceled - do nothing.
                Some(c) => { git.checkout(&c)? }
            }
        }
    }
    Ok(())
}