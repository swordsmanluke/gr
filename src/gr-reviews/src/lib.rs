mod review;
mod github;
mod none;

use std::iter::Rev;
use gr_git::Git;
pub use review::*;
use crate::github::GithubReviewer;
use crate::none::NoneReviewer;

pub fn review_services() -> Vec<String> {
    vec!["none".to_string(),
         "github".to_string()]
}

pub fn review_service_for<T>(name: &str) -> Result<Option<T>, String>
where T: ReviewService
{
    if name == "none" {
        return Ok(Some(NoneReviewer::new()));
    }

    if name == "github" {
        return Ok(get_github_reviewer());
    }

    return Err(format!("Unknown code review service: {}", name));
}

fn get_github_reviewer() -> Option<GithubReviewer> {
    let git = Git::new();

    // use the remote url to determine if this is a github repo
    let remotes = git.remote(vec!["-v"]).unwrap().lines().map(|s| s.to_owned()).collect::<Vec<String>>();
    let gh_remotes = remotes.into_iter().filter(|r| r.contains("github.com")).collect::<Vec<String>>();

    if gh_remotes.len() == 0 {
        println!("No github remote found. Remotes: \n{}", git.remote(vec!["-v"]).unwrap());
        return None;
    }

    let remote = gh_remotes.first().unwrap();

    let owner = remote.split("/").collect::<Vec<&str>>()[0];
    let repo = remote.split("/").collect::<Vec<&str>>()[1];

    Some(GithubReviewer::new(owner, repo))
}