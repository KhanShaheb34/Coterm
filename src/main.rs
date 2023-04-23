use colored::*;
use dialoguer::Input;
use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::process::Command;

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
        "temperature": 0.3,
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
    if args.len() < 2 {
        let err = "Please provide a prompt. Example:".red();
        let example = "ct \"delete a file\"".green();
        println!("{}", err);
        println!("$ {}", example);
        std::process::exit(1);
    }
    let prompt = args[1].clone();

    let max_attempts = 10;

    let mut prompt_template = format!(
        "User: I want to {} on {} OS. What is the command? Write only the command and nothing else.\nAI: ",
        prompt,
        std::env::consts::OS
    );

    for _ in 0..max_attempts {
        let command = get_command_from_openai(prompt_template.clone()).await;

        println!("\nGenerated command:\n$ {}", format!("{}", command).green());
        let new_prompt = Input::<String>::new()
            .with_prompt("Press ENTER to run command, or type a new prompt")
            .allow_empty(true)
            .interact_text()
            .expect("Error reading input");

        if new_prompt.is_empty() {
            println!("\nRunning command:\n$ {}", format!("{}", command).green());
            let output = Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
                .expect("Error running command");
            println!("\nOutput:\n{}", String::from_utf8_lossy(&output.stdout));
            println!("{}", String::from_utf8_lossy(&output.stderr).red());
            break;
        } else {
            prompt_template = format!(
                "{}{}\nUser: Make this modification to the command: {}\nAI: ",
                prompt_template, command, new_prompt
            );
        }
    }
}
