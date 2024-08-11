use std::{error::Error, fmt::Display};

use crate::domain::{location::Location, scanning::TokenType};

#[derive(Debug)]
pub(crate) struct ParserError {
    context: String,
    location: ErrorLocation,
}

#[derive(Debug)]
pub(crate) enum ErrorLocation {
    EndOfInput,
    Position(Location),
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{msg}", msg = self.context)
    }
}

impl ParserError {
    pub(crate) fn context(&self) -> &str {
        &self.context
    }

    pub(crate) fn other_err(msg: impl Into<String>, location: Location) -> Self {
        ParserError {
            context: msg.into(),
            location: ErrorLocation::Position(location),
        }
    }

    pub(crate) fn unexpected_end<T>() -> Result<T, Self> {
        Err(Self::unexpected_end_err())
    }

    pub(crate) fn unexpected_end_err() -> Self {
        let msg = "unexpected end of token stream".to_owned();
        ParserError {
            context: msg,
            location: ErrorLocation::EndOfInput,
        }
    }

    pub(crate) fn unexpected_token<T>(
        token: TokenType,
        location: Location,
        context: &'static str,
    ) -> Result<T, Self> {
        let msg = msg_unexpected_token(token, context);
        let err = Self {
            context: msg,
            location: ErrorLocation::Position(location),
        };
        Err(err)
    }

    pub(crate) fn token_mismatch<T>(
        expected: TokenType,
        actual: TokenType,
        location: Location,
        context: &'static str,
    ) -> Result<T, Self> {
        let msg = msg_token_mismatch(expected, actual, context);
        let err = Self {
            context: msg,
            location: ErrorLocation::Position(location),
        };
        Err(err)
    }

    pub(crate) fn location(&self) -> &ErrorLocation {
        &self.location
    }
}

fn msg_unexpected_token(token: TokenType, context: &'static str) -> String {
    format!("Unexpected token: '{token:?}'; Context: '{context}'")
}

fn msg_token_mismatch(expected: TokenType, actual: TokenType, context: &'static str) -> String {
    format!(
        "Toke mismatch: Expected '{:?}' but got '{:?}'; Context: '{context}'",
        expected, actual
    )
}
