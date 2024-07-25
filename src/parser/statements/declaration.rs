use anyhow::Result;

use crate::{
    domain::{grammar::Declaration, scanning::TokenType},
    parser::{errors::ParserError, Parser},
};

impl<'tokens> Parser<'tokens> {
    pub(crate) fn declaration(&mut self) -> Result<Declaration, ParserError> {
        if self.at_start_of_fun_declaration()? {
            let fun_declaration = self.fun_declaration()?;
            Ok(Declaration::FuncDeclaration(fun_declaration))
        } else if self.at_start_of_var_declaration()? {
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

    fn at_start_of_fun_declaration(&self) -> Result<bool> {
        Ok(self.current()?.t_type == TokenType::FUN)
    }
}
