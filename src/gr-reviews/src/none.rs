use anyhow::Result;
use async_trait::async_trait;
use crate::{Review, ReviewService};

pub struct NoneReviewer {}

impl NoneReviewer {
    pub fn new() -> NoneReviewer {
        NoneReviewer {}
    }

}

#[async_trait]
impl ReviewService for NoneReviewer {
    async fn reviews(&self) -> Result<Vec<Review>> {
        Ok(vec![])
    }

    async fn reviews_for(&self, _branch: &str) -> Result<Vec<Review>> {
        todo!()
    }

    async fn review(&self, _id: &str) -> Result<Option<Review>> {
        Ok(None)
    }

    async fn create_review(&self, _branch: &str, _parent: &str, _title: &str, _body: &str) -> Result<Review> {
        Ok(Review::default())
    }
}