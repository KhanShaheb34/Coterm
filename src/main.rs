mod args;
mod coterm;
mod structs;

use args::{validate_prompt, CotermArgs};
use clap::Parser;
use coterm::command_loop;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    let args = CotermArgs::parse();

    dotenv().ok();
    let prompt = args.prompt.join(" ");
    validate_prompt(prompt.clone());

    let max_attempts = 10;
    command_loop(prompt, max_attempts).await;
}
