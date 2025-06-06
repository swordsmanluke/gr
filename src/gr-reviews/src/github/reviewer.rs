use anyhow::{anyhow, Result};
use async_trait::async_trait;
use colored::Colorize;
use crate::{CodeReviewService, MergeRequest, Review, ReviewService, ReviewState};
use octocrab;
use octocrab::models::{IssueState};
use octocrab::models::checks::CheckRun;
use octocrab::models::pulls::PullRequest;
use octocrab::Octocrab;
use octocrab::params::repos::Commitish;

pub struct GithubReviewer {
    client: Octocrab,
    owner: String,
    repo: String
}

struct PullRequestWithChecks {
    pull: PullRequest,
    checks: Vec<CheckRun>
}

const BACKOFF_TIME_SECONDS : [u64; 10] = [1, 5, 5, 5, 5, 10, 10, 10, 30, 60];

impl GithubReviewer {
    pub fn new(gh_owner: &str, gh_repo: &str) -> GithubReviewer {
        let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
        let client = Octocrab::builder().personal_token(token).build().unwrap();

        GithubReviewer { client,
            owner: gh_owner.to_string(),
            repo: gh_repo.to_string()
        }
    }

    async fn await_mergability(&self, pull: &PullRequest) -> Result<PullRequest> {
        // Check whether the review can be merged
        let mut pull = pull.clone();
        let mut backoff_time_idx: usize = 0;

        // It can take minutes for the mergeability to be determined
        for _ in 0..15 {
            // Don't hang on merged PRs - they were clearly "mergeable"
            if pull.merged_at.is_some() { pull.mergeable = Some(true); }

            // Don't hang on closed PRs - they are clearly "unmergeable"
            if pull.closed_at.is_some() { pull.mergeable = Some(false); }

            if pull.mergeable.is_some() { break; }

            let backoff_time = BACKOFF_TIME_SECONDS[backoff_time_idx];
            backoff_time_idx = (backoff_time_idx + 1) % BACKOFF_TIME_SECONDS.len();
            tokio::time::sleep(std::time::Duration::from_secs(backoff_time)).await;

            pull = self.client.pulls(&self.owner, &self.repo)
                .get(pull.number)
                .await.unwrap();
        }

        if pull.mergeable.is_some() { return Ok(pull); }

        Err(anyhow!("Could not determine mergeability for PR: {}", pull.number).into())
    }

    async fn with_check_runs(&self, pull: &PullRequest) -> Result<PullRequestWithChecks> {
        let checks = self.client.checks(&self.owner, &self.repo)
            .list_check_runs_for_git_ref(Commitish::from(pull.head.sha.clone())).send().await;

        let check_runs = match checks {
            Ok(c) => c.check_runs,
            Err(_) => Vec::new()
        };


        Ok(PullRequestWithChecks { pull: pull.clone(), checks: check_runs })
    }

    async fn convert_to_review(&self, pull: PullRequest) -> Result<Review> {
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
            branch: prc.pull.head.label.unwrap().split(":").last().unwrap().to_owned(),
            base: prc.pull.base.label.unwrap().split(":").last().unwrap().to_owned(),

            // Title and body
            title: prc.pull.title.clone().unwrap_or(String::new()).to_owned(),
            body: prc.pull.body.clone().unwrap_or(String::new()).to_owned(),
            // Where is this review?
            service: CodeReviewService::Github,
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
    async fn merge(&self, review: &Review) -> Result<MergeRequest> {
        let pull = self.client
            .pulls(&self.owner, &self.repo)
            .get(review.id.parse::<u64>()?)
            .await?;

        let review = self.convert_to_review(pull).await?;
        let msg = format!("{}\n\n{}", review.title, review.body);

        self.client
            .repos(&self.owner, &self.repo)
            .merge(&review.branch, &review.base)
            .commit_message(&msg)
            .send().await?;
        Ok(MergeRequest::new(review.clone()))
    }

    async fn review(&self, id: &str) -> Result<Option<Review>> {
        let pr = self.client
            .pulls(&self.owner, &self.repo)
            .get(id.parse::<u64>()?)
            .await?;
        Ok(Some(self.convert_to_review(pr).await?))
    }

    async fn reviews(&self) -> Result<Vec<Review>> {
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

    async fn reviews_for(&self, branch: &str) -> Result<Vec<Review>> {
        Ok(self.reviews().await?
            .into_iter()
            .filter(|review| review.branch == branch)
            .map(|review| review.clone())
            .collect())
    }

    async fn create_review(&self, branch: &str, parent: &str, title: &str, body: &str) -> Result<Review> {
        let handler = self.client.pulls(&self.owner, &self.repo);
        println!("Creating review for {} [on {}] at {}/{}", branch.cyan(), parent.black(), self.owner.green(), self.repo.blue());
        println!("PR title: {}", title.bold());
        let pull = handler
            .create(title, branch, parent)
            .body(String::from(body))
            .send()
            .await?;

        let review = self.convert_to_review(pull).await?;

        Ok(review)
    }
}
