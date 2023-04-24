mod args;
mod coterm;
mod structs;
mod utils;

use args::{validate_prompt, CotermArgs};
use clap::Parser;
use coterm::command_loop;
use dotenv::dotenv;
use utils::manage_environment_variables;

#[tokio::main]
async fn main() {
    let args = CotermArgs::parse();

    dotenv().ok();
    manage_environment_variables();

    let prompt = args.prompt.join(" ");
    validate_prompt(prompt.clone());
    println!("Prompt: {}", prompt);
    let max_attempts = 10;

    command_loop(prompt, max_attempts).await;
}
