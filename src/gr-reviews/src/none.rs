use std::hash::{DefaultHasher, Hash, Hasher};
use anyhow::{Result};
use async_trait::async_trait;
use gr_git::Git;
use crate::{CodeReviewService, MergeRequest, MergeState, Review, ReviewService, ReviewState};

pub struct NoneReviewer {}

impl NoneReviewer {
    pub fn new() -> NoneReviewer {
        NoneReviewer {}
    }

}

#[async_trait]
impl ReviewService for NoneReviewer {
    async fn merge(&self, review: &Review) -> Result<MergeRequest> {
        // Checkout the branch's parent, merge our branch onto it then move/rebase our children
        let git = Git::new();
        let parent = review.base.clone();
        let children = git.children_of(&review.branch)?;

        git.switch(&parent)?;
        git.merge(vec![&review.branch])?;

        // Reparent our children
        for child in children {
            git.switch(&child)?;
            git.branch(vec!["--set-upstream-to", &parent])?;
        }

        // sync our parent and everyone downstream of it
        git.sync(&parent)?;

        // and finally, delete this branch - as it's been merged
        git.branch(vec!["-d", &review.branch])?;

        Ok(MergeRequest {
            state: MergeState::Merged,
            review: review.clone(),
        })
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

    async fn create_review(&self, branch: &str, parent: &str, title: &str, body: &str) -> Result<Review> {
        let mut hasher = DefaultHasher::new();
        (branch.to_string() + parent).hash(&mut hasher);
        let id = format!("{:x}", hasher.finish());
        Ok(Review {
            id,
            branch: branch.to_string(),
            base: parent.to_string(),
            title: title.to_string(),
            body: body.to_string(),
            service: CodeReviewService::None,
            reviewers: vec![],
            state: ReviewState::Approved,  // No one has to approve a "None" review
            tests: vec![],
            url: None,
        })
    }
}