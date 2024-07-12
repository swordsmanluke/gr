use std::error::Error;
use std::fmt::{Display, Formatter};
use async_trait::async_trait;
use url::Url;

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
#[derive(Clone, Default)]
pub struct Review {
    pub id: String,
    pub branch: String,
    pub base: String,
    pub title: String,
    pub body: String,
    pub service: String,
    pub reviewers: Vec<String>,
    pub state: ReviewState,
    pub tests: Vec<ReviewTest>,
    pub url: Option<Url>,
}

/// Functions which must be implemented by all code review objects
/// e.g. this + Review struct
pub trait TReview {
    fn refresh(&mut self) -> Result<(), Box<dyn Error>>;
    fn merge(&mut self) -> Result<MergeRequest, Box<dyn Error>>;
    /*
    // Github Review Merge
    self.client
            .repos(&self.owner, &self.repo)
            .merge(branch, parent)
            .commit_message("This is a custom merge-commit message")
            .send()
     */
}

/// Whether or not a given code review has been merged - but from the Merge Request's perspective
pub enum MergeState {
    Pending,
    Merged
}

/// Tracks the state of a request to merge a Review
pub struct MergeRequest
{
    pub state: MergeState,
    review: Box<dyn TReview>,
}

/// Traits required for all merge requests
pub trait TMergeRequest {
    async fn refresh(&mut self) -> Result<(), Box<dyn Error>>;
    fn state(&self) -> MergeState;
}

#[async_trait]
pub trait ReviewService {
    async fn reviews(&self) -> Result<Vec<Review>, Box<dyn Error>>;
    async fn reviews_for(&self, branch: &str) -> Result<Vec<Review>, Box<dyn Error>>;
    async fn review(&self, id: &str) -> Result<Option<Review>, Box<dyn Error>>;
    async fn create_review(&self, branch: &str, parent: &str, title: &str, body: &str) -> Result<Review, Box<dyn Error>>;
}