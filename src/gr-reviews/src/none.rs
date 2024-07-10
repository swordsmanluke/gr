use std::error::Error;
use std::sync::Arc;
use crate::{Review, ReviewService};

pub struct NoneReviewer {}

impl NoneReviewer {
    pub fn new() -> NoneReviewer {
        NoneReviewer {}
    }

}

impl ReviewService for NoneReviewer {
    async fn reviews(&self) -> Result<Vec<Review>, Box<dyn Error>> {
        Ok(vec![])
    }

    async fn review(&self, id: &str) -> Result<Option<Review>, Box<dyn Error>> {
        Ok(None)
    }
}