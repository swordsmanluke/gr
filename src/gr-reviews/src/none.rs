use std::error::Error;
use std::sync::Arc;
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
    async fn reviews(&self) -> Result<Vec<Review>, Box<dyn Error>> {
        Ok(vec![])
    }

    async fn reviews_for(&self, branch: &str) -> Result<Vec<Review>, Box<dyn Error>> {
        todo!()
    }

    async fn review(&self, id: &str) -> Result<Option<Review>, Box<dyn Error>> {
        Ok(None)
    }

    async fn create_review(&self, branch: &str, parent: &str, title: &str, body: &str) -> Result<Review, Box<dyn Error>> {
        Ok((Review::default()))
    }
}