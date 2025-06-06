
use gr_reviews::{CodeReviewService, review_service_for};
use gr_reviews::ReviewService;
use gr_reviews::Review;
use anyhow::{anyhow, Result};
use colored::Colorize;
use gr_git::Git;
use gr_git::BranchType;
use itertools::Itertools;
use candy::symbols::SMALL_SQUARE;
use regex::Regex;
use candy::candy::Candy;
use candy::events::CandyEvent::Submit;
use crate::indent::Indentable;

/// Retrieves the list of reviews for the current repo
pub async fn reviews(cr_tool: &CodeReviewService) -> Result<Vec<Review>> {
    let service = review_service_for(cr_tool)?;
    service.reviews().await
}

/// Creates / Updates code reviews for the current stack of branches
/// e.g. the current branch and all of its ancestors down to the root
pub async fn submit(cr_tool: &CodeReviewService, remote: &str) -> Result<()> {
    let git = Git::new();
    let cr_service = review_service_for(cr_tool)?;

    println!("{}", "Submitting stack".green());
    // recursively, from the lowest branch, submit our branches
    let reviews = submit_branch(&cr_service, remote, &git.current_branch()?).await?;

    for rv in reviews {
        println!("  {}: {}", rv.id.cyan(), rv.url.unwrap().to_string().green());
    }

   Ok(())
}

async fn submit_branch(cr_service: &Box<dyn ReviewService>, remote: &str, branch: &str) -> Result<Vec<Review>> {
    let git = Git::new();
    let parent = git.parent_of(&branch, BranchType::Local)?;

    // Recurse down the stack to the root branch before we continue
    let mut reviews = match parent.clone()
    {
        Some(p) => Box::pin(submit_branch(cr_service, remote, &p)).await?,
        None => Vec::new(),
    };
    // Return unless we have a diff vs our parent to submit
    if !needs_submitting(branch)? {
        return Ok(reviews);
    }

    //  Ok, we need to submit this branch, so display it to the User for context
    println!("  {} {}", SMALL_SQUARE.green(), branch.cyan());

    // TODO: Spinner while we push the branch to the remote and create the PR

    // 1. push to the remote, creating a remote branch!
    push_branch(remote, branch)?;

    // 2a. Check to see if there's a PR for this branch
    let existing_reviews = cr_service.reviews_for(branch).await?;
    if let Some(r) = existing_reviews.first() {
        // Existing PR will have been updated when we pushed to the branch - so just return the reference.
        reviews.push(r.clone());
        return Ok(reviews);
    }

    // 2b.Nope - Create a new review
    let parent = parent.unwrap_or_else(|| String::new());

    // TODO: Use the system editor to get the PR body.
    // For now, collect the git commit messages

    let mut commit_messages = get_commit_message(branch, &parent)?;

    let title = match commit_messages.get(0) {
        Some(t) => t.to_string(),
        None => "".to_string(),
    };
    let candy = Candy::new();
    let Submit(title) = candy.edit_line("Title", Some(&title)) else { Err(anyhow!("Failed to edit title"))? };
    let body = commit_messages.join("\n");

    println!("{}\n\n{}", title.green(), body.indent(2).green());
    if candy.yn("Create Review?") {
        let rv = cr_service.create_review(&branch, &parent, &title, &body).await?;
        reviews.push(rv);
    }

    // And return!
    Ok(reviews)
}

pub(crate) fn get_commit_message(branch: &str, parent: &str) -> Result<Vec<String>> {
    let git = Git::new();
    let commit_messages = git.log(vec![&format!("{}..{}", parent, branch), "--format=%B", "--reverse"])?;

    Ok(commit_messages)
}

fn push_branch(remote: &str, branch: &str) -> Result<()> {
    let git = Git::new();

    // if we fail to push normal-like, let's try a force push before we give up
    match git.push(vec![remote, branch]) {
        Ok(_) => (),
        Err(_e) => { git.push(vec![remote, branch, "-f"])?; }
    }
    Ok(())
}

fn needs_submitting(branch: &str) -> Result<bool> {
    let git = Git::new();
    let parent = git.parent_of(&branch, BranchType::All)?;
    match parent {
        None => Ok(false), // No parent, so nothing to submit
        Some(p) => Ok(git.commit_diff(branch, &p)?.split("\n").all(|l| !l.is_empty())),  // If we have no diff, we don't need to submit
    }
}