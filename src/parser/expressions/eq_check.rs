use anyhow::Result;

use crate::{
    domain::{grammar::Equality, scanning::TokenType},
    matches_t_type,
    parser::Parser,
};

impl<'tokens> Parser<'tokens> {
    pub(crate) fn equality(&mut self) -> Result<Equality> {
        let mut comp = Equality::Comparison(self.comparison()?);
        let mut current = self.current()?;
        while matches_t_type!(current, &TokenType::EqualEqual, &TokenType::BangEqual) {
            self.advance(); // consume the ==/!=
            let left = Box::new(comp);
            let right = self.comparison()?;
            comp = match current.t_type() {
                TokenType::EqualEqual => Equality::EqualityCheck { left, right },
                TokenType::BangEqual => Equality::InequalityCheck { left, right },
                _ => unreachable!(),
            };
            if let Ok(c) = self.current() {
                current = c;
            } else {
                break;
            }
        }
        Ok(comp)
    }
}
