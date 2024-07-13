use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crate::{MergeRequest, MergeState, Review, ReviewService};

pub struct NoneReviewer {}

impl NoneReviewer {
    pub fn new() -> NoneReviewer {
        NoneReviewer {}
    }

}

#[async_trait]
impl ReviewService for NoneReviewer {
    async fn merge(&self, _id: &str) -> Result<MergeRequest> {
        Err(anyhow!("Can't merge with None reviewer"))
    }

    async fn review(&self, _id: &str) -> Result<Option<Review>> {
        Ok(None)
    }

    async fn reviews(&self) -> Result<Vec<Review>> {
        Ok(vec![])
    }

    async fn reviews_for(&self, _branch: &str) -> Result<Vec<Review>> {
        Ok(vec![])
    }

    async fn create_review(&self, _branch: &str, _parent: &str, _title: &str, _body: &str) -> Result<Review> {
        Err(anyhow!("Can't create review with None reviewer"))
    }
}