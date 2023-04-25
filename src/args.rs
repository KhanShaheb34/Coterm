use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    bin_name = "ct",
    author = "KhanShaheb34",
    version = "0.1.0",
    about = "Copilot for your terminal",
    long_about = "Copilot for your terminal using GPT3 and Rust"
)]
pub struct CotermArgs {
    #[clap()]
    /// What do you want to do?
    pub prompt: Vec<String>,
}

pub fn validate_prompt(prompt: String) {
    if prompt.is_empty() {
        panic!("Prompt cannot be empty");
    }
}
