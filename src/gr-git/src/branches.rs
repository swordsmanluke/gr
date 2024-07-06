use std::collections::HashMap;
use std::error::Error;
use regex::{Match, Regex};
use crate::Git;

// TODO: Standardize on Vec<String> for args

impl Git {
    /***** Commands *****/

    pub fn checkout(&self, args: &str) -> Result<(), Box<dyn Error>> {
        self.assert_in_repo()?;
        self.git("checkout", args)?;
        Ok(())
    }

    pub fn switch(&self, branch: &str) -> Result<(), Box<dyn Error>> {
        self.assert_in_repo()?;
        self.git("switch", branch)?;
        Ok(())
    }

    /**** Information ***/
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
        /// Formatted output of `git branch -vv` looks like this:
        ///   main      483f881 Add: Branch movement commands
        /// * movements 483f881 [main] Add: Branch movement commands
        ///
        /// Our regex here extracts the parent branch name from the output.
        /// First group is the name of the branch, second group is the parent branch name
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
        let parent = self.parents()?.get(branch).cloned();
        match parent {
            Some(parent) => {
                if parent.is_empty() { Ok(None) }
                else { Ok(Some(parent)) }
            },
            None => Ok(None),
        }
    }

    fn extract_parent(&self, parent: Option<Match>) -> String {
        match parent {
            Some(parent) => {
                parent.as_str().split(":").next().unwrap().to_string()
            },
            None => "".to_string(),
        }
    }

    /// Returns all direct children of the given branch
    pub fn children_of(&self, branch: &str) -> Result<Vec<String>, Box<dyn Error>> {
        self.assert_in_repo()?;
        // invert `parents` and take all children that belong to `branch` directly
        let output = self.parents()?
            .into_iter()
            .filter(|(_name, parent)| parent == branch)
            .map(|(name, _parent)| name)
            .filter (|name| !name.is_empty())
            .collect::<Vec<String>>();

        Ok(output)
    }
}