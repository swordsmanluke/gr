use std::error::Error;
use colored::Colorize;
use gr_git::Git;
use gr_tui::symbols::{CHECK, CROSS};

pub fn restack() -> Result<(), Box<dyn Error>> {
    let git = Git::new();

    // Get the current branch
    let branch = git.current_branch()?;

    // Sync the current branch (and its parents)
    sync(branch.as_str())?;
    Ok(())
}

fn sync(branch: &str) -> Result<(), Box<dyn Error>> {
    let git = Git::new();

    // Is 'branch' local, or remote?
    let branch_is_remote = git.remotes()?.into_iter().any(|remote| branch.starts_with(&remote));

    // If 'branch' is remote, return - we can't sync remote branches
    if branch_is_remote { return Ok(()); }

    // Okay, we're local - let's switch to 'branch' and update!
    git.switch(branch)?;
    let parent = git.parent_of(branch)?;

    // Update parent if any
    let res: Result<(), Box<dyn Error>> = match parent {
        None => Ok(()),
        Some(onto) => {
            // Recurse: sync our parent
            sync(onto.as_str())?;

            // Update ourselves (pull from parent then rebase on parent's changes)
            git.pull(Vec::new())?;
            git.rebase(branch, &onto)?;
            Ok(())
        },
    };

    // If we had an error, show the user that we failed to update this branch.
    // TODO: Display the error and exit
    let signal = match res {
        Ok(_) => CHECK.green(),
        Err(_) => CROSS.red()
    };

    println!("Updated {} {}", branch, signal);
    Ok(())
}

fn rebase(branch: &str) -> Result<(), Box<dyn Error>> {
    let git = Git::new();
    let onto = git.parent_of(branch)?;
    let res: Result<(), Box<dyn Error>> = match onto {
        Some(onto) => { git.rebase(branch, &onto)?; Ok(()) },
        None => Ok(()),
    };
    let signal = match res {
        Ok(_) => CHECK.green(),
        Err(e) => CROSS.red()
    };
    println!("Updated {} {}", branch, signal);
    Ok(())
}