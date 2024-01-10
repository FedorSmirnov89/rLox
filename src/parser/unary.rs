use anyhow::{bail, Result};

use crate::{
    domain::{grammar::Unary, scanning::TokenType},
    matches_t_type,
};

use super::Parser;

impl<'tokens> Parser<'tokens> {
    pub(super) fn unary(&mut self) -> Result<Unary> {
        if let Some(current) = self.current() {
            if matches_t_type!(current, &TokenType::Bang, &TokenType::Minus) {
                self.advance();
                let unary = match current.t_type() {
                    TokenType::Bang => Unary::LogicalNegation(Box::new(self.unary()?)),
                    TokenType::Minus => Unary::ArithmNegation(Box::new(self.unary()?)),
                    _ => unreachable!(),
                };
                Ok(unary)
            } else {
                let primary = Unary::Primary(self.primary()?);
                Ok(primary)
            }
        } else {
            bail!("Unexpected end when parsing a unary");
        }
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::{
        domain::{
            grammar::Unary,
            location::Location,
            scanning::{Token, TokenType},
        },
        parser::{assert_expression, parse},
    };

    #[test]
    fn simple_negation() {
        let loc = Location::default();

        let input = vec![
            Token::one_char(TokenType::Minus, loc),
            Token::string("a", loc),
            Token::semicolon(loc),
            Token::eof(loc),
        ];

        let expected_unary = Unary::string_arithm_negation("a");
        let output = parse(input).expect("failed to parse");
        assert_expression(output, expected_unary.into());
    }
}
