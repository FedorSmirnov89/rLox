use std::{error::Error, fmt::Display};

use crate::domain::{location::Location, scanning::TokenType};

#[derive(Debug)]
pub(crate) enum ParserError {
    UnexpectedToken(UnexpectedToken),
    UnexpectedEndOfInput,
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedToken(e) => write!(f, "{}", e),
            ParserError::UnexpectedEndOfInput => write!(f, "Unexpected end of input"),
        }
    }
}

impl ParserError {
    pub(crate) fn unexpected_token<T>(
        expected: TokenType,
        actual: TokenType,
        location: Location,
        context: &'static str,
    ) -> Result<T, Self> {
        let error = ParserError::UnexpectedToken(UnexpectedToken {
            expected,
            actual,
            location,
            context,
        });
        Err(error)
    }
}

fn highlight_location(original_input: &str, location: Location) -> String {
    // let mut result = String::new();
    // let lines: Vec<&str> = original_input.split('\n').collect();
    // let line = lines[location.line as usize];
    // result.push_str(line);
    // result.push('\n');
    // for _ in 0..location.column {
    //     result.push(' ');
    // }
    // result.push('^');
    // result
    todo!()
}

#[derive(Debug)]
struct UnexpectedToken {
    expected: TokenType,
    actual: TokenType,
    location: Location,
    context: &'static str,
}

impl Display for UnexpectedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expected token type '{:?}' but got '{:?}'; Context: '{context}'; Token location: {loc};",
            self.expected,
            self.actual,
            context = self.context,
            loc = self.location,
        )
    }
}
