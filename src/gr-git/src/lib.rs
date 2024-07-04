use std::collections::HashMap;
use std::error::Error;
use std::process::Command;
use regex::{Match, Regex};

pub struct Git;

impl Git {
    pub fn new() -> Git {
        Git {}
    }

    pub fn in_repo(&self) -> Result<bool, Box<dyn Error>> {
        // Check if the current directory is a git repository
        let output = self.git("rev-parse", "--is-inside-work-tree")?;
        Ok(output == "true")
    }

    pub fn current_branch(&self) -> Result<String, Box<dyn Error>> {
        self.assert_in_repo()?;
        self.git("rev-parse", "--abbrev-ref HEAD")
    }

    pub fn branches(&self) -> Result<Vec<String>, Box<dyn Error>> {
        self.assert_in_repo()?;
        let output = self.git("for-each-ref", "--format=%(refname:short) refs/heads/")?
            .lines()
            .map(|s| s.to_string())
            .collect();

        Ok(output)
    }

    pub fn branch(&self, args: &str) -> Result<String, Box<dyn Error>> {
        self.assert_in_repo()?;
        self.git("branch", args)
    }

    pub fn parents(&self) -> Result<HashMap<String, String>, Box<dyn Error>> {
        self.assert_in_repo()?;
        let parent_regex = Regex::new(r"\s*(\S+\s+[a-f0-9]+)(\s+\[(.*)\])?\s+(.*)")?;

        let output = self.branch("-vv")?
            .lines()
            .map(|s| parent_regex.captures(s))
            .filter(|cap| cap.is_some())
            .map(|cap| cap.unwrap())
            .map (|cap| (
                cap.get(1)
                    .unwrap().as_str()
                    .split(" ")
                    .into_iter()
                    .filter(|s| *s != "*")
                    .next()
                    .unwrap().to_string(),
                cap.get(3)))
            .map (|(name, parent)| (name, self.extract_parent(parent)))
            .collect::<HashMap<String, String>>();

        Ok(output)
    }

    pub fn parent_of(&self, branch: &str) -> Result<Option<String>, Box<dyn Error>> {
        self.assert_in_repo()?;
        Ok(self.parents()?.get(branch).cloned())
    }

    fn extract_parent(&self, parent: Option<Match>) -> String {
        match parent {
            Some(parent) => {
                parent.as_str().split(":").next().unwrap().to_string()
            },
            None => "".to_string(),
        }
    }

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

    pub fn checkout(&self, args: &str) -> Result<(), Box<dyn Error>> {
        self.assert_in_repo()?;
        self.git("checkout", args)?;
        Ok(())
    }

    pub fn switch(&self, branch: &str) -> Result<(), Box<dyn Error>> {
        self.assert_in_repo()?;
        self.git("checkout", branch)?;
        Ok(())
    }

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