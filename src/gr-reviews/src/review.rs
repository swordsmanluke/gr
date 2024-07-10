use crate::none::NoneReviewer;
use crate::github::GithubReviewer;

pub enum CodeReviewService {
    None(NoneReviewer),
    Github(GithubReviewer)
}


pub enum ReviewState {
    Pending,
    Approved,
    Rejected,
}

pub enum ReviewTestState {
    Pending,
    Passed,
    Failed,
}

pub struct ReviewTest {
    pub name: String,
    pub state: ReviewTestState
}

pub struct Review {
    pub id: String,
    pub title: String,
    pub body: String,
    pub service: String,
    pub reviewers: Vec<String>,
    pub labels: Vec<String>,
    pub state: ReviewState,
    pub tests: Vec<ReviewTest>
}

pub trait TReview {
    fn refresh(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn merge(&mut self) -> Result<MergeRequest, Box<dyn std::error::Error>>;
}

pub enum MergeState {
    Pending,
    Merged
}

pub struct MergeRequest
{
    pub state: MergeState,
    review: Box<dyn TReview>,
}

pub trait TMergeRequest {
    async fn refresh(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn state(&self) -> MergeState;
}

pub trait ReviewService {
    async fn reviews(&self) -> Result<Vec<Review>, Box<dyn std::error::Error>>;
    async fn review(&self, id: &str) -> Result<Option<Review>, Box<dyn std::error::Error>>;
}