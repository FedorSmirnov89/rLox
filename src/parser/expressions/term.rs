use anyhow::Result;

use crate::{
    domain::{grammar::Term, scanning::TokenType},
    matches_t_type,
    parser::Parser,
};

impl<'tokens> Parser<'tokens> {
    ///
    /// Reads out a term expression from the current position in the token stream. Also advances the
    /// current position in the token stream to the next token after the term.
    ///
    pub(super) fn term(&mut self) -> Result<Term> {
        let mut term = Term::Factor(self.factor()?);
        if let Ok(mut current) = self.current() {
            while matches_t_type!(current, &TokenType::Plus, &TokenType::Minus) {
                self.advance();
                let left = Box::new(term);
                let right = self.factor()?;
                term = match current.t_type() {
                    TokenType::Plus => Term::Addition { left, right },
                    TokenType::Minus => Term::Subtraction { left, right },
                    _ => unreachable!(),
                };
                if let Ok(c) = self.current() {
                    current = c;
                } else {
                    break;
                }
            }
        }
        Ok(term)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        domain::{
            grammar::Term,
            location::Location,
            scanning::{Token, TokenType},
        },
        parser::{assert_expression, parse},
    };

    #[test]
    fn simple_term() {
        let loc = Location::default();

        let input = vec![
            Token::string("a", loc),
            Token::one_char(TokenType::Plus, loc),
            Token::string("b", loc),
            Token::semicolon(loc),
            Token::eof(loc),
        ];

        let output = parse(input).expect("failed to parse");

        let expected_term = Term::string_addition("a", "b");
        assert_expression(output, expected_term.into());
    }
}
