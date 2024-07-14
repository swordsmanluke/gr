use anyhow::Result;
use colored::Colorize;
use gr_git::{BranchType, ExecGit, Git};
use gr_tui::symbols::{CHECK, CROSS};
use gr_tui::TuiWidget;

enum SyncStatus {
    Success,
    NoDiff,
    ConflictWith(String),
}

struct Pair<A, B> {
    a: A,
    b: B,
}

type SyncResult = Pair<String, SyncStatus>; // SyncResult

impl SyncResult {
    pub fn new(branch: &str, status: SyncStatus) -> Self { Self { a: branch.to_string(), b: status } }
    pub fn branch(&self) -> &str { &self.a }
    pub fn status(&self) -> &SyncStatus { &self.b }
}

pub fn restack(tui: &mut TuiWidget) -> Result<()> {
    let git = Git::new();

    // Get the current branch
    let branch = git.current_branch()?;

    // Sync the current branch (and its parents)
    let results = sync(branch.as_str())?;
    for res in &results { println!("{}: {}", res.branch().green(), sync_result_to_status_char(res.status())); }
    println!();
    // Try to fix any conflicts first
    for res in &results {
        match res.status() {
            SyncStatus::ConflictWith(b) => { ask_to_fix_conflicts(tui, b, res.branch())?; }
            _ => continue
        }
    }

    // Okay, if conflicts are all resolved, then delete merged branches
    for res in &results {
        match res.status() {
            SyncStatus::NoDiff => ask_to_delete(res.branch(), tui)?,
            _ => continue
        }
    }

    Ok(())
}

fn ask_to_fix_conflicts(tui: &mut TuiWidget, parent: &str, branch: &str) -> Result<()> {
    let git = Git::new();
    let exec_git = ExecGit::new();

    println!("Could not rebase branch {} <- {}", parent.green(), branch.red());
    println!("Please fix the conflicts, then attempt to sync again.");
    git.switch(branch)?;
    exec_git.pull(vec!["--rebase".to_string()])?;

    Ok(())
}

fn ask_to_delete(branch: &str, tui: &mut TuiWidget) -> Result<()> {
    let git = Git::new();
    let parent = match git.parent_of(branch, BranchType::Local)?{
        Some(p) => p,
        None => return Ok(())  // If we don't have a local parent, we can't delete the branch.
    };

    // Ask the user if they want to delete the branch
    if tui.yn(&format!("Delete branch {}?", branch.yellow()))? {
        // Get our children and rebase them onto our parent
        let children = git.children_of(branch)?;
        for child in &children {
            git.switch(child)?;
            git.branch(vec![&format!("--set-upstream-to={}", &parent)])?;
        }
        // Move to an _only_ child OR our parent
        if children.len() == 1 { git.switch(&children.get(0).unwrap())?; }
        else { git.switch(&parent)?; }
        // Delete the branch
        git.branch(vec!["-d", branch])?;
    }
    Ok(())
}

fn sync_result_to_status_char(status: &SyncStatus) -> String {
    match status {
        SyncStatus::Success => CHECK.green(),
        SyncStatus::NoDiff => CHECK.green(),
        SyncStatus::ConflictWith(_) => CROSS.red(),
    }.to_string()
}

fn sync(branch: &str) -> Result<Vec<SyncResult>> {
    let git = Git::new();

    // Is 'branch' local, or remote?
    let branch_is_remote = git.remotes()?.into_iter().any(|remote| branch.starts_with(&remote));

    // If 'branch' is remote, return - we don't need to sync remote branches!
    if branch_is_remote { return Ok(vec![SyncResult::new(branch, SyncStatus::Success)]); }

    // Next step requires recursing to our parent - if one is present
    let parent = git.parent_of(branch, BranchType::Local)?;

    // Update parent if any
    let mut results = match &parent {
        None => vec![],  // No parent - we're at the bottom and ready to begin!
        Some(onto) => sync(onto)?  // Recurse to sync our parent!
    };

    // Okay, we're ready to sync - checkout 'branch' and update it!
    git.switch(branch)?;
    let pull_res = git.pull(vec!["--rebase"]);
    let rebase_res = git.rebase(vec![]);

    if pull_res.is_err() || rebase_res.is_err() {
        // reset git to 'good' state
        git.rebase(vec!["--abort"])?;
        results.push(SyncResult::new(branch, SyncStatus::ConflictWith(parent.unwrap().to_string())));
    } else {
        // Success! Check to see if there's any diff post-merge
        let is_different = match &parent {
            None => { false }
            Some(p) => {
                git.commit_diff(branch, p)?
                    .split("\n")
                    .filter(|s| !s.is_empty())
                    .count() > 0
            }
        };

        if is_different {
            results.push(SyncResult::new(branch, SyncStatus::Success));
        } else {
            // Success - but no diff, so we can probably delete this branch.
            results.push(SyncResult::new(branch, SyncStatus::NoDiff));
        }
    }


    Ok(results)
}

