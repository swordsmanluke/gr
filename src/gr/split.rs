use std::fmt::{Display, Formatter};
use anyhow::Result;
use candy::candy::{Candy, CandyOption};
use candy::events::CandyEvent;
use candy::events::CandyEvent::{Select, Submit};
use colored::Color::{Blue, BrightCyan, BrightGreen, BrightWhite, BrightYellow, Cyan, Green, Magenta, Red, White, Yellow};
use colored::{Color, Colorize};
use itertools::Itertools;
use gr_git::Git;


pub fn split() -> Result<()> {
    // Select commits on the current branch to split into new branches
    // e.g.
    // 1 - <sha123> commit 5
    // 1 - <sha123> commit 4
    // 2 - <sha123> commit 3
    // 2 - <sha123> commit 2
    // 3 - <sha123> commit 1
    // Toggling a commit in the selector will select all commits which depend on it _and_ which
    // are not already selected. Selected commits will be grouped into new branches.
    // e.g. in the above example, commits (5, 4), (3, 2) and (1) will be grouped into new branches.
    let git = Git::new();
    let candy = Candy::new();

    let cur_branch = git.current_branch()?;
    let parent_branch = git.parent_of(&cur_branch, gr_git::BranchType::Local)?;
    let commits = match parent_branch.as_ref() {
        Some(parent_branch) => git.commit_diff(&cur_branch, &parent_branch).unwrap().lines().map(|s| s.to_string()).collect(),
        None => git.log(vec!["--format=oneline"]).unwrap(),
    }.into_iter()
        .map(|s| FormattedCommit::from_line(&s))
        .collect::<Vec<FormattedCommit>>();

    match candy.choose_option("Select commit to split on", commits.clone(), None, true) {
        Select(selections) => {
            // Collect commits per branch
            let mut branches: Vec<Vec<FormattedCommit>> = vec![];
            let mut branch: Vec<FormattedCommit> = vec![];
            let last_sha = commits.last().as_ref().unwrap().sha.clone();
            for commit in commits {
                let is_last = commit.sha == last_sha;

                // the "first" commit is the last one we want to apply,
                // so reverse the commit order by inserting at 0 here
                branch.insert(0, commit.clone());
                if is_last || selections.contains(&commit.sha) {
                    // Done with the current branch!
                    branches.insert(0, branch); // reverse "branches" order so code closest to the root is applied first
                    branch = vec![]; // start a new branch
                }
            }

            // Create new branches - branch names will be <og-branch>-<number>
            // and should branch off the og-branch's parent.
            let mut parent = git.parent_of(&cur_branch, gr_git::BranchType::Local)?.unwrap();

            for (i, branch) in branches.iter().enumerate() {
                // Basing off of "parent", create a new branch and cherry pick the necessary commits over
                let branch_name = format!("{}-{}", cur_branch, i + 1);
                git.checkout(vec!["-t", &parent, "-b", &branch_name])?;
                for commit in branch {
                    git.cherry_pick(vec![&commit.sha])?;
                }
                // We are the parent now - make sure the next branch tracks us.
                parent = branch_name;
            }
            // Next, rebase the original branch's children onto our final branch (now the "parent")
            let children = git.children_of(&cur_branch)?;
            for child in &children {
                git.switch(child)?;
                git.branch(vec![&format!("--set-upstream-to={}", &parent)])?;
            }

            // Now, delete the original branch
            git.branch(vec!["-D", &cur_branch])?;

            // recursively rebase our children onto their new parent
            git.recursive_rebase(&parent, vec![])?;

            // Finally Switch back to the "parent" - the top branch,
            // containing the changes of all the new branches
            git.switch(&parent)?;
        }
        CandyEvent::Cancel => {
            println!("Cancelled");
        }
        _ => {}  // We won't get other events from select_one
    }

    Ok(())
}

#[derive(Clone)]
struct FormattedCommit {
    sha: String,
    title: String,
}

impl FormattedCommit {
    const COLOR_CYCLE: [Color; 10] = [White, BrightCyan, BrightGreen, BrightYellow, Blue, Green, Magenta, Red, Yellow, Cyan];

    pub fn from_line(line: &str) -> FormattedCommit {
        let pattern = regex::Regex::new(r"(\S+)\s+(\(.*\)\s+)?(.*)").unwrap();
        let cap = pattern.captures(line).unwrap();  // can this realistically fail?

        FormattedCommit {
            sha: cap.get(1).unwrap().as_str().to_string(),
            title: cap.get(3).unwrap().as_str().to_string(),
        }
    }
}

impl Display for FormattedCommit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{} {}", self.sha[0..6].to_string(), self.title))
    }
}

impl CandyOption for FormattedCommit {
    fn id(&self) -> String {
        self.sha.clone()
    }

    fn render(&self, index: usize, cursor_at: usize, selections: &Vec<bool>) -> String {
        let mut selections = selections.clone();
        let last_idx = selections.len() - 1;
        selections[last_idx] = true; // last commit is always considered selected for Splitting

        let colors = selections.iter()
            .filter(|s| **s)
            .count();

        // only apply colors if there is at least one selection
        let mut branch_id = if colors > 0 { 1 } else { 0 };

        let branch_indices = selections
            .iter().map(|s| {
            let b_id = branch_id;
            if *s { branch_id += 1 };
            b_id
        }).collect_vec();

        let color_idx = branch_indices[index] % Self::COLOR_CYCLE.len();
        let candy_color = Self::COLOR_CYCLE[color_idx];

        let mut out = format!("{} {}", self.sha[0..6].to_string(), self.title).color(candy_color);

        if selections[index] || index == selections.len() - 1 {
            out = format!("{} -- Branch {}", out, branch_indices[index]).color(candy_color);
        }
        if cursor_at == index { out = out.bold(); }

        out.to_string()
    }

    fn filter(&self, query: &str) -> bool {
        query.chars().all(|c| self.title.contains(c))
    }
}