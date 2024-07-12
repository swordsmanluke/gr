use itertools::Itertools;
use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use colored::Colorize;
use crate::{Review, ReviewService, ReviewState};
use octocrab;
use octocrab::models::{IssueState};
use octocrab::models::checks::CheckRun;
use octocrab::models::pulls::PullRequest;
use octocrab::Octocrab;
use octocrab::params::repos::Commitish;

pub struct GithubReviewer {
    client: Arc<octocrab::Octocrab>,
    owner: String,
    repo: String,
    page: u32
}

struct PullRequestWithChecks {
    pull: PullRequest,
    checks: Vec<CheckRun>
}

impl GithubReviewer {
    pub fn new(gh_owner: &str, gh_repo: &str) -> GithubReviewer {
        GithubReviewer { client: octocrab::instance(),
            owner: gh_owner.to_string(),
            repo: gh_repo.to_string()
            , page: 1
        }
    }

    async fn await_mergability(&self, pull: &PullRequest) -> Result<PullRequest, Box<dyn Error>> {
        // Check whether the review can be merged
        let mut pull = pull.clone();
        for _ in 0..10 {
            if pull.mergeable.is_some() { break; }
            pull = self.client.pulls(&self.owner, &self.repo)
                .get(pull.number)
                .await.unwrap();
        }

        if pull.mergeable.is_some() { return Ok(pull); }

        Err(format!("Could not determine mergeability for PR: {}", pull.number).into())
    }

    async fn with_check_runs(&self, pull: &PullRequest) -> Result<PullRequestWithChecks, Box<dyn Error>> {

        let checks = self.client.checks(&self.owner, &self.repo)
            .list_check_runs_for_git_ref(Commitish::from(pull.head.sha.clone())).send().await.unwrap();

        Ok(PullRequestWithChecks { pull: pull.clone(), checks: checks.check_runs })
    }

    async fn convert_to_review(&self, pull: PullRequest) -> Result<Review, Box<dyn Error>> {
        let pull = self.await_mergability(&pull).await?;
        let prc = self.with_check_runs(&pull).await?;

        Ok(Review::from(prc))
    }
}

impl From<PullRequestWithChecks> for Review {
    fn from(prc: PullRequestWithChecks) -> Self {
        let review_state = state_of_review(&prc);
        Review {
            // Number monotonically increases per repo - we want this as our ID
            id: prc.pull.number.to_string(),
            // Branch names
            branch: prc.pull.head.label.unwrap().to_owned(),
            base: prc.pull.base.label.unwrap().to_owned(),

            // Title and body
            title: prc.pull.title.clone().unwrap_or(String::new()).to_owned(),
            body: prc.pull.body.clone().unwrap_or(String::new()).to_owned(),
            // Where is this review?
            service: "github".to_string(),
            url: prc.pull.html_url.clone(),

            // What's its state?
            reviewers: Vec::new(),
            state: review_state,
            tests: Vec::new()
        }
    }

}

fn state_of_review(prc: &PullRequestWithChecks) -> ReviewState {
    let review = prc.pull.clone();

    if review.draft.unwrap() { return ReviewState::Pending; } // not ready yet!
    if review.merged_at.is_some() { return ReviewState::Merged; }  // This is already merged!
    if review.state == Some(IssueState::Closed) { return ReviewState::Closed; } // this was closed!


    // Conflicts need to be resolved
    if review.mergeable.unwrap() == false { return ReviewState::Conflicted; }

    // See if the PR's checks have passed
    // let check_states = *prc.checks
    //     .iter()
    //     .map(|check| { check.conclusion.unwrap_or(String::new()).to_owned().to_lowercase() })
    //     .filter(|c| *c != "")
    //     .collect::<Vec<String>>()
    //     .iter()
    //     .unique()
    //     .collect::<Vec<&String>>();

    ReviewState::Pending
}

#[async_trait]
impl ReviewService for GithubReviewer {
    async fn reviews(&self) -> Result<Vec<Review>, Box<dyn Error>> {
        let data = self.client
            .pulls(&self.owner, &self.repo)
            .list()
            .send().await?
            .items.iter()
            .map(|pr| pr.to_owned())
            .collect::<Vec<PullRequest>>();

        let mut reviews = Vec::new();
        for pr in data {
            reviews.push(self.convert_to_review(pr.clone()).await?);
        }

        Ok(reviews)
    }

    async fn reviews_for(&self, branch: &str) -> Result<Vec<Review>, Box<dyn Error>> {
        Ok(self.reviews().await?
            .into_iter()
            .filter(|review| review.branch == branch)
            .map(|review| review.clone())
            .collect())
    }

    async fn review(&self, id: &str) -> Result<Option<Review>, Box<dyn Error>> {
        let pr = self.client
            .pulls(&self.owner, &self.repo)
            .get(id.parse::<u64>()?)
            .await?;
        Ok(Some(self.convert_to_review(pr).await?))
    }

    async fn create_review(&self, branch: &str, parent: &str, title: &str, body: &str) -> Result<Review, Box<dyn Error>> {
        let handler = self.client.pulls(&self.owner, &self.repo);
        let pull = handler
            .create(title, branch, parent)
            .body(String::from(body))
            .send().await?;

        let review = self.convert_to_review(pull).await?;

        Ok(review)
    }
}