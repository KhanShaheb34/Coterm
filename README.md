# 🤖 coterm

Copilot for your terminal using GPT3

## Installation

### Automated installation:

```bash
curl -s https://raw.githubusercontent.com/KhanShaheb34/coterm/main/install.sh | bash
```

On first run it will ask for your OpenAI API key. You can get it from [here](https://beta.openai.com/account/api-keys).

Later you can change the api key from `$HOME/.coterm/.env` file.

> TODO: Change API key from cli

### Manual installation:

- Install Rust and Cargo from [here](https://www.rust-lang.org/tools/install).
- Clone the repository and `cd` into it.
- Make a copy of `.env.example` to `.env` and add OpenAI API key to the environment variable.
- Run `cargo build --release` to build the binary.
- Copy the binary from `target/release/coterm` to your `PATH`.

Get your API key from [here](https://beta.openai.com/account/api-keys).

## Usage

- Run `coterm` in your terminal.
- Type your prompt and press `Enter`.
- Press `Ctrl + C` to exit.

> TODO: Create binary for easy installation

## Examples

[![asciicast](https://asciinema.org/a/OPYtzVWXJL64YhSXX2Nml6fEP.svg)](https://asciinema.org/a/OPYtzVWXJL64YhSXX2Nml6fEP?autoplay=1)

[![asciicast](https://asciinema.org/a/lHpiN2qGq3nowhTxjXvP60xDP.svg)](https://asciinema.org/a/lHpiN2qGq3nowhTxjXvP60xDP?autoplay=1)
