# 🤖 coterm

Copilot for your terminal

## Installation

Install the required packages:

```
pip install -r requirements.txt
```

Make a copy of `.env.example` to `.env` and add OpenAI API key to the environment variable.

Get your API key from [here](https://beta.openai.com/account/api-keys).

## Usage

Run the `ct.py` file with the prompt as an argument. Example:

```
python ct.py "read first 10 lines of a file"
```

> TODO: Create binary for easy installation

## Examples

```
> python ct.py "read contents of a file"

Generated Command:
$ cat <filename>
Press ENTER to run command, or type a new prompt: read from README.md

Generated Command:
$ cat README.md
Press ENTER to run command, or type a new prompt: read first three lines

Generated Command:
$ head -n 3 README.md
Press ENTER to run command, or type a new prompt: <enter>

Running Command:
$ head -n 3 README.md

Output:
# 🤖 coterm

Copilot for your terminal
```

```
> python ct.py "show all running processes"

Generated Command:
$ ps -ef
Press ENTER to run command, or type a new prompt: show only that is running on port 3000

Generated Command:
$ ps -ef | grep :3000
Press ENTER to run command, or type a new prompt: kill the process

Generated Command:
$ kill -9 $(ps -ef | grep :3000 | awk '{print $2}')
Press ENTER to run command, or type a new prompt: <enter>

Running Command:
$ kill -9 $(ps -ef | grep :3000 | awk '{print $2}')

Output:
```
