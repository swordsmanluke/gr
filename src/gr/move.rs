use std::error::Error;
use colored::Colorize;
use gr_git::{BranchType, Git};
use gr_tui::TuiWidget;

pub fn move_relative(tui: &mut TuiWidget, command: &str) -> Result<(), Box<dyn Error>> {
    let git = Git::new();
    match command {
        "bu" | "up" => { move_up(tui, &git)?; },
        "bd" | "down" => { move_down(&git)?; },
        "top" => {
            let mut cur_branch = git.current_branch()?;
            while git.children_of(&cur_branch)?.len() > 0 {
                move_up(tui, &git)?;
                cur_branch = git.current_branch()?
            }
        },
        "bottom" => {
            let mut cur_branch = git.current_branch()?;
            while git.parent_of(&cur_branch, BranchType::Local)?.is_some() {
                move_down(&git)?;
                cur_branch = git.current_branch()?
            }
        },
        _ => { println!("Unknown command: {}", command) },
    }

    Ok(())
}

fn move_down(git: &Git) -> Result<(), Box<dyn Error>> {
    let cur_branch = git.current_branch()?;

    let parent = git.parent_of(&cur_branch, BranchType::Local)?;

    match parent {
        None => { println!("{}", "You are already at the bottom of the stack".green()) },
        Some(p) => { git.checkout(vec![&p])? }
    }
    Ok(())
}

fn move_up(tui: &mut TuiWidget, git: &Git) -> Result<(), Box<dyn Error>> {
    let cur_branch= git.current_branch()?;

    let children = git.children_of(&cur_branch)?;
    match children.len() {
        0 => { println!("{}", "You are already at the top of the stack".green()) },
        1 => { git.checkout(vec![&children[0]])? }
        _ => {
            let child = tui.select_one("Which branch do you want to go up to?".into(), children)?;
            match child {
                None => {}, // canceled - do nothing.
                Some(c) => { git.checkout(vec![&c])? }
            }
        }
    }
    Ok(())
}