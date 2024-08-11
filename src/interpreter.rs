use std::{
    fmt::{Debug, Display},
    vec,
};

use anyhow::Result;
use itertools::Itertools;

use crate::{
    domain::grammar::{Declaration, Expression, Program, Statement},
    parser::{self, ErrorLocation, ParserError},
    scanner::scan_input,
};

use self::{error::InterpreterError, statements::InterpretedStatement};

pub mod environment;
pub mod error;
pub mod value;

pub use environment::*;
pub use value::{Value, ValueType};

mod expressions;
mod statements;

#[derive(Default)]
pub struct Interpreter {
    environment: Environment,
}

pub enum LoxError {
    InterpreterError(String),
    ParserError(ParserErrDesc),
    Other(String),
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_msg = match self {
            LoxError::InterpreterError(msg) => msg,
            LoxError::Other(msg) => msg,
            LoxError::ParserError(_desc) => todo!(),
        };

        write!(f, "{err_msg}")
    }
}

impl Debug for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{s}", s = self)
    }
}

pub struct ParserErrDesc {
    context: String,
    prefix: String,
    highlighted: String,
    suffix: String,
}

impl Interpreter {
    ///
    /// Interprets the given source string while mutating the current state of the interpreter
    ///
    pub fn interpret_src_str(&mut self, source_str: &str) -> Result<Option<Value>, Vec<LoxError>> {
        println!("interpreting the following: '{source_str}'");
        let tokens = scan_input(source_str).map_err(|anyhow_errs| {
            let lox_errors: Vec<LoxError> = anyhow_errs
                .into_iter()
                .map(|err| LoxError::Other(err.to_string()))
                .collect();
            lox_errors
        })?;
        let program = parser::parse(tokens).map_err(|parser_errs| {
            let anyhow_errors: Vec<LoxError> = parser_errs
                .into_iter()
                .map(|err| from_parser_err(err, source_str))
                .collect();
            anyhow_errors
        })?;

        if let Some(expr) = single_expression(&program) {
            print_expr_ast(expr);
        }

        match self.interpret(program) {
            Ok(value) => Ok(value),
            Err(errors) => {
                let interpreter_errs: Vec<LoxError> = errors
                    .into_iter()
                    .map(|err| from_interpreter_err(err, source_str))
                    .collect();
                Err(interpreter_errs)
            }
        }
    }
    ///
    /// Interprets the given program. If the last statement is an expression, the value of that
    /// expression is returned.
    ///
    pub(crate) fn interpret(
        &mut self,
        program: Program,
    ) -> Result<Option<Value>, Vec<InterpreterError>> {
        let environment = &mut self.environment;
        let mut errors = vec![];
        for decl in program.into_iter() {
            match decl.interpret_statement(environment) {
                Ok(Outcome::Void) => (),
                Ok(Outcome::Return(return_value)) | Ok(Outcome::Value(return_value)) => {
                    return Ok(Some(return_value));
                }
                Err(e) => errors.push(e),
            }
        }
        if errors.is_empty() {
            Ok(environment.get_tmp_value().cloned())
        } else {
            Err(errors)
        }
    }

    pub fn environment(&self) -> &Environment {
        &self.environment
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

fn from_parser_err(err: ParserError, source_str: &str) -> LoxError {
    let location = err.location();
    let context = err.context();
    let desc = err_desc(source_str, location, context);
    LoxError::ParserError(desc)
}

fn from_interpreter_err(err: InterpreterError, source_str: &str) -> LoxError {
    let msg = err.msg(source_str);
    LoxError::InterpreterError(msg)
}

const LEN_HIGHLIGHTED: usize = 1; // number of highlighted characters
const LEN_DISPLAYED: usize = 10; // number of non-highlighted characters before and after the highlighted characters

fn err_desc(source_str: &str, location: &ErrorLocation, context: &str) -> ParserErrDesc {
    let len = source_str.len();
    let err_loc = match location {
        ErrorLocation::EndOfInput => len - 1,
        ErrorLocation::Position(loc) => loc.pos,
    };

    let start_dsp = err_loc.saturating_sub(LEN_DISPLAYED);
    let start_hl = err_loc.saturating_sub(LEN_HIGHLIGHTED - 1);
    let end_hl = (err_loc + LEN_HIGHLIGHTED).min(len);
    let end_dsp = (err_loc + LEN_DISPLAYED + 1).min(len);

    let split_indices = vec![0, start_dsp, start_hl, end_hl, end_dsp, source_str.len()];
    let (_front, dsp_prefix, highlighted, dsp_suffix, _back) = split_indices
        .into_iter()
        .tuple_windows()
        .map(|(start, end)| &source_str[start..end])
        .collect_tuple()
        .expect("failed to split into substrings");

    ParserErrDesc {
        context: context.to_owned(),
        prefix: dsp_prefix.to_owned(),
        highlighted: highlighted.to_owned(),
        suffix: dsp_suffix.to_owned(),
    }
}

#[must_use]
#[derive(Debug)]
pub(crate) enum Outcome {
    Return(Value),
    Value(Value),
    Void,
}

#[cfg(test)]
mod test {
    use crate::{domain::location::Location, parser::ErrorLocation};

    #[test]
    fn error_desc_long_input() {
        // Arrange
        let source_str = "this is the input we are processing;\n We assume an error here: X .\nWe also have a lot of other input after that.";
        let context = "test context";
        let location = Location {
            line: 2,
            column: 26,
            pos: 63,
        };
        let err_location = ErrorLocation::Position(location);

        // Act
        let desc = super::err_desc(source_str, &err_location, context);

        // Assert
        assert_eq!(desc.context, context);
        assert_eq!(desc.prefix, "ror here: ");
        assert_eq!(desc.highlighted, "X");
        assert_eq!(desc.suffix, " .\nWe also");
    }

    #[test]
    fn error_desc_long_input_eof() {
        // Arrange
        let source_str = "this is the input we are processing;\n We assume an error here: X .\nWe also have a lot of other input after that.";
        let context = "test context";
        let err_location = ErrorLocation::EndOfInput;

        // Act
        let desc = super::err_desc(source_str, &err_location, context);

        // Assert
        assert_eq!(desc.context, context);
        assert_eq!(desc.prefix, "after that");
        assert_eq!(desc.highlighted, ".");
        assert_eq!(desc.suffix, "");
    }

    #[test]
    fn error_desc_short_prefix() {
        // Arrange
        let source_str = "this is the input we are processing;\n We assume an error here: X .\nWe also have a lot of other input after that.";
        let context = "test context";
        let location = Location {
            line: 1,
            column: 5,
            pos: 2,
        };
        let err_location = ErrorLocation::Position(location);

        // Act
        let desc = super::err_desc(source_str, &err_location, context);

        // Assert
        assert_eq!(desc.context, context);
        assert_eq!(desc.prefix, "th");
        assert_eq!(desc.highlighted, "i");
        assert_eq!(desc.suffix, "s is the i");
    }

    #[test]
    fn error_desc_short_suffix() {
        // Arrange
        let source_str = "this is the input we are processing;\n We assume an error here: X .\nWe also have a lot of other input after that.";
        let context = "test context";
        let location = Location {
            line: 3,
            column: 1,
            pos: 101,
        };
        let err_location = ErrorLocation::Position(location);

        // Act
        let desc = super::err_desc(source_str, &err_location, context);

        // Assert
        assert_eq!(desc.context, context);
        assert_eq!(desc.prefix, "her input ");
        assert_eq!(desc.highlighted, "a");
        assert_eq!(desc.suffix, "fter that.");
    }
}
