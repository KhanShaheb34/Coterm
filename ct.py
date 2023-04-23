from dotenv import load_dotenv
from langchain.llms import OpenAI
from langchain.prompts import PromptTemplate
from langchain.chains import LLMChain
import os
import click

load_dotenv()

os_name = os.name
template = "User: I want to {prompt} " + f"on {os_name} OS. " + \
    "What is the command? Write only the command and nothing else." + \
    "\nAI:"

template_with_history = "{history}\nUser: Make this modification to the command: {prompt}.\nAI:"


@click.command()
@click.argument("prompt", type=str)
def ct(prompt):
    llm = OpenAI()
    max_attempts = 10

    for i in range(max_attempts):
        command, history = generate_with_prompt(llm, prompt) if i == 0 else generate_with_prompt_and_history(
            llm, prompt, history)

        click.echo("\nGenerated Command: \n$ " +
                   click.style(text=command, fg="green"))

        new_prompt = click.prompt(
            "Press ENTER to run command, or type a new prompt", default="", show_default=False)

        if new_prompt == "":
            click.echo("\nRunning Command: \n$ " +
                       click.style(text=command, fg="green"))
            click.echo("\nOutput:")
            os.system(command)
            break
        else:
            prompt = new_prompt

        if i == max_attempts - 1:
            click.echo(
                "\nYou have reached the maximum number of attempts. Please try again.")


def generate_with_prompt(llm, prompt):
    prompt_template = PromptTemplate(
        input_variables=["prompt"], template=template)
    chain = LLMChain(llm=llm, prompt=prompt_template)

    command = chain(prompt)["text"].strip()

    history = f"{prompt_template.format(prompt=prompt)} {command}"

    return command, history


def generate_with_prompt_and_history(llm, prompt, history):
    prompt_template = PromptTemplate(
        input_variables=["prompt", "history"], template=template_with_history)
    chain = LLMChain(llm=llm, prompt=prompt_template)

    command = chain({"prompt": prompt, "history": history})["text"].strip()

    history = f"{prompt_template.format(prompt=prompt, history=history)} {command}"

    return command, history


if __name__ == "__main__":
    ct()
