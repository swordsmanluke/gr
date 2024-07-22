use std::fmt::Display;
use colored::Colorize;
use itertools::Itertools;
use gr_git::{BranchType, Git};
use crate::gr::log::tree::{Node, WithChildren};
use crate::indent::Indentable;

pub trait NamedData {
    fn name(&self) -> String;
    fn data(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct LogBranch {
    sha: String,
    name: String,
    commits: Vec<LogCommit>
}

#[derive(Debug, Clone)]
pub struct LogCommit {
    sha: String,
    title: String,
    index: usize
}

pub struct GitBranch {
    name: String,
    sha: String
}

impl GitBranch {
    pub fn new(name: String, sha: String) -> Self {
        Self { name, sha }
    }

    pub fn root() -> Self {
        let git = Git::new();
        let name = git.root_branches().unwrap().first().unwrap().to_string();
        let sha = git.rev_parse(vec![&name]).unwrap();
        Self::new(name, sha)
    }
}

impl From<GitBranch> for LogBranch {
    fn from(branch: GitBranch) -> Self {
        let git = Git::new();
        let parent = git.parent_of(&branch.name, BranchType::All).unwrap_or(None);

        let commits = match parent {
            None => { Vec::new() }
            Some(p) => { LogCommit::from_diff(git.commit_diff(&branch.name, &p).unwrap_or(String::new())) }
        };

        LogBranch {
            name: branch.name,
            sha: branch.sha,
            commits
        }
    }
}

impl WithChildren for LogBranch {
    fn children(&self) -> Vec<Self> {
        let git = Git::new();
        git.children_of(&self.name).unwrap().iter()
            .map(|c| GitBranch::new(c.to_string(), git.rev_parse(vec![&c]).unwrap()).into())
            .collect_vec()
    }
}

impl LogBranch {
    pub fn from_branch(branch: &str) -> Self {
        let git = Git::new();
        let name = branch.to_string();
        let sha = git.rev_parse(vec![branch]).unwrap();

        let parent = git.parent_of(branch, BranchType::All).unwrap_or(None);
        let commits = match parent {
            None => { Vec::new() }
            Some(p) => { LogCommit::from_diff(git.commit_diff(branch, &p).unwrap_or(String::new())) }
        };

        Self {
            name,
            sha,
            commits
        }
    }
}

impl Display for LogCommit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Display for LogBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name() + "\n" + &self.data().indent(2))
    }
}

impl NamedData for LogBranch {
    fn name(&self) -> String {
        // Header of <branch name> - <commit title>
        let title = match self.commits.iter().last() {
            None => String::new(),
            Some(commit) => commit.title.clone()
        };
        format!("{} - {}", self.name, title)
    }

    fn data(&self) -> String {
        let cstrs = self.commits.iter().map(|cstr|cstr.to_string()).collect_vec();

        // commit message block
        cstrs.join("\n")
    }
}

impl LogCommit {
    pub fn from_diff(diff: String) -> Vec<Self> {
        let mut commit_idx: usize = 0;
        let mut commits = Vec::new();

        for commit in diff.split("\n") {
            let mut parts = commit.split(" ").collect::<Vec<_>>();
            let sha = parts[0].to_string();
            let message = parts[1..].join(" ");
            let commit = LogCommit { sha, title: message, index: commit_idx };
            commits.push(commit);
            commit_idx += 1;
        }

        commits
    }

    pub fn to_string(&self) -> String {
        match self.sha.len() {
            0..=7 => format!("{} {}", self.sha.bright_black(), self.title),
            _ => format!("{} {}", self.sha[0..7].bright_black(), self.title)
        }
    }
}
