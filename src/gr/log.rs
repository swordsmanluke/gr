mod color_cycle;
mod tree;
mod log_tree;

use anyhow::Result;
use colored::{Colorize};
use itertools::Itertools;
use crate::gr::log::log_tree::{GitBranch, LogBranch};
use crate::gr::log::tree::Tree;
use crate::indent::Indentable;

pub(crate) const USAGE: &str = "gq log

Displays the commit log stack.

The log is displayed as a stack of branches, with each branch's unique commits listed
between them in stack order. The top of the stack is the latest changes, which will
be merged _downward_ to the root branch.";

pub fn log() -> Result<()> {
    let t: Tree<LogBranch> = Tree::new(GitBranch::root().into());
    let log = t.to_string();
    println!("{}", log);
    Ok(())
}

