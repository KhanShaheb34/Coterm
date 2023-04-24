![coterm banner](images/coterm_banner.jpeg)

## Installation

### Automated installation:

```bash
curl -s https://raw.githubusercontent.com/KhanShaheb34/coterm/main/install.sh | sh
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

```bash
coterm [PROMPT]
```

For example if you want to read first 3 lines of a file:

```bash
coterm "kill process that is running on port 3000"
```

## Examples

[![asciicast](https://asciinema.org/a/tOqHkyYAiSEWTLWIN1w9xHcMB.svg)](https://asciinema.org/a/tOqHkyYAiSEWTLWIN1w9xHcMB?autoplay=1)
