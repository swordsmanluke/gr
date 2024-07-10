use std::error::Error;
use std::fmt::{Display, Formatter};
use dirs::home_dir;
use serde::{Deserialize, Serialize};

pub struct CRAuth {
    pub user: Option<String>,
    pub pass: Option<String>,
    pub token: Option<String>,
}

pub enum ReviewTool {
    None,
    Github,
}

impl Display for ReviewTool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReviewTool::None => write!(f, "None"),
            ReviewTool::Github => write!(f, "Github"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrConfBranch {
    pub name: String,
    pub parent: Option<String>,
    pub remote_branch: Option<String>,
    pub review_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GRConfig {
    pub origin: String,
    pub root_branch: String,
    pub code_review_tool: String,
    pub code_review_user: Option<String>,
    pub code_review_pass: Option<String>,
    pub code_review_key: Option<String>,
    pub version: String,
    pub branches: Vec<GrConfBranch>,
}

pub fn read_config() -> Result<GRConfig, Box<dyn Error>> {
    let config_file_path = format!("{}/config.toml", config_dir_path()?);
    let config = std::fs::read_to_string(config_file_path)?;
    let config: GRConfig = toml::from_str(&config)?;
    Ok(config)
}

pub fn config_file_exists(conf_file_path: &str) -> bool {
    std::path::Path::new(conf_file_path).exists()
}

pub fn config_dir_path() -> Result<String, Box<dyn Error>> {
    // Get the user's home directory
    let home = home_dir().unwrap();
    // the current working directory
    let cwd = std::env::current_dir().unwrap();

    // The project directory (the end of the path after the cwd)
    let project = cwd.as_path().iter().last().unwrap();

    // Now we can build our config directory, which will be ~/.config/gr/[project]/[path to project]
    let gr_dir = format!("{}/.config/gr/{}/{}",
                         home.to_str().unwrap(),
                         project.to_str().unwrap(),
                         cwd.to_str().unwrap());
    Ok(gr_dir)
}