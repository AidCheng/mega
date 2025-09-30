use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ThirdPartyCommit {
    pub sha: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ThirdPartyTree {
    pub sha: String,
    pub url: String,
    pub tree: Vec<ThirdPartyTreeNode>,
    pub truncated: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ThirdPartyTreeNode{
    pub path: String,
    pub mode: String,
    pub sha: String,
    pub url: String,
}

impl ThirdPartyTreeNode {
     pub fn new() -> Self {
        Self {
            path: String::new(),
            mode: String::new(),
            sha: String::new(),
            url: String::new(),
        }
    }
}


