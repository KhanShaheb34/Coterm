use crate::structs::{PromptBlock, Response};
use colored::*;
use dialoguer::Input;
use reqwest;
use serde_json::json;
use std::{mem, process::Command};

pub async fn get_command_from_openai(prompt_blocks: Vec<PromptBlock>) -> String {
    let client = reqwest::Client::new();
    let url = "https://coterm.vercel.app/api/openai";
    let params = json!({
        "prompts": prompt_blocks,
        "os": std::env::consts::OS,
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&params)
        .send()
        .await
        .expect("Error sending request");

    if response.status().is_success() {
        let data = response
            .json::<Response>()
            .await
            .expect("Error parsing response");

        if data.success == false {
            println!("Error: {}", data.message);
            std::process::exit(1);
        }

        return data.commands[0].clone();
    } else {
        println!("Error: {}", response.status());
        println!("{}", response.text().await.expect("Error reading response"));
        std::process::exit(1);
    }
}

pub async fn command_loop(prompt: String, max_attempts: usize) {
    let mut prompt_blocks: Vec<PromptBlock> = Vec::new();
    prompt_blocks.push(PromptBlock {
        user: prompt.clone(),
        ai: "".to_string(),
    });

    for i in 0..max_attempts {
        let command = get_command_from_openai(prompt_blocks.clone()).await;
        let _ = mem::replace(&mut prompt_blocks[i].ai, command.clone());

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
            prompt_blocks.push(PromptBlock {
                user: new_prompt.clone(),
                ai: "".to_string(),
            });
        }

        if i == max_attempts - 1 {
            println!("Max attempts reached. Exiting.");
        }
    }
}
