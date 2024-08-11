//! Module for functionality which is shared between different parsing modules

use anyhow::Result;

use crate::{
    domain::{
        grammar::{Expression, StringLiteral},
        scanning::{Token, TokenType},
    },
    parser::{errors::ParserError, Parser},
};

impl<'tokens> Parser<'tokens> {
    pub(crate) fn parse_arguments(&mut self) -> Result<Option<Vec<Expression>>, ParserError> {
        self.parse_comma_separated(Self::expression)
    }

    pub(crate) fn parse_parameters(&mut self) -> Result<Option<Vec<String>>, ParserError> {
        self.parse_comma_separated(Self::parse_parameter)
    }

    fn parse_comma_separated<T, F>(&mut self, parse_func: F) -> Result<Option<Vec<T>>, ParserError>
    where
        F: Fn(&mut Self) -> Result<T, ParserError>,
    {
        if let TokenType::ParenLeft = self.current()?.t_type {
            self.advance();
        } else {
            return Ok(None);
        };

        let mut parameters = vec![];

        // case with no arguments
        if self.current()?.t_type == TokenType::ParenRight {
            self.advance();
        } else {
            loop {
                let parameter = parse_func(self)?;
                parameters.push(parameter);
                match self.current()?.t_type {
                    TokenType::ParenRight => {
                        self.advance();
                        break;
                    }
                    TokenType::Comma => self.advance(),
                    _ => return Ok(None), // incorrect arg list
                }
            }
        }
        Ok(Some(parameters))
    }

    fn parse_parameter(&mut self) -> Result<String, ParserError> {
        let cur_token = self.current()?;
        match &cur_token.t_type {
            TokenType::Identifier(ref s) => {
                self.advance();
                Ok(s.clone())
            }
            t => ParserError::token_mismatch(
                TokenType::Identifier("".into()),
                t.clone(),
                cur_token.location(),
                "parsing function parameter",
            ),
        }
    }

    pub(crate) fn parse_identifier(&mut self) -> Result<Option<String>, ParserError> {
        match self.current()?.t_type {
            TokenType::Identifier(ref s) => {
                self.advance();
                Ok(Some(s.clone()))
            }
            _ => Ok(None),
        }
    }

    pub(crate) fn identifier_from_token(token: &Token) -> Result<StringLiteral, ParserError> {
        let TokenType::Identifier(iden) = token.t_type() else {
            return ParserError::token_mismatch(
                TokenType::Identifier("".into()),
                token.t_type.clone(),
                token.location(),
                "reading identifier",
            );
        };
        let start = token.location();
        Ok(StringLiteral::new_identifier(iden, start))
    }
}
