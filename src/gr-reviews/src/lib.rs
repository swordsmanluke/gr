mod review;
mod none;
mod github;

use gr_git::Git;
pub use review::*;
use crate::github::GithubReviewer;
use crate::none::NoneReviewer;

pub fn review_service_for(name: &str) -> Result<Option<Box<dyn ReviewService>>, String>
{
    if name == "None" {
        return Ok(Some(Box::new(NoneReviewer::new())));
    }

    if name == "Github" {
        return Ok(get_github_reviewer());
    }

    return Err(format!("Unknown code review service: {}", name));
}

fn get_github_reviewer() -> Option<Box<dyn ReviewService>> {
    let git = Git::new();

    // use the remote url to determine if this is a github repo
    let remotes = git.remote(vec!["-v"]).unwrap().lines().map(|s| s.to_owned()).collect::<Vec<String>>();
    let gh_remotes = remotes.into_iter().filter(|r| r.contains("github.com")).collect::<Vec<String>>();

    if gh_remotes.len() == 0 {
        println!("No github remote found. Remotes: \n{}", git.remote(vec!["-v"]).unwrap());
        return None;
    }

    // Drop everything before "github.com/" to get the owner and repo names
    let remote = gh_remotes.first().unwrap().split("github.com").collect::<Vec<&str>>()[1];

    let owner = remote.split("/").collect::<Vec<&str>>()[0].trim().replace(":", "").replace("/", "");
    let repo = remote.split("/").collect::<Vec<&str>>()[1].split(" ").collect::<Vec<&str>>()[0].trim()
        .replace("/", "")
        .replace(".git", "");

    Some(Box::new(GithubReviewer::new(&owner, &repo)))
}