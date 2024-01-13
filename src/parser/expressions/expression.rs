use anyhow::Result;

use crate::{
    domain::{
        grammar::{Equality, Expression},
        scanning::TokenType,
    },
    matches_t_type,
    parser::Parser,
};

impl<'tokens> Parser<'tokens> {
    pub(crate) fn expression(&mut self) -> Result<Expression> {
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
}
