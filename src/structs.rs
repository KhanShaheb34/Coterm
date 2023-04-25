use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromptBlock {
    pub user: String,
    pub ai: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub success: bool,
    pub message: String,
    pub commands: Vec<String>,
}
