use anyhow::{anyhow, bail, Result};

use crate::domain::{
    grammar::{Equality, Expression, Program, Statement},
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

pub(super) fn parse(tokens: Vec<Token>) -> Result<Program, Vec<anyhow::Error>> {
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

    fn parse(mut self) -> Result<Program, Vec<anyhow::Error>> {
        let mut statements = vec![];
        let mut errors = vec![];
        while self.not_finished() {
            match self.statement() {
                Ok(s) => statements.push(s),
                Err(err) => {
                    errors.push(err);
                    self.synchronize();
                }
            }
        }
        if errors.is_empty() {
            Ok(Program(statements))
        } else {
            Err(errors)
        }
    }

    fn not_finished(&self) -> bool {
        &self.current().expect("current pos is out of bounds").t_type != &TokenType::EOF
    }

    ///
    /// Reads out an expression; Checks that it is followed by a semicolon. Also advances the current
    ///
    fn statement(&mut self) -> Result<Statement> {
        let statement = if let TokenType::PRINT =
            self.current().expect("current pos is out of bounds").t_type
        {
            self.advance();
            let expr = self.expression()?;
            Statement::Print(expr)
        } else {
            let expr = self.expression()?;
            Statement::Expression(expr)
        };
        self.expect(&TokenType::Semicolon)?;
        self.advance();
        Ok(statement)
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
fn assert_expression(program: Program, expected: Expression) {
    assert_eq!(1, program.len());
    match &program[0] {
        Statement::Expression(e) => assert_eq!(expected, *e),
        _ => panic!("Expected expression"),
    }
}

#[cfg(test)]
mod test {
    use crate::domain::{
        grammar::{Comparison, Equality, Expression, Factor, Primary, StringLiteral, Term, Unary},
        location::Location,
        scanning::{Token, TokenType},
    };

    use super::{assert_expression, parse};

    #[test]
    fn matches_basic() {
        use crate::domain::scanning::TokenType;

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
        use crate::domain::scanning::TokenType;

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
        let loc = Location::default();

        let input = vec![
            Token::string("a", loc),
            Token::one_two_char(TokenType::EqualEqual, loc),
            Token::string("b", loc),
            Token::semicolon(loc),
            Token::eof(loc),
        ];

        let output = parse(input).expect("failed to parse");

        let expected_expr = Expression::Equality(Equality::string_equality("a", "b"));
        assert_expression(output, expected_expr);
    }

    #[test]
    fn expr_and_print() {
        let loc = Location::default();

        let input = vec![
            Token::string("a", loc),
            Token::one_two_char(TokenType::EqualEqual, loc),
            Token::string("b", loc),
            Token::semicolon(loc),
            Token::keyword_or_identifier("print", loc),
            Token::string("abc", loc),
            Token::semicolon(loc),
            Token::eof(loc),
        ];

        let output = parse(input).expect("failed to parse");
        assert_eq!(2, output.len());

        let expected_first_expr = Expression::Equality(Equality::string_equality("a", "b"));
        let expected_second_expr = Expression::Equality(Equality::Comparison(Comparison::Term(
            Term::Factor(Factor::Unary(Unary::Primary(Primary::String(
                StringLiteral::new_string("abc", loc),
            )))),
        )));

        match &output[0] {
            crate::domain::grammar::Statement::Expression(expr) => {
                assert_eq!(expected_first_expr, *expr);
            }
            _ => panic!("Expected expression"),
        }
        match &output[1] {
            crate::domain::grammar::Statement::Print(expr) => {
                assert_eq!(expected_second_expr, *expr);
            }
            _ => panic!("Expected print statement"),
        }
    }
}
