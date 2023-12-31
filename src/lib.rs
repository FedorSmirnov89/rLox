use anyhow::{Context, Result};
use domain::grammar::Expression;
use std::fmt::Write;

pub mod domain;
pub mod errors;

mod arguments;
mod interpreter;
mod parser;
mod scanner;

pub use arguments::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
pub use interpreter::{Value, ValueType};

use crate::{domain::location::CodeSpan, scanner::scan_input};

pub fn interpret_lox_file(path: &str) -> Result<()> {
    let lox_str = std::fs::read_to_string(path)
        .with_context(|| format!("error reading in file at '{path}'"))?;
    let mut interpreter = Interpreter::default();
    match interpreter.interpret_src_str(&lox_str) {
        Ok(v) => {
            println!("file interpreted; evaluation result: {v}");
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
            Ok(v) => {
                println!("evaluation result: {v}");
                last_value = v;
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

#[derive(Default)]
pub struct Interpreter {}

impl Interpreter {
    ///
    /// Interprets the given source string while mutating the current state of the interpreter
    ///
    pub fn interpret_src_str(&mut self, source_str: &str) -> Result<Value, Vec<anyhow::Error>> {
        println!("interpreting the following: '{source_str}'");
        let tokens = scan_input(source_str)?;
        let expressions = parser::parse(tokens)?;
        print_ast(&expressions);

        match interpreter::interpret(expressions) {
            Ok(value) => Ok(value),
            Err(errors) => {
                let mut interpreter_errors = vec![];
                for error in errors {
                    let e = anyhow::anyhow!(error.msg(source_str));
                    interpreter_errors.push(e);
                }
                Err(interpreter_errors)
            }
        }
    }
}

fn print_ast(expressions: &[Expression]) {
    println!("here is the AST we got: ");
    for expr in expressions {
        println!("{}", expr);
    }
}

fn summarize_errors(errors: Vec<anyhow::Error>) -> Result<anyhow::Error> {
    let mut msg = String::new();
    for e in errors {
        writeln!(msg, "{e}").context("error writing summary error")?;
    }
    Ok(anyhow::anyhow!("Input error:\n\n{msg}"))
}
