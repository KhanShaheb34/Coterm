![coterm banner](images/coterm_banner.jpeg)

![Platforms](https://img.shields.io/badge/Platform-linux%20%7C%20macos-blue?logo=linux&style=flat-square&logoColor=white) ![Hits](https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2FKhanShaheb34%2Fcoterm&count_bg=%23B7410E&title_bg=%23555555&icon=rust.svg&icon_color=%23E7E7E7&title=Coterm&edge_flat=true)

# Installation

## Automated installation:

```bash
curl -s https://raw.githubusercontent.com/KhanShaheb34/coterm/main/install.sh | bash
```

## Manual installation:

- Install Rust and Cargo from [here](https://www.rust-lang.org/tools/install).
- Clone the repository and `cd` into it.
- Run `cargo build --release` to build the binary.
- Copy the binary from `target/release/coterm` to your `PATH`.

# Usage

```bash
ct [PROMPT]
```

For example if you want a list of all typescript files:

```bash
ct list ts files
```

And after the tool generates a command you can add revisions to it, for example:

- ignore node modules
- sort by size
- etc.

## Show Help

Check out the help page for more information:

```bash
ct -h | --help
```

# Examples

![demo](images/demo.gif)

# Contributing

We welcome contributions from the community to help improve and enhance Coterm.
If you are interested in contributing, please take a moment to review our [Contribution Guidelines](docs/CONTRIBUTING.md) before submitting your changes.
The guidelines provide information on how to report issues, request new features, contribute code, review contributions, and adhere to our code of conduct.
We appreciate your support in making Coterm even better and thank you for considering contributing to our project!

# Code of Conduct

Please review and abide by our [Code of Conduct](docs/CODE_OF_CONDUCT.md) at all times during your involvement with this project.

# License

Coterm is licensed under the [GNU GENERAL PUBLIC LICENSE Version 3](LICENSE).
