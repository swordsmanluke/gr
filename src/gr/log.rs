mod color_cycle;
mod tree;
mod log_tree;

use anyhow::Result;
use colored::{Colorize};
use itertools::Itertools;
use crate::gr::log::log_tree::{GitBranch, LogBranch};
use crate::gr::log::tree::Tree;
use crate::indent::Indentable;


pub fn log() -> Result<()> {
    let t: Tree<LogBranch> = Tree::new(GitBranch::root().into());
    let log = t.to_string();
    println!("{}", log);
    Ok(())
}

