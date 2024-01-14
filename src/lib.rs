use anyhow::{Context, Result};
use domain::grammar::{Declaration, Expression, Program, Statement};
use std::fmt::Write;

pub mod domain;
pub mod errors;

mod arguments;
mod interpreter;
mod parser;
mod scanner;

pub use arguments::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
pub use interpreter::State;
pub use interpreter::{Value, ValueType};

use crate::{domain::location::CodeSpan, scanner::scan_input};

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

#[derive(Default)]
pub struct Interpreter {
    state: State,
}

impl Interpreter {
    ///
    /// Interprets the given source string while mutating the current state of the interpreter
    ///
    pub fn interpret_src_str(
        &mut self,
        source_str: &str,
    ) -> Result<Option<Value>, Vec<anyhow::Error>> {
        println!("interpreting the following: '{source_str}'");
        let tokens = scan_input(source_str)?;
        let program = parser::parse(tokens)?;

        if let Some(expr) = single_expression(&program) {
            print_expr_ast(expr);
        }

        match self.interpret(program) {
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

fn single_expression(program: &Program) -> Option<Expression> {
    if program.len() != 1 {
        return None;
    }

    match &program[0] {
        Declaration::Statement(Statement::Expression(e)) => Some(e.clone()),
        _ => None,
    }
}

fn print_expr_ast(expr: Expression) {
    println!("here is the AST we got: ");
    println!("{}", expr);
}

fn summarize_errors(errors: Vec<anyhow::Error>) -> Result<anyhow::Error> {
    let mut msg = String::new();
    for e in errors {
        writeln!(msg, "{e}").context("error writing summary error")?;
    }
    Ok(anyhow::anyhow!("Input error:\n\n{msg}"))
}
