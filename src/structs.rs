use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    finish_reason: String,
    index: usize,
    logprobs: Option<()>,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Usage {
    completion_tokens: usize,
    prompt_tokens: usize,
    total_tokens: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Completion {
    pub choices: Vec<Choice>,
    created: usize,
    id: String,
    model: String,
    object: String,
    usage: Usage,
}
