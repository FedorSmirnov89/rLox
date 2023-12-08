use anyhow::{anyhow, bail, Result};

use crate::domain::{
    grammar::{Equality, Expression},
    scanning::{Token, TokenType},
};

mod comparison;
mod factor;
mod primary;
mod term;
mod unary;

#[macro_export]
macro_rules! matches_t_type {
    ( $token: ident, $t_type: expr ) => {
        $token.t_type.matches($t_type)
    };
    ( $token: ident, $t_type: expr, $($t_types: expr),+ ) => {
        matches_t_type!($token, $t_type) | matches_t_type!($token, $($t_types),+)
    };
}

pub(super) fn parse(tokens: Vec<Token>) -> Result<Vec<Expression>, Vec<anyhow::Error>> {
    Parser::new(&tokens).parse()
}

struct Parser<'tokens> {
    tokens: &'tokens [Token],
    cur_pos: usize,
}

impl<'tokens> Parser<'tokens> {
    fn new(tokens: &'tokens [Token]) -> Self {
        Self { tokens, cur_pos: 0 }
    }

    fn parse(mut self) -> Result<Vec<Expression>, Vec<anyhow::Error>> {
        let mut expressions = vec![];
        let mut errors = vec![];
        while self.not_finished() {
            match self.expression() {
                Ok(expr) => expressions.push(expr),
                Err(err) => {
                    errors.push(err);
                    self.synchronize();
                }
            }
        }
        if errors.is_empty() {
            Ok(expressions)
        } else {
            Err(errors)
        }
    }

    fn not_finished(&self) -> bool {
        &self.current().expect("current pos is out of bounds").t_type != &TokenType::EOF
    }

    fn expression(&mut self) -> Result<Expression> {
        let mut comp = Equality::Comparison(self.comparison()?);
        if let Some(mut current) = self.current() {
            while matches_t_type!(current, &TokenType::EqualEqual, &TokenType::BangEqual) {
                self.advance();
                let left = Box::new(comp);
                let right = self.comparison()?;
                comp = match current.t_type() {
                    TokenType::EqualEqual => Equality::EqualityCheck { left, right },
                    TokenType::BangEqual => Equality::InequalityCheck { left, right },
                    _ => unreachable!(),
                };
                if let Some(c) = self.current() {
                    current = c;
                } else {
                    break;
                }
            }
        }
        Ok(Expression::Equality(comp))
    }

    fn current(&self) -> Option<&'tokens Token> {
        self.tokens.get(self.cur_pos)
    }

    fn synchronize(&mut self) {
        let mut current = self.current().expect("current pos is out of bounds");
        while !matches_t_type!(current, &TokenType::Semicolon, &TokenType::EOF) {
            self.advance();
            current = self.current().expect("current pos is out of bounds");
        }
        match current.t_type() {
            TokenType::Semicolon => self.advance(),
            _ => (),
        }
    }

    fn advance(&mut self) {
        self.cur_pos += 1;
    }

    fn expect(&mut self, t_type: &TokenType) -> Result<()> {
        if let Some(current) = self.current() {
            if matches_t_type!(current, t_type) {
                Ok(())
            } else {
                bail!(
                    "Expected token type {:?} but got {:?}; Token location: {loc}",
                    t_type,
                    current.t_type(),
                    loc = current.location()
                )
            }
        } else {
            Err(anyhow!("Unexpected end of token stream"))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::domain::{
        grammar::{Equality, Expression},
        scanning::{Location, Token, TokenType},
    };

    use super::parse;

    #[test]
    fn matches_basic() {
        use crate::domain::scanning::{Location, TokenType};

        let location = Location {
            column: 0,
            line: 0,
            pos: 0,
        };
        let token = Token::one_char(TokenType::ParenLeft, location);
        assert!(matches_t_type!(token, &TokenType::ParenLeft));
        assert!(!matches_t_type!(token, &TokenType::ParenRight));
    }

    #[test]
    fn matches_multi() {
        use crate::domain::scanning::{Location, TokenType};

        let location = Location {
            column: 0,
            line: 0,
            pos: 0,
        };
        let token = Token::one_char(TokenType::ParenLeft, location);
        assert!(matches_t_type!(
            token,
            &TokenType::ParenLeft,
            &TokenType::ParenRight,
            &TokenType::BraceLeft,
            &TokenType::BraceRight
        ));
        assert!(!matches_t_type!(
            token,
            &TokenType::ParenRight,
            &TokenType::BraceLeft
        ));
    }

    #[test]
    fn simple_equality_check() {
        let loc = Location {
            column: 0,
            line: 0,
            pos: 0,
        };

        let input = vec![
            Token::string("a", loc),
            Token::one_two_char(TokenType::EqualEqual, loc),
            Token::string("b", loc),
            Token::eof(loc),
        ];

        let output = parse(input).expect("failed to parse");

        let expected_expr = Expression::Equality(Equality::string_equality("a", "b"));
        let expected = vec![expected_expr];

        assert_eq!(expected, output);
    }
}
