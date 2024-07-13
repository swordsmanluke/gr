mod code_review;
mod none;
mod github;
mod merge_requests;

use std::fmt::{Display, Formatter};
use gr_git::Git;
pub use code_review::*;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use crate::github::GithubReviewer;
use crate::none::NoneReviewer;
pub use crate::merge_requests::{MergeRequest, MergeState};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CodeReviewService {
    Github,
    None
}

impl Display for CodeReviewService {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeReviewService::None => write!(f, "None"),
            CodeReviewService::Github => write!(f, "Github"),
        }
    }
}

pub fn review_service_for(service: &CodeReviewService) -> Result<Box<dyn ReviewService>>
{
    match service {
        CodeReviewService::Github => Ok(get_github_reviewer()?),
        CodeReviewService::None => Ok(Box::new(NoneReviewer::new()))
    }
}

fn get_github_reviewer() -> Result<Box<dyn ReviewService>> {
    let git = Git::new();

    // use the remote url to determine if this is a github repo
    let remotes = git.remote(vec!["-v"]).unwrap().lines().map(|s| s.to_owned()).collect::<Vec<String>>();
    let gh_remotes = remotes.into_iter().filter(|r| r.contains("github.com")).collect::<Vec<String>>();

    if gh_remotes.len() == 0 {
        return Err(anyhow!("No github remote found. Remotes: \n{}", git.remote(vec!["-v"]).unwrap()));
    }

    // Drop everything before "github.com/" to get the owner and repo names
    let remote = gh_remotes.first().unwrap().split("github.com").collect::<Vec<&str>>()[1];

    let owner = remote.split("/").collect::<Vec<&str>>()[0].trim().replace(":", "").replace("/", "");
    let repo = remote.split("/").collect::<Vec<&str>>()[1].split(" ").collect::<Vec<&str>>()[0].trim()
        .replace("/", "")
        .replace(".git", "");

    Ok(Box::new(GithubReviewer::new(&owner, &repo)))
}