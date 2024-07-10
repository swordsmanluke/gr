
use gr_reviews::{review_service_for, TReview};
use gr_reviews::review_services;
use gr_reviews::CodeReviewService;
use gr_reviews::ReviewService;
use gr_reviews::Review;
use std::error::Error;

pub async fn reviews(cr_tool: &str) -> Result<Vec<Review>, Box<dyn Error>> {
    let service = review_service_for(cr_tool)?.unwrap();

    get_reviews(service).await
}

async fn get_reviews<T>(tool: T) -> Result<Vec<Review>, Box<dyn Error>>
where T: ReviewService
{
    tool.reviews().await
}