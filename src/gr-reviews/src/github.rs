use std::error::Error;
use std::sync::Arc;
use crate::{Review, ReviewService};
use octocrab;

pub(crate) struct GithubReviewer {
    client: Arc<octocrab::Octocrab>,
    owner: String,
    repo: String,
    page: u32
}

impl GithubReviewer {
    pub fn new(gh_owner: &str, gh_repo: &str) -> GithubReviewer {
        GithubReviewer { client: octocrab::instance(),
            owner: gh_owner.to_string(),
            repo: gh_repo.to_string()
            , page: 1
        }
    }

}

impl ReviewService for GithubReviewer {
    async fn reviews(&self) -> Result<Vec<Review>, Box<dyn Error>> {
        let data = self.client
            .pulls(&self.owner, &self.repo)
            .list()
            .page(self.page)
            .per_page(10)
            .send().await?;

        let out = data.items
            .iter()
            .map(|pull| Review {
                id: pull.number.to_string(),
                title: pull.title.clone().unwrap_or(String::new()).to_owned(),
                body: pull.body.clone().unwrap_or(String::new()).to_owned(),
                service: "github".to_string(),
                reviewers: Vec::new(),
                labels: Vec::new(),
                state: crate::review::ReviewState::Pending,
                tests: Vec::new()
            })
            .collect();

        Ok(out)
    }

    async fn review(&self, id: &str) -> Result<Option<Review>, Box<dyn Error>> {
        todo!()
    }
}