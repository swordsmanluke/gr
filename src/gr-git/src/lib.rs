mod exec_git;
mod branches;
pub use exec_git::ExecGit;  // export for consumers of this crate

use std::error::Error;
use std::process::Command;

pub struct Git;

impl Git {
    pub fn new() -> Git {
        Git {}
    }

    /***** Information *****/

    pub fn in_repo(&self) -> Result<bool, Box<dyn Error>> {
        // Check if the current directory is a git repository
        let output = self.git("rev-parse", "--is-inside-work-tree")?;
        Ok(output == "true")
    }

    pub fn status(&self) -> Result<String, Box<dyn Error>> {
        self.assert_in_repo()?;
        self.git("status", "")
    }

    /***** Remotes *****/

    pub fn remotes(&self) -> Result<Vec<String>, Box<dyn Error>> {
        self.assert_in_repo()?;
        let output = self.remote("")?
            .lines()
            .map(|s| s.to_string())
            .collect();
        Ok(output)
    }

    pub fn remote(&self, args: &str) -> Result<String, Box<dyn Error>> {
        self.assert_in_repo()?;
        self.git("remote", args)
    }

    /***** Utilities *****/

    fn git(&self, command: &str, args: &str) -> Result<String, Box<dyn Error>> {
        // Execute the git executable with the given command and arguments
        // and return the output
        let split_args = args.split(" ").filter(|s| !s.is_empty()).collect::<Vec<&str>>();
        let mut g = Command::new("git");
        let mut cmd = g.arg(command);

        if split_args.len() != 0 {
            cmd = cmd.args(split_args);
        }

        // Execute the command and get the output
        let output = cmd.output()?;
        let out = String::from_utf8_lossy(&output.stdout).to_string();
        let err = String::from_utf8_lossy(&output.stderr).to_string();
        let text = out + "\n" + &err;
        let text = text.lines().into_iter().map(|l| l.trim()).filter(|l| !l.is_empty()).collect::<Vec<&str>>().join("\n");
        Ok(text)
    }

    fn assert_in_repo(&self) -> Result<(), Box<dyn Error>> {
        if !self.in_repo()? {
            let cur_path = std::env::current_dir()?;
            let msg = format!("Not in a git repository: {}", cur_path.display());
            return Err(msg.into());
        }
        Ok(())
    }
}