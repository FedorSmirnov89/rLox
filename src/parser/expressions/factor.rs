use anyhow::Result;

use crate::{
    domain::{grammar::Factor, scanning::TokenType},
    matches_t_type,
    parser::Parser,
};

impl<'tokens> Parser<'tokens> {
    pub(super) fn factor(&mut self) -> Result<Factor> {
        let mut factor = Factor::Unary(self.unary()?);
        if let Some(mut current) = self.current() {
            while matches_t_type!(current, &TokenType::Star, &TokenType::Division) {
                self.advance();
                let left = Box::new(factor);
                let right = self.unary()?;
                factor = match current.t_type() {
                    TokenType::Star => Factor::Multiplication { left, right },
                    TokenType::Division => Factor::Division { left, right },
                    _ => unreachable!(),
                };
                if let Some(c) = self.current() {
                    current = c;
                } else {
                    break;
                }
            }
        }

        Ok(factor)
    }
}

#[cfg(test)]
mod test {

    use crate::{
        domain::{
            grammar::Factor,
            location::Location,
            scanning::{Token, TokenType},
        },
        parser::{assert_expression, parse},
    };

    #[test]
    fn simple_factor() {
        let loc = Location::default();

        let input = vec![
            Token::string("a", loc),
            Token::one_char(TokenType::Star, loc),
            Token::string("b", loc),
            Token::semicolon(loc),
            Token::eof(loc),
        ];

        let output = parse(input).expect("failed to parse");

        let expected_factor = Factor::string_multiplication("a", "b");
        assert_expression(output, expected_factor.into());
    }
}
