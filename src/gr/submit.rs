
use gr_reviews::review_service_for;
use gr_reviews::ReviewService;
use gr_reviews::Review;
use std::error::Error;

pub async fn reviews(cr_tool: &str) -> Result<Vec<Review>, Box<dyn Error>> {
    let service = review_service_for(cr_tool)?.unwrap();
    service.reviews().await
}

pub async fn submit(id: &str, cr_tool: &str) -> Result<Option<Review>, Box<dyn Error>> {
    let service = review_service_for(cr_tool)?.unwrap();
    service.review(id).await
}
