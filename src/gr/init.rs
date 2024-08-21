use anyhow::{anyhow, Result};
use colored::Colorize;
use gr_git::{BranchType, Git};
use gr_reviews::CodeReviewService;
use candy::candy::Candy;
use candy::events::CandyEvent::{Cancel, Submit};
use crate::config::{config_dir_path, config_file_exists, CRAuth, GrConfBranch, GRConfig };

pub fn initialize_gr() -> Result<()> {
    let git = Git::new();
    let candy = Candy::new();
    let gr_dir = config_dir_path()?;
    let config_file_path = format!("{}/config.toml", gr_dir);

    // Check if the config file exists
    if config_file_exists(&config_file_path) {
        if candy.yn("gr is already initialized - reinitialize?") {}
        else {
            println!("{}", "Aborted initialization".red());
            return Ok(());
        }
    }

    println!("{}", "Initializing GQ...".green());

    // Prepare -
    // Create the app's config directory if it doesn't exist
    std::fs::create_dir_all(gr_dir)?;

    // Gather configuration info from the user

    let root_branch = select_root_branch(&git)?;
    let remote = select_remote(&git)?;
    let cr_tool = select_review_tool()?;
    let cr_auth = get_cr_auth(&cr_tool)?;

    // Build config data

    let config = GRConfig {
        origin: remote.unwrap_or("".to_string()),
        root_branch: root_branch,
        code_review_tool: cr_tool,
        code_review_user: cr_auth.user,
        code_review_pass: cr_auth.pass,
        code_review_key: cr_auth.token,
        version: "1.0.0".to_string(),
        branches: build_branch_conf(&git)?,
    };

    // Serialize to TOML
    let toml = toml::to_string_pretty(&config)?;

    // (over)Write config file
    std::fs::write(config_file_path, toml)?;

    Ok(())
}

fn build_branch_conf(git: &Git) -> Result<Vec<GrConfBranch>> {
    let branches = git.branches()?;

    // TODO: add support for tracking remote branches
    let mut config = Vec::new();
    for b in branches.iter() {
        config.push(build_gr_conf_branch(git, b)?);
    }
    Ok(config)
}

fn build_gr_conf_branch(git: &Git, branch: &str) -> Result<GrConfBranch> {
    Ok(GrConfBranch { name: branch.to_string(), parent: git.parent_of(branch, BranchType::Local)?, remote_branch: None, review_id: None })
}

fn get_cr_auth(cr_tool: &CodeReviewService) -> Result<CRAuth> {
    let candy = Candy::new();
    match cr_tool {
        CodeReviewService::None => Ok(CRAuth { user: None, pass: None, token: None }),
        CodeReviewService::Github => {
            // check for pre-configured env vars
            if let Ok(token) = std::env::var("GITHUB_TOKEN") {
                if candy.yn("Found GITHUB_TOKEN in environment variables. Use it?") {
                    return Ok(CRAuth { user: None, pass: None, token: Some(token) });
                }
            }
            if let (Ok(user), Ok(pass)) = (std::env::var("GITHUB_USER"), std::env::var("GITHUB_PASS")) {
                if candy.yn("Found GITHUB_USER and GITHUB_PASS in environment variables. Use them?") {
                    return Ok(CRAuth { user: Some(user), pass: Some(pass), token: None });
                }
            }

            if candy.yn("Do you have a personal access token?") {
                let Submit(token) = candy.edit_line("Paste your Github token: ", None) else { Err(anyhow!("Cancelled"))? };
                Ok(CRAuth { user: None, pass: None, token: Some(token) })
            } else {
                let Submit(user) = candy.edit_line("Enter your Github username: ", None) else { Err(anyhow!("Cancelled"))? };
                let Submit(pass) = candy.edit_line("Enter your Github password: ", None) else { Err(anyhow!("Cancelled"))? };
                Ok(CRAuth { user: Some(user), pass: Some(pass), token: None })
            }
        }
    }
}

fn select_remote(git: &Git) -> Result<Option<String>> {
    let candy = Candy::new();
    let remotes = git.remotes()?;
    if remotes.is_empty() {
        return Ok(None);
    }

    match candy.select_one("Select your remote:", remotes) {
        Submit(remote) => {
            let msg = format!("  {} {}", "Remote: ".green(), remote.clone().cyan());
            println!("{}", msg);

            Ok(Some(remote.clone()))
        },
        Cancel => {
            println!("  {}", "No git remote selected - Proceeding without one".yellow());
            return Ok(None);
        },
        ce => {
            Err(anyhow!("Unhandled event from Candy: {:?}", ce))
        }
    }
}

fn select_review_tool() -> Result<CodeReviewService> {
    let candy = Candy::new();
    match candy.select_one("Select your review tool:", vec![CodeReviewService::None.to_string(), CodeReviewService::Github.to_string()]) {
        Submit(tool) => {
            let tool = match tool.as_str() {
                "Github" => CodeReviewService::Github,
                _ => CodeReviewService::None
            };
            let msg = format!("{} {}", "Review tool: ".green(), tool);
            println!("  {}", msg);
            Ok(tool)
        },
        Cancel => {
            println!("  {}", "No review tool selected - Aborted initialization".red());
            Err(anyhow!("No review tool selected"))
        },
        ce => {
            Err(anyhow!("Unhandled event from Candy: {:?}", ce))
        }
    }
}

fn select_root_branch(git: &Git) -> Result<String> {
    let candy = Candy::new();
    let branches = git.branches()?;
    match candy.select_one("Select root branch:", branches) {
        Submit(branch) => {
            let msg = format!("  {} {}", "Root branch: ".green(), branch.clone().cyan());
            println!("{}", msg);
            Ok(branch.clone())
        },
        Cancel => {
            println!("  {}", "No root branch selected - Aborted initialization".red());
            Err(anyhow!("No root branch selected"))
        },
        ce => {
            Err(anyhow!("Unhandled event from Candy: {:?}", ce))
        }
    }
}

