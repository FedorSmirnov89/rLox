use anyhow::Result;

use crate::{
    domain::{grammar::Comparison, scanning::TokenType},
    matches_t_type,
    parser::Parser,
};

impl<'tokens> Parser<'tokens> {
    ///
    /// Reads out a comparison expression from the current position in the token stream.
    /// Also advances the current position in the token stream to the next token after the comparison.
    ///
    pub(crate) fn comparison(&mut self) -> Result<Comparison> {
        let mut comp = Comparison::Term(self.term()?);
        if let Ok(mut current) = self.current() {
            while matches_t_type!(
                current,
                &TokenType::Greater,
                &TokenType::GreaterEqual,
                &TokenType::Less,
                &TokenType::LessEqual
            ) {
                self.advance();
                let left = Box::new(comp);
                let right = self.term()?;
                comp = match current.t_type() {
                    TokenType::Greater => Comparison::Greater { left, right },
                    TokenType::GreaterEqual => Comparison::GreaterEqual { left, right },
                    TokenType::Less => Comparison::Less { left, right },
                    TokenType::LessEqual => Comparison::LessEqual { left, right },
                    _ => unreachable!(),
                };
                if let Ok(c) = self.current() {
                    current = c;
                } else {
                    break;
                }
            }
        }

        Ok(comp)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        domain::{
            grammar::Comparison,
            location::Location,
            scanning::{Token, TokenType},
        },
        parser::{assert_expression, parse},
    };

    #[test]
    fn simple_comparison() {
        let loc = Location::default();

        let input = vec![
            Token::string("a", loc),
            Token::one_two_char(TokenType::LessEqual, loc),
            Token::string("b", loc),
            Token::semicolon(loc),
            Token::eof(loc),
        ];

        let output = parse(input).expect("failed to parse");

        let expected_comp = Comparison::string_less_equal("a", "b");
        assert_expression(output, expected_comp.into());
    }
}
