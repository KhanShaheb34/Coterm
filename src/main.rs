use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    finish_reason: String,
    index: usize,
    logprobs: Option<()>,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Usage {
    completion_tokens: usize,
    prompt_tokens: usize,
    total_tokens: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Completion {
    choices: Vec<Choice>,
    created: usize,
    id: String,
    model: String,
    object: String,
    usage: Usage,
}

async fn get_command_from_openai(prompt: String) -> String {
    let client = reqwest::Client::new();
    let url = "https://api.openai.com/v1/completions";
    let params = json!({
        "model": "text-davinci-003",
        "prompt": prompt,
        "temperature": 0.2
    });
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key).as_str())
        .json(&params)
        .send()
        .await
        .expect("Error sending request")
        .json::<Completion>()
        .await
        .expect("Error parsing response");

    let command = response.choices[0].text.trim();
    command.to_string()
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args: Vec<String> = std::env::args().collect();
    let prompt = args[1].clone();

    let prompt_template = format!(
        "I want to {} on {} OS. What is the command? Write only the command and nothing else.",
        prompt,
        std::env::consts::OS
    );

    let command = get_command_from_openai(prompt_template).await;
    println!("Command: {}", command);
}
