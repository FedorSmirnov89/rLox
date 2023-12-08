use anyhow::{anyhow, Result};

use crate::domain::{grammar::Primary, scanning::TokenType};

use super::Parser;

impl<'tokens> Parser<'tokens> {
    pub(super) fn primary(&mut self) -> Result<Primary> {
        let current = self.current().ok_or(anyhow!("Unexpected end of input"))?;

        let primary = match current.t_type {
            TokenType::Number(n) => Primary::Number(n.into()),
            TokenType::String(ref s) => Primary::String(s.clone().into()), // TODO check whether we can not copy here
            TokenType::Identifier(ref i) => Primary::Identifier(i.clone().into()),
            TokenType::TRUE => Primary::True,
            TokenType::FALSE => Primary::False,
            TokenType::NIL => Primary::Nil,
            TokenType::ParenLeft => {
                self.advance();
                let expr = self.expression()?;
                self.expect(&TokenType::ParenRight)?;
                Primary::GroupedExpression(Box::new(expr))
            }

            TokenType::BraceLeft => todo!(),
            TokenType::BraceRight => todo!(),
            TokenType::ParenRight => todo!(),
            TokenType::Comma => todo!(),
            TokenType::Dot => todo!(),
            TokenType::Minus => todo!(),
            TokenType::Plus => todo!(),
            TokenType::Semicolon => todo!(),
            TokenType::Star => todo!(),
            TokenType::EqualEqual => todo!(),
            TokenType::BangEqual => todo!(),
            TokenType::LessEqual => todo!(),
            TokenType::GreaterEqual => todo!(),
            TokenType::Equal => todo!(),
            TokenType::Bang => todo!(),
            TokenType::Less => todo!(),
            TokenType::Greater => todo!(),
            TokenType::Division => todo!(),
            TokenType::PRINT => todo!(),
            TokenType::AND => todo!(),
            TokenType::CLASS => todo!(),
            TokenType::ELSE => todo!(),
            TokenType::FOR => todo!(),
            TokenType::FUN => todo!(),
            TokenType::IF => todo!(),
            TokenType::OR => todo!(),
            TokenType::RETURN => todo!(),
            TokenType::SUPER => todo!(),
            TokenType::THIS => todo!(),
            TokenType::VAR => todo!(),
            TokenType::WHILE => todo!(),
            TokenType::EOF => todo!(),
        };
        self.advance();
        Ok(primary)
    }
}

#[cfg(test)]
mod test {
    use claim::assert_err;

    use crate::{
        domain::{
            grammar::{Expression, Primary},
            scanning::{Location, Token, TokenType},
        },
        parser::parse,
    };

    #[test]
    fn correct_grouping() {
        let location = Location {
            column: 0,
            line: 0,
            pos: 0,
        };

        let input = vec![
            Token::one_char(TokenType::ParenLeft, location),
            Token::number("42", location),
            Token::one_char(TokenType::ParenRight, location),
            Token::eof(location),
        ];

        let output = parse(input).expect("parsing failed");

        let expected_inner: Expression = 42.0.into();
        let expected_group = Primary::grouped_expr(expected_inner);

        assert_eq!(vec![expected_group], output);
    }

    #[test]
    fn missing_closing_bracket() {
        let location = Location {
            column: 0,
            line: 0,
            pos: 0,
        };

        let input = vec![
            Token::one_char(TokenType::ParenLeft, location),
            Token::number("42", location),
            Token::eof(location),
        ];

        let output = parse(input);
        assert_err!(output);
    }
}
