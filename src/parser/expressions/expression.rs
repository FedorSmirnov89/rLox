use anyhow::Result;

use crate::{domain::grammar::Expression, parser::Parser};

impl<'tokens> Parser<'tokens> {
    pub(crate) fn expression(&mut self) -> Result<Expression> {
        let l_or = self.logic_or()?;
        Ok(Expression::LogicOr(l_or))
    }
}
