use crate::{Review, ReviewState};
use anyhow::Result;

/// Whether or not a given code review has been merged - but from the Merge Request's perspective
#[derive(Clone)]
pub enum MergeState {
    Pending,
    Merged,
    Failed
}

/// Tracks the state of a request to merge a Review
#[derive(Clone)]
pub struct MergeRequest {
    pub state: MergeState,
    pub review: Review,
}

impl MergeRequest {
    pub fn new (review: Review) -> Self {
        let state = Self::map_state(&review.state);
        Self {
            state,
            review,
        }
    }

    pub fn in_state(mut self, state: MergeState) -> Self {
        self.state = state;
        self
    }

    pub async fn refresh(&mut self) -> Result<()> {
        // Pull the latest state of the review and use that to determine our own state
        self.review.refresh().await?;
        self.update_state();
        Ok(())
    }

    fn update_state(&mut self) -> () {
        self.state = Self::map_state(&self.review.state);
    }

    fn map_state(review_state: &ReviewState) -> MergeState {
        match review_state {
            // Unclear if we can merge this PR or not - just leave it pending for now.
            ReviewState::Pending => MergeState::Pending,

            // Ready to merge - waiting for the review service to merge the review!
            ReviewState::Approved => MergeState::Pending,

            // Successfully merged the PR!
            ReviewState::Merged => MergeState::Merged,

            // Failed to merge the PR - or the PR is in a non-mergeable state.
            ReviewState::Conflicted |
            ReviewState::Closed |
            ReviewState::Rejected => MergeState::Failed,
        }
    }
}
