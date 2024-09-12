use anyhow::Result;
use colored::Colorize;
use candy::candy::Candy;
use candy::events::CandyEvent::Submit;
use gr_git::{BranchType, Git};

pub const MOVE_USAGE: [&str; 4] = [
"stk move <up | bu>

Move up in the stack. If the current branch has multiple children, you will be prompted to
select one.",

"stk move <down | bd>

Move down in the stack.",

"stk move <bottom | bb>

Move to the bottom of the stack - this is the root of the current repo.",

"stk move <top | bt>

Move to the top of the stack - if any branch has multiple children, you will be prompted to
select which branch to follow.",
];

pub fn move_relative(command: &str) -> Result<()> {
    let git = Git::new();
    match command {
        "bu" | "up" => { move_up(&git)?; },
        "bd" | "down" => { move_down(&git)?; },
        "top" => {
            let mut cur_branch = git.current_branch()?;
            while git.children_of(&cur_branch)?.len() > 0 {
                move_up(&git)?;
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

fn move_down(git: &Git) -> Result<()> {
    let cur_branch = git.current_branch()?;

    let parent = git.parent_of(&cur_branch, BranchType::Local)?;

    match parent {
        None => { println!("{}", "You are already at the bottom of the stack".green()) },
        Some(p) => { git.checkout(vec![&p])? }
    }
    Ok(())
}

fn move_up(git: &Git) -> Result<()> {
    let cur_branch= git.current_branch()?;
    let candy = Candy::new();

    let children = git.children_of(&cur_branch)?;
    match children.len() {
        0 => { println!("{}", "You are already at the top of the stack".green()) },
        1 => { git.checkout(vec![&children[0]])? }
        _ => {
            match candy.select_one("Which branch do you want to go up to?", children, None) {
                Submit(c) => { git.checkout(vec![&c])? }
                _ => {}, // canceled - do nothing.
            }
        }
    }
    Ok(())
}