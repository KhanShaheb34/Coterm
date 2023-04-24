mod args;
mod coterm;
mod structs;
mod utils;

use args::{validate_prompt, CotermArgs};
use clap::Parser;
use coterm::command_loop;
use dotenv::dotenv;
use utils::{get_api_key, set_api_key};

#[tokio::main]
async fn main() {
    let args = CotermArgs::parse();

    dotenv().ok();
    let api_key = get_api_key();
    let prompt = args.prompt.join(" ");

    if let Some(api_key_arg) = args.api_key {
        set_api_key(api_key_arg);
        return;
    }
    validate_prompt(prompt.clone());

    let max_attempts = 10;
    command_loop(prompt, max_attempts, api_key.clone()).await;
}
