use anyhow::Result;

use crate::{
    domain::{grammar::LogicAnd, scanning::TokenType},
    matches_t_type,
    parser::Parser,
};

impl<'tokens> Parser<'tokens> {
    pub(crate) fn logic_and(&mut self) -> Result<LogicAnd> {
        let mut eq_check = LogicAnd::Equality(self.equality()?);
        let mut current = self.current()?;
        while matches_t_type!(current, &TokenType::AND) {
            self.advance(); // consume the and
            let left = Box::new(eq_check);
            let right = self.equality()?;
            eq_check = LogicAnd::And { left, right };
            if let Ok(c) = self.current() {
                current = c;
            } else {
                break;
            }
        }
        Ok(eq_check)
    }
}
