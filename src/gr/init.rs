use dirs::home_dir;
use std::error::Error;
use std::fmt::{Display, Formatter};
use colored::Colorize;
use gr_git::Git;
use gr_tui::TuiWidget;
use crate::config::{config_dir_path, config_file_exists, CRAuth, GrConfBranch, GRConfig, ReviewTool};

pub fn initialize_gr(tui: &mut TuiWidget) -> Result<(), Box<dyn Error>> {
    let git = Git::new();
    let gr_dir = config_dir_path()?;
    let config_file_path = format!("{}/config.toml", gr_dir);

    // Check if the config file exists
    if config_file_exists(&config_file_path) {
        if tui.yn("gr is already initialized - reinitialize?".into())? {}
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

    let root_branch = select_root_branch(tui, &git)?;
    let remote = select_remote(tui, &git)?;
    let cr_tool = select_review_tool(tui)?;
    let cr_auth = get_cr_auth(tui, &cr_tool)?;

    // Build config data

    let config = GRConfig {
        origin: remote.unwrap_or("".to_string()),
        root_branch: root_branch,
        code_review_tool: cr_tool.to_string(),
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

fn build_branch_conf(git: &Git) -> Result<Vec<GrConfBranch>, Box<dyn Error>> {
    let branches = git.branches()?;

    // TODO: add support for tracking remote branches
    let mut config = Vec::new();
    for b in branches.iter() {
        config.push(build_gr_conf_branch(git, b)?);
    }
    Ok(config)
}

fn build_gr_conf_branch(git: &Git, branch: &str) -> Result<GrConfBranch, Box<dyn Error>> {
    Ok(GrConfBranch { name: branch.to_string(), parent: git.parent_of(branch)?, remote_branch: None, review_id: None })
}

fn get_cr_auth(tui: &mut TuiWidget, cr_tool: &ReviewTool) -> Result<CRAuth, Box<dyn Error>> {
    match cr_tool {
        ReviewTool::None => Ok(CRAuth { user: None, pass: None, token: None }),
        ReviewTool::Github => {
            // check for pre-configured env vars
            if let Ok(token) = std::env::var("GITHUB_TOKEN") {
                if tui.yn("Found GITHUB_TOKEN in environment variables. Use it?".into())? {
                    return Ok(CRAuth { user: None, pass: None, token: Some(token) });
                }
            }
            if let (Ok(user), Ok(pass)) = (std::env::var("GITHUB_USER"), std::env::var("GITHUB_PASS")) {
                if tui.yn("Found GITHUB_USER and GITHUB_PASS in environment variables. Use them?".into())? {
                    return Ok(CRAuth { user: Some(user), pass: Some(pass), token: None });
                }
            }

            if tui.yn("Do you have a personal access token?".into())? {
                let token = tui.prompt("Paste your Github token: ".into())?;
                Ok(CRAuth { user: None, pass: None, token: Some(token) })
            } else {
                let user = tui.prompt("Enter your Github username: ".into())?;
                let pass = tui.prompt("Enter your Github password: ".into())?;
                Ok(CRAuth { user: Some(user), pass: Some(pass), token: None })
            }
        }
    }
}

fn select_remote(tui: &mut TuiWidget, git: &Git) -> Result<Option<String>, Box<dyn Error>> {
    let remotes = git.remotes()?;
    if remotes.is_empty() {
        return Ok(None);
    }

    let remote_op = tui.select_one("Select your remote:".into(), remotes)?;
    if remote_op.is_none() {
        println!("  {}", "No git remote selected - Proceeding without one".yellow());
        return Ok(None);
    }
    let remote = remote_op.unwrap();
    let msg = format!("  {} {}", "Remote: ".green(), remote.clone().cyan());
    println!("{}", msg);

    Ok(Some(remote.clone()))
}

fn select_review_tool(tui: &mut TuiWidget) -> Result<ReviewTool, Box<dyn Error>> {
    let tool_str = tui.select_one("Select your review tool:".into(), vec![ReviewTool::None.to_string(), ReviewTool::Github.to_string()])?.unwrap_or("None".to_string());
    let tool = match tool_str.as_str() {
        "Github" => ReviewTool::Github,
        _ => ReviewTool::None
    };

    let msg = format!("{} {}", "Review tool: ".green(), tool_str.clone().cyan());
    println!("  {}", msg);

    Ok(tool)
}

fn select_root_branch(tui: &mut TuiWidget, git: &Git) -> Result<String, Box<dyn Error>> {
    let branches = git.branches()?;
    let root_branch_op = tui.select_one("Select root branch:".into(), branches)?;
    if root_branch_op.is_none() {
        println!("{}", "No root branch selected - Aborted initialization".red());
        return Err("No root branch selected".into());
    }
    let root_branch = root_branch_op.unwrap();
    let msg = format!("  {} {}", "Root branch: ".green(), root_branch.clone().cyan());
    println!("{}", msg);

    Ok(root_branch.clone())
}

