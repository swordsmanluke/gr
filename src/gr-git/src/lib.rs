mod exec_git;
mod branches;
pub use exec_git::ExecGit;  // export for consumers of this crate

use anyhow::{anyhow, Result};
use std::process::Command;
use colored::Colorize;
pub use branches::BranchType;

pub struct Git;

impl Git {
    pub fn new() -> Git {
        Git {}
    }

    /***** Transformative *****/
    pub fn pull(&self, args: Vec<&str>) -> Result<String> {
        self.assert_in_repo()?;
        self.git("pull", args)
    }

    pub fn push(&self, args: Vec<&str>) -> Result<String> {
        self.assert_in_repo()?;
        self.git("push", args)
    }

    pub fn rebase(&self, branch: &str, base: &str) -> Result<String> {
        self.assert_in_repo()?;

        if branch == base {
            return Err(anyhow!("Cannot rebase onto self"));
        }

        if !self.branches()?.contains(&String::from(branch)) {
            return Err(anyhow!("Branch {} does not exist", branch));
        }

        if base.contains("/") &&
            self.remotes()?.iter().any(|remote| base.starts_with(remote)) {
                self.git("rebase", vec![base])
            }
        else {
            self.git("rebase", vec![base, branch])
        }
    }

    /***** Information *****/

    pub fn in_repo(&self) -> Result<bool> {
        // Check if the current directory is a git repository
        let output = self.git("rev-parse", vec!["--is-inside-work-tree"])?;
        Ok(output == "true")
    }

    pub fn log(&self, args: Vec<&str>) -> Result<Vec<String>> {
        self.assert_in_repo()?;
        Ok(self.git("log", args)?.lines().map(|s| s.trim().to_string()).collect())
    }

    pub fn revlist(&self, args: Vec<&str>) -> Result<String> {
        self.assert_in_repo()?;
        self.git("rev-list", args)
    }

    pub fn status(&self) -> Result<String> {
        self.assert_in_repo()?;
        self.git("status", vec![""])
    }

    pub fn commit_diff(&self, branch: &str, parent: &str) -> Result<String> {
        self.assert_in_repo()?;
        let dotdot= format!("{}..{}", parent, branch);
        self.git("log", vec![&dotdot, "--format=oneline"])
    }

    /***** Remotes *****/

    pub fn remotes(&self) -> Result<Vec<String>> {
        self.assert_in_repo()?;
        let output = self.remote(Vec::new())?
            .lines()
            .map(|s| s.to_string())
            .collect();
        Ok(output)
    }

    pub fn remote(&self, args: Vec<&str>) -> Result<String> {
        self.assert_in_repo()?;
        self.git("remote", args)
    }

    /***** Utilities *****/

    fn git(&self, command: &str, args: Vec<&str>) -> Result<String> {
        // Execute the git executable with the given command and arguments
        // and return the output
        let mut g = Command::new("git");
        let mut cmd = g.arg(command);

        if args.len() != 0 { cmd = cmd.args(args.clone()); }

        // Execute the command and get the output
        let output = cmd.output()?;

        // Check the exit code
        if !output.status.success() {
            let msg = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(anyhow!("{}\n{}", format!("> git {} {}", command, args.join(" ")).red(), msg.yellow()));
        }

        let out = String::from_utf8_lossy(&output.stdout).to_string();
        let err = String::from_utf8_lossy(&output.stderr).to_string();
        let text = out + "\n" + &err;
        let text = text.lines().into_iter().map(|l| l.trim()).filter(|l| !l.is_empty()).collect::<Vec<&str>>().join("\n");
        Ok(text)
    }

    fn assert_in_repo(&self) -> Result<()> {
        if !self.in_repo()? {
            let cur_path = std::env::current_dir()?;
            let msg = format!("Not in a git repository: {}", cur_path.display());
            return Err(anyhow!(msg));
        }
        Ok(())
    }
}