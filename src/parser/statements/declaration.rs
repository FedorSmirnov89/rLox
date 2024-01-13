use anyhow::Result;

use crate::{domain::grammar::Declaration, parser::Parser};

impl<'tokens> Parser<'tokens> {
    pub(crate) fn declaration(&mut self) -> Result<Declaration> {
        // TODO add options for var declarations

        let statement = self.statement()?;
        Ok(Declaration::Statement(statement))
    }
}
