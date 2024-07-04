use serde::{Deserialize, Serialize};

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