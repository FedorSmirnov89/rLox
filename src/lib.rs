use anyhow::{Context, Result};
use std::fmt::Write;

pub mod domain;
pub mod errors;

mod arguments;
mod interpreter;
mod native_functions;
mod parser;
mod scanner;

pub use arguments::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
pub use interpreter::Environment;
pub use interpreter::{Interpreter, Value, ValueType};

use crate::domain::location::CodeSpan;

pub fn interpret_lox_file(path: &str) -> Result<()> {
    let lox_str = std::fs::read_to_string(path)
        .with_context(|| format!("error reading in file at '{path}'"))?;
    let mut interpreter = Interpreter::default();
    match interpreter.interpret_src_str(&lox_str) {
        Ok(Some(v)) => {
            println!("file interpreted; evaluation result: {v}");
            Ok(())
        }
        Ok(None) => {
            println!("file interpreted; no evaluation result");
            Ok(())
        }
        Err(errors) => Err(summarize_errors(errors)?),
    }
}

const COMMAND_EXIT: &str = "exit";

pub fn run_prompt() -> Result<()> {
    let prompt_theme = ColorfulTheme::default();
    let prompt =
        format!("Enter the next line of lox code. Type '{COMMAND_EXIT}' to terminate the prompt.");
    let mut interpreter = Interpreter::default();

    let mut last_value = Value::new(ValueType::Nil, CodeSpan::default());
    loop {
        let input: String = Input::with_theme(&prompt_theme)
            .with_prompt(&prompt)
            .interact_text()
            .context("failed reading in user prompt input")?;

        if &input == COMMAND_EXIT
            && Confirm::with_theme(&prompt_theme)
                .with_prompt("Do you want to terminate the interpreter?")
                .interact()?
        {
            break;
        }

        match interpreter.interpret_src_str(&input) {
            Ok(Some(v)) => {
                println!("evaluation result: {v}");
                last_value = v;
            }
            Ok(None) => {
                println!("no evaluation result");
            }
            Err(errors) => {
                let err_summary = summarize_errors(errors)?;
                println!("{err_summary}");
                // return Err(err_summary);
            }
        }
    }

    println!("last value: {last_value}");
    Ok(())
}

fn summarize_errors(errors: Vec<anyhow::Error>) -> Result<anyhow::Error> {
    let mut msg = String::new();
    for e in errors {
        writeln!(msg, "{e}").context("error writing summary error")?;
    }
    Ok(anyhow::anyhow!("Input error:\n\n{msg}"))
}
