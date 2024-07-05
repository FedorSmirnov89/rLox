//! Module for functionality which is shared between different parsing modules

use anyhow::{bail, Result};

use crate::{
    domain::{grammar::Expression, scanning::TokenType},
    parser::Parser,
};

impl<'tokens> Parser<'tokens> {
    pub(crate) fn parse_arguments(&mut self) -> Result<Option<Vec<Expression>>> {
        self.parse_comma_separated(Self::expression)
    }

    pub(crate) fn parse_parameters(&mut self) -> Result<Option<Vec<String>>> {
        self.parse_comma_separated(Self::parse_parameter)
    }

    fn parse_comma_separated<T, F>(&mut self, parse_func: F) -> Result<Option<Vec<T>>>
    where
        F: Fn(&mut Self) -> Result<T>,
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

    fn parse_parameter(&mut self) -> Result<String> {
        match self.current()?.t_type {
            TokenType::Identifier(ref s) => {
                self.advance();
                Ok(s.clone())
            }
            _ => bail!("failed to read function parameter"),
        }
    }

    pub(crate) fn parse_identifier(&mut self) -> Result<Option<String>> {
        match self.current()?.t_type {
            TokenType::Identifier(ref s) => {
                self.advance();
                Ok(Some(s.clone()))
            }
            _ => Ok(None),
        }
    }
}
