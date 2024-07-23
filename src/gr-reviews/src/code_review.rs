use anyhow::{anyhow, Result};
use std::fmt::{Display, Formatter};
use async_trait::async_trait;
use url::Url;
use crate::{CodeReviewService, MergeRequest, review_service_for};

/// Represents the state of a code review
#[derive(Clone, Default)]
pub enum ReviewState {
    Conflicted,
    #[default]
    Pending,
    Approved,
    Rejected,
    Merged,
    Closed
}

impl Display for ReviewState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReviewState::Conflicted => write!(f, "Conflicted"),
            ReviewState::Pending => write!(f, "Pending"),
            ReviewState::Approved => write!(f, "Approved"),
            ReviewState::Rejected => write!(f, "Rejected"),
            ReviewState::Merged => write!(f, "Merged"),
            ReviewState::Closed => write!(f, "Closed"),
        }
    }
}

/// Represents the state of a test which may (or may not) block the review's approval
#[derive(Clone)]
pub enum ReviewTestState {
    Pending,
    Passed,
    Failed,
}

/// Represents a test which may (or may not) block the review's approval
/// e.g. the number of received approvals being lower than the number required
#[derive(Clone)]
pub struct ReviewTest {
    pub name: String,
    pub state: ReviewTestState
}

/// Represents a code review
#[derive(Clone)]
pub struct Review {
    pub id: String,
    pub branch: String,
    pub base: String,
    pub title: String,
    pub body: String,
    pub service: CodeReviewService,
    pub reviewers: Vec<String>,
    pub state: ReviewState,
    pub tests: Vec<ReviewTest>,
    pub url: Option<Url>,
}

impl Review {
    pub async fn refresh(&mut self) -> Result<()> {
        let service = review_service_for(&self.service)?;
        *self = service.review(&self.id).await?.unwrap();
        Ok(())
    }

    pub async fn merge(&self) -> Result<MergeRequest> {
        review_service_for(&self.service)?.merge(&self).await
    }
}

#[async_trait]
pub trait ReviewService {
    async fn merge(&self, review: &Review) -> Result<MergeRequest>;
    async fn review(&self, id: &str) -> Result<Option<Review>>;
    async fn reviews(&self) -> Result<Vec<Review>>;
    async fn reviews_for(&self, branch: &str) -> Result<Vec<Review>>;
    async fn create_review(&self, branch: &str, parent: &str, title: &str, body: &str) -> Result<Review>;
}