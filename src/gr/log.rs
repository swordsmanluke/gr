use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use anyhow::Result;
use colored::{Color, Colorize};
use itertools::Itertools;
use gr_git::Git;

struct Log {
    stack: Stack,
    git: Git,
    color_cycle: Vec<Color>
}


pub fn log() -> Result<()> {
    let mut log = Log::new();
    log.call(&[]);
    Ok(())
}

impl Log {
    pub fn call(&self, _args: &[&str]) -> Result<()> {
        println!("{}", self.as_string()?);
        Ok(())
    }

    pub fn new() -> Self {
        let git = Git::new();
        let color_cycle: Vec<Color> = vec![Color::Cyan,
                                           Color::Magenta,
                                           Color::BrightGreen,
                                           Color::BrightRed,
                                           Color::Blue,
                                           Color::Yellow,
                                           Color::BrightYellow];
        let stack = Stack {
            roots: Vec::new(),
            branches: HashMap::new(),
        };
        Self {
            stack,
            git,
            color_cycle
        }
    }

    fn as_string(&self) -> Result<String> {
        let mut roots = Vec::new();
        for root in &self.stack.roots {
            roots.push(self.root_as_string(&root.clone())?);
        }
        Ok(roots.into_iter().join("\n----------------\n"))
    }

    fn track_stack_presence(&self, all_branches: &mut HashMap<String, Branch>, stack: (&Stack, usize)) {
        let (stack, stack_id) = stack;
        for (branch_name, branch) in &stack.branches {
            let entry = all_branches.entry(branch_name.clone()).or_insert_with(|| branch.clone());
            entry.stacks.push(stack_id);
            entry.stacks.sort();
            entry.stacks.dedup();
        }
    }

    fn root_as_string(&self, root: &Branch) -> Result<String> {
        let branches = self.set_color_and_depth(root);
        Ok(branches.iter().map(|lb| lb.name.to_string()).collect::<Vec<_>>().join("\n"))
    }

    fn formatted_diff(&self, cur_branch: Option<&Branch>, parent_branch: Option<&Branch>, max_len: usize) -> Result<String> {
        if let (Some(cur_branch), Some(parent_branch)) = (cur_branch, parent_branch) {
            let commits = self.git.commit_diff(&parent_branch.name, &cur_branch.name)?
                .split("\n")
                .map(|s| Commit::from_line(s))
                .collect::<Vec<_>>();
            let commits: Vec<_> = commits.into_iter().map(|(commit)| format!("{} {}", commit.sha[0..7].bright_black(), commit.message)).collect();
            if commits.len() > max_len {
                let remaining = commits.len() - max_len;
                Ok(commits[0..max_len].to_vec().join("\n").to_string() + &format!("\n    ... ({}) more", remaining))
            } else {
                Ok(commits.join("\n"))
            }
        } else {
            Ok(String::new())
        }
    }

    fn set_color_and_depth(&self, root: &Branch) -> Vec<LogBranch> {
        let root_branch = &self.stack.branches[&root.name];
        let mut branches = HashMap::new();
        self.dfs_branch(root_branch, None, &mut branches);
        let max_depth = branches.values().map(|b| b.depth).min().unwrap_or(0).abs();

        branches.values_mut().for_each(|b| b.depth += max_depth);
        branches.into_iter().map(|(_, b)| b).collect()
    }

    fn dfs_branch(&self, branch: &Branch, parent: Option<&LogBranch>, branches: &mut HashMap<String, LogBranch>) -> Result<()> {
        let color = self.branch_color(parent);
        let depth = self.depth(branch, parent);
        let commits = self.git.commit_diff(&branch.name, &parent.unwrap().name)?.split("\n").map(|s| Commit::from_line(s)).collect();
        let children = branch.children.clone();

        let log_branch = LogBranch {
            name: branch.name.clone(),
            sha: branch.sha.clone(),
            color,
            depth,
            commits,
            children,
        };

        branches.insert(branch.name.clone(), log_branch.clone());

        for child in &branch.children {
            self.dfs_branch(&self.stack.branches[child].clone(), Some(&log_branch), branches);
        }

        Ok(())
    }

    fn branch_color(&self, parent: Option<&LogBranch>) -> Color {
        if let Some(parent) = parent {
            if parent.split() {
                self.color_cycle[parent.children.len() % self.color_cycle.len()]
            } else {
                parent.color.clone()
            }
        } else {
            Color::Cyan
        }
    }

    fn depth(&self, branch: &Branch, parent: Option<&LogBranch>) -> isize {
        if let Some(parent) = parent {
            if parent.split() {
                parent.depth - parent.children.iter().position(|child| child == &branch.name).unwrap_or(0) as isize
            } else {
                parent.depth
            }
        } else {
            0
        }
    }
}

#[derive(Clone)]
struct Commit {
    sha: String,
    message: String,
}

impl Commit {
    pub fn from_line(line: &str) -> Self {
        let mut parts = line.split(" ");
        let sha = parts.next().unwrap().to_string();
        let message = parts.collect::<Vec<_>>().join(" ");
        Self { sha, message }
    }
}

#[derive(Clone)]
struct Branch {
    name: String,
    sha: String,
    children: Vec<String>,
    stacks: Vec<usize>,
}

impl Branch {
    fn new(name: String, sha: String) -> Self {
        Self {
            name,
            sha,
            children: Vec::new(),
            stacks: Vec::new(),
        }
    }
}

#[derive(Clone)]
struct LogBranch {
    name: String,
    sha: String,
    color: Color,
    depth: isize,
    commits: Vec<Commit>,
    children: Vec<String>,
}

impl LogBranch {
    fn new(name: String, sha: String, color: Color, depth: isize, commits: Vec<Commit>, children: Vec<String>) -> Self {
        Self {
            name,
            sha,
            color,
            depth,
            commits,
            children,
        }
    }

    fn leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn pipe(&self) -> bool {
        self.children.len() == 1
    }

    fn split(&self) -> bool {
        self.children.len() > 1
    }

    fn to_string(&self) -> Result<String> {
        let git = Git::new();
        let selected = self.name == git.current_branch()?;
        let dot = if selected { "●" } else { "○" };
        let pretty_branch = format!("{} {}", dot, self.name).color(self.color);
        let pretty_sha = format!("{}", self.sha[0..7].to_string()).green();

        let branch_line = format!("{} ({})", pretty_branch, pretty_sha);
        let diff = self.formatted_diff().split('\n').map(|line| format!("|  {}", line)).collect::<Vec<_>>().join("\n");
        let line = vec![branch_line, diff].into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join("\n");

        let line = if self.split() {
            tree(&line, self.depth, "|-")
        } else {
            tree(&line, self.depth, "| ")
        };

        Ok(line)
    }

    fn formatted_diff(&self) -> String {
        let commits: Vec<_> = self.commits.iter().map(|commit| format!("{} {}", commit.sha[0..7].bright_black(), commit.message)).collect();
        if commits.len() > 5 {
            commits[0..5].to_vec().join("\n") + &format!("\n    ... ({} more)", commits.len() - 5)
        } else {
            commits.join("\n")
        }
    }
}

struct Stack {
    roots: Vec<Branch>,
    branches: HashMap<String, Branch>,
}

fn tree(line: &str, depth: isize, pipe: &str) -> String {
    format!("{}{}{}", "  ".repeat(depth as usize), pipe, line)
}
