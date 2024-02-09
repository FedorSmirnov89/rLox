use anyhow::Result;

use crate::{
    domain::{grammar::LogicOr, scanning::TokenType},
    matches_t_type,
    parser::Parser,
};

impl<'tokens> Parser<'tokens> {
    pub(crate) fn logic_or(&mut self) -> Result<LogicOr> {
        let mut l_and = LogicOr::LogicAnd(self.logic_and()?);
        let mut current = self.current()?;
        while matches_t_type!(current, &TokenType::OR) {
            self.advance(); // consume the or
            let left = Box::new(l_and);
            let right = self.logic_and()?;
            l_and = LogicOr::Or { left, right };
            if let Ok(c) = self.current() {
                current = c;
            } else {
                break;
            }
        }
        Ok(l_and)
    }
}
