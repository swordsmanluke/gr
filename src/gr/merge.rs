use std::thread::sleep;
use std::time::Duration;
use gr_reviews::{CodeReviewService, MergeRequest, MergeState, review_service_for};
use gr_reviews::ReviewService;
use anyhow::Result;
use colored::Colorize;
use gr_git::Git;
use gr_git::BranchType;
use candy::symbols::{BACKSPACE, CHECK, CROSS};
use crate::indent::Indentable;

struct Pair<A, B> {
    a: A,
    b: B
}

/// Merges approved / mergeable code reviews for the current stack of branches
pub async fn merge(cr_tool: &CodeReviewService, remote: &str) -> Result<()> {
    let git = Git::new();
    let cr_service = review_service_for(cr_tool)?;

    println!("{}", "Merging stack".green());
    // recursively, from the lowest branch, merge our reviews
    let mut merge_requests = merge_branch(&cr_service, remote, &git.current_branch()?).await?;

    while !merge_requests.is_empty() {
        let mut pair = merge_requests.remove(0);

        print!("  {}: ?", pair.a.green());
        match pair.b {
            None => { println!("  {}", "Up to date".yellow()); continue; },
            Some(mut mr) => { track_mr_progress(&mut mr).await?; }
        }
    }

    Ok(())
}

async fn track_mr_progress(mr: &mut MergeRequest) -> Result<()> {
    let spinner_seq = vec!["-", "\\", "|", "/"];
    let mut spinner = spinner_seq.iter().cycle();

    loop {
        print!("{}", BACKSPACE);
        match mr.state {
            MergeState::Pending => {
                print!("{}", spinner.next().unwrap().yellow());
                mr.refresh().await?;
                sleep(Duration::from_millis(250))
            },
            MergeState::Merged => {
                println!("{}", CHECK.green());
                break;
            },
            MergeState::Failed => {
                println!("{}", CROSS.red());
                break;
            },
        }
    }
    Ok(())
}

async fn  merge_branch(cr_service: &Box<dyn ReviewService>, remote: &str, branch: &str) -> Result<Vec<Pair<String, Option<MergeRequest>>>> {
    let git = Git::new();
    let parent = git.parent_of(&branch, BranchType::Local)?;

    // Recurse down the stack to the root branch before we continue
    let mut merge_requests = match parent.clone()
    {
        Some(p) => Box::pin(merge_branch(cr_service, remote, &p)).await?,
        None => Vec::new(),
    };

    // Check to see if there's an open review for this branch - if so, try to merge it!
    let existing_reviews = cr_service.reviews_for(branch).await?;
    let merge_req = match existing_reviews.first() {
        Some(r) => Some(r.merge().await?),
        None => { None },
    };

    merge_requests.push(Pair {
        a: branch.to_string(),
        b: merge_req
    });

    Ok(merge_requests)
}

