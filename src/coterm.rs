use crate::structs::Completion;
use colored::*;
use dialoguer::Input;
use reqwest;
use serde_json::json;
use std::process::Command;

pub async fn get_command_from_openai(prompt: String, api_key: String) -> String {
    let client = reqwest::Client::new();
    let url = "https://api.openai.com/v1/completions";
    let params = json!({
        "model": "text-davinci-003",
        "prompt": prompt,
        "max_tokens": 256,
        "temperature": 0.3,
        "best_of": 3,
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!("Bearer {}", api_key.clone()).as_str(),
        )
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

pub async fn command_loop(prompt: String, max_attempts: i32, api_key: String) {
    let mut prompt_template = format!(
        "User: I want to {} on {}. What is the shell command? Write only the shell command and nothing else.\nAI:",
        prompt,
        std::env::consts::OS
    );

    for i in 0..max_attempts {
        let command = get_command_from_openai(prompt_template.clone(), api_key.clone()).await;

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

        if i == max_attempts - 1 {
            println!("Max attempts reached. Exiting.");
        }
    }
}
