use anyhow::{bail, Result};

use crate::{
    domain::{grammar::VarDeclaration, scanning::TokenType},
    parser::Parser,
};

impl<'tokens> Parser<'tokens> {
    pub(crate) fn var_declaration(&mut self) -> Result<VarDeclaration> {
        // first, we expect an identifier
        let TokenType::Identifier(iden) = self.current().unwrap().t_type.clone() else {
            bail!("Expected identifier");
        };
        self.advance();

        if self.current().unwrap().t_type == TokenType::Equal {
            self.advance();
            let expr = self.expression()?;
            self.expect(&TokenType::Semicolon)?;
            self.advance();
            Ok(VarDeclaration::DeclareAndAssign(iden, expr))
        } else {
            self.expect(&TokenType::Semicolon)?;
            self.advance();
            Ok(VarDeclaration::Declare(iden))
        }
    }
}
