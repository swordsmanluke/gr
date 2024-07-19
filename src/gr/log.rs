mod color_cycle;

use std::collections::HashMap;
use anyhow::Result;
use colored::{Color, Colorize};
use itertools::Itertools;
use gr_git::{BranchType, Git};
use crate::gr::log::color_cycle::ColorCycle;
use crate::indent::Indentable;

struct Log {
    git: Git,
    stack_id: isize,
    color_cycle: ColorCycle,
    log_branches: Vec<LogBranch>
}

struct LogCommit {
    sha: String,
    message: String,
    index: usize
}

struct LogBranch {
    name: String,
    parent: Option<String>,
    stack_id: isize,
    depth: usize,
    color: Color,
    commits: Vec<LogCommit>,
    indent_level: usize
}

impl LogBranch {
    fn new(name: String, parent: Option<String>, depth: usize, color: Color) -> Self {
        Self {
            name,
            parent,
            color,
            depth,
            stack_id: 1,
            commits: Vec::new(),
            indent_level: 0
        }
    }

    pub fn to_string(&self) -> String {
        self.commits.iter().map(|c| c.to_string().indent(1)).collect::<Vec<_>>().join("\n")
    }
}

impl LogCommit {
    pub fn to_string(&self) -> String {
        format!("{} {}", self.sha.bright_black(), self.message)
    }
}

impl Log {
    pub fn new() -> Self {
        let git = Git::new();
        let color_cycle = ColorCycle::new();
        Self {
            git,
            color_cycle,
            stack_id: 0,
            log_branches: Vec::new()  // empty for now
        }
    }

    pub fn call(&mut self) -> Result<String> {
        // step one, collect commit diffs for each branch
        self.collect_log_branches()?;
        // step two, assign stack ids to each branch
        self.assign_stack_ids()?;
        // step three, sort by stack id, then depth
        self.sort_log_branches();
        // step four, convert to strings!
        let mut output = String::new();
        for branch in self.log_branches.iter() {
            output += &branch.to_string();
        }

        Ok(String::new())
    }

    fn assign_stack_ids(&mut self) -> Result<()> {
        // For the current branch and ancestors, stack = 0
        let mut branch = self.git.current_branch()?;
        self.set_stack_id(&branch, 0)?;

        while let Some(parent) = self.git.parent_of(&branch, BranchType::All)? {
            self.set_stack_id(&parent, 0)?;
            branch = parent;
        }

        // Now, starting from the root, traverse the tree and assign stack id by decrementing it
        // any time there's more than one child.
        // e.g. if parent.stack = 0, child_1.stack = 0, child_2.stack = -1

        let roots = self.log_branches.iter().filter(|b| b.parent.is_none()).map(|b| b.name.clone()).collect_vec();
        for root in roots {
            self.update_stack_ids(&root, 0)?;
        }

        Ok(())
    }

    fn next_stack_id(&mut self) -> isize {
        self.stack_id -= 1;  // stack ids go down
        self.stack_id
    }

    fn update_stack_ids(&mut self, branch: &str, stack_id: isize) -> Result<()> {
        let mut log_branch = self.log_branches.iter_mut().find(|l| l.name == branch).unwrap();
        if log_branch.stack_id > 0 { log_branch.stack_id = stack_id; }
        let children = self.git.children_of(branch)?;
        match children.len() {
            0 => {},
            1 => self.update_stack_ids(&children[0], stack_id)?,
            _ => {
                for child in children {
                    let stack_id = self.next_stack_id();
                    self.update_stack_ids(&child, stack_id)?;
                }
            }
        }
        Ok(())
    }

    fn set_stack_id(&mut self, branch: &str, stack_id: isize) -> Result<()> {
        if let Some(log_branch) = self.log_branches.iter_mut().find(|l| l.name == branch){
            log_branch.stack_id=stack_id;
        }
        Ok(())
    }

    fn branch_depth(&self, branch: &str) -> Result<usize> {
        let mut depth = 0;
        let mut current = branch.to_string();
        // Count how long it takes until we no longer have a parent - then we're at the root.
        // And remote branches don't count for our needs.
        while let Some(parent) = self.git.parent_of(&current, BranchType::Local)? {
            depth += 1;
            current = parent.to_string();
        }
        Ok(depth)
    }

    fn collect_log_branches(&mut self) -> Result<()> {
        let parents = self.git.parents()?;

        for branch in &self.git.branches()? {
            let mut log_branch = LogBranch::new(
                branch.clone(),
                parents.get(branch).cloned(),
                self.branch_depth(&branch)?,
                self.color_cycle.color());


            match parents.get(branch) {
                None => {},
                Some(parent) => {
                    // Capture the commit diffs
                    let diff = self.git.commit_diff(&branch, &parent)?;
                    let mut commit_idx: usize = 0;

                    for commit in diff.split("\n") {
                        let mut parts = commit.split(" ").collect::<Vec<_>>();
                        let sha = parts[0].to_string();
                        let message = parts[1..].join(" ");
                        let commit = LogCommit { sha, message, index: commit_idx };
                        log_branch.commits.push(commit);
                        commit_idx += 1;
                    }

                    self.log_branches.push(log_branch);
                }
            }
        };
        Ok(())
    }

    fn sort_log_branches(&mut self) {
        self.log_branches.sort_by(|a, b| {
            a.stack_id.cmp(&b.stack_id)
                .then_with(|| a.depth.cmp(&b.depth))
        });
    }
}


pub fn log() -> Result<()> {
    let mut log = Log::new();
    log.call();
    Ok(())
}

