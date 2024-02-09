use anyhow::Result;

use crate::{
    domain::{grammar::Declaration, scanning::TokenType},
    parser::Parser,
};

impl<'tokens> Parser<'tokens> {
    pub(crate) fn declaration(&mut self) -> Result<Declaration> {
        if self.at_start_of_block()? {
            self.block()
        } else if self.at_start_of_var_declaration()? {
            self.advance();
            let var_declaration = self.var_declaration()?;
            Ok(Declaration::Declaration(var_declaration))
        } else {
            let statement = self.statement()?;
            Ok(Declaration::Statement(statement))
        }
    }

    fn at_start_of_var_declaration(&self) -> Result<bool> {
        Ok(self.current()?.t_type == TokenType::VAR)
    }
}
