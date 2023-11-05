use anyhow::{Context, Result};
use std::fmt::Write;

pub mod domain;
pub mod errors;

mod arguments;
mod scanner;

pub use arguments::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};

use crate::scanner::scan_input;

pub fn interpret_lox_file(path: &str) -> Result<()> {
    let lox_str = std::fs::read_to_string(path)
        .with_context(|| format!("error reading in file at '{path}'"))?;
    let mut interpreter = Interpreter::default();
    if let Err(errors) = interpreter.interpret_src_str(&lox_str) {
        Err(summarize_errors(errors)?)
    } else {
        println!("All done");
        Ok(())
    }
}

const COMMAND_EXIT: &str = "exit";

pub fn run_prompt() -> Result<()> {
    let prompt_theme = ColorfulTheme::default();
    let prompt =
        format!("Enter the next line of lox code. Type '{COMMAND_EXIT}' to terminate the prompt.");
    let mut interpreter = Interpreter::default();

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

        if let Err(errors) = interpreter.interpret_src_str(&input) {
            let err_summary = summarize_errors(errors)?;
            return Err(err_summary);
        }
    }
    Ok(())
}

#[derive(Default)]
pub struct Interpreter {}

impl Interpreter {
    ///
    /// Interprets the given source string while mutating the current state of the interpreter
    ///
    pub fn interpret_src_str(&mut self, source_str: &str) -> Result<(), Vec<anyhow::Error>> {
        let _tokens = scan_input(source_str)?;
        println!("interpreting the following: '{source_str}'");
        Ok(())
    }
}

fn summarize_errors(errors: Vec<anyhow::Error>) -> Result<anyhow::Error> {
    let mut msg = String::new();
    for e in errors {
        writeln!(msg, "{e}").context("error writing summary error")?;
    }
    Ok(anyhow::anyhow!("Input error:\n{msg}"))
}
