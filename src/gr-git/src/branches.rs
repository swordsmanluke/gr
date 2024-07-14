use std::collections::HashMap;
use anyhow::Result;
use regex::{Match, Regex};
use crate::Git;

// TODO: Standardize on Vec<String> for args

pub enum BranchType {
    Local,
    Remote,
    All
}

impl Git {
    /***** Commands *****/

    pub fn checkout(&self, args: Vec<&str>) -> Result<()> {
        self.assert_in_repo()?;
        self.git("checkout", args)?;
        Ok(())
    }

    pub fn switch(&self, branch: &str) -> Result<()> {
        self.assert_in_repo()?;
        self.git("switch", vec![branch])?;
        Ok(())
    }

    /**** Information ***/
    pub fn current_branch(&self) -> Result<String> {
        self.assert_in_repo()?;
        self.git("rev-parse", vec!["--abbrev-ref", "HEAD"])
    }

    pub fn branches(&self) -> Result<Vec<String>> {
        self.assert_in_repo()?;
        let output = self.git("for-each-ref", vec!["--format=%(refname:short)", "refs/heads/"])?
            .lines()
            .map(|s| s.to_string())
            .collect();

        Ok(output)
    }

    pub fn branch(&self, args: Vec<&str>) -> Result<String> {
        self.assert_in_repo()?;
        self.git("branch", args)
    }

    pub fn parents(&self) -> Result<HashMap<String, String>> {
        // Formatted output of `git branch -vv` looks like this:
        //   main      483f881 Add: Branch movement commands
        // * movements 483f881 [main] Add: Branch movement commands
        //
        // Our regex here extracts the parent branch name from the output.
        // First group is the name of the branch, second group is the parent branch name
        self.assert_in_repo()?;
        let parent_regex = Regex::new(r"\s*(\S+\s+[a-f0-9]+)(\s+\[(.*)\])?\s+(.*)")?;

        let output = self.branch(vec!["-vv"])?
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

    pub fn parent_of(&self, branch: &str, branch_type: BranchType) -> Result<Option<String>> {
        self.assert_in_repo()?;
        let maybe_parent = self.parents()?.get(branch).cloned();
        let parent_is_remote = maybe_parent.is_some() && !self.branches()?.contains(maybe_parent.as_ref().unwrap());

        match maybe_parent {
            None => Ok(None),
            Some(parent) => {
                match branch_type {
                    BranchType::All => Ok(Some(parent)),
                    BranchType::Local => if parent_is_remote { Ok(None) } else { Ok(Some(parent)) },
                    BranchType::Remote => if parent_is_remote { Ok(Some(parent)) } else { Ok(None) }
                }
            }
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
    pub fn children_of(&self, branch: &str) -> Result<Vec<String>> {
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