use anyhow::Result;

use crate::{
    domain::{
        grammar::{StringLiteral, VarDeclaration},
        scanning::TokenType,
    },
    parser::Parser,
};

impl<'tokens> Parser<'tokens> {
    pub(crate) fn var_declaration(&mut self) -> Result<VarDeclaration> {
        let iden = StringLiteral::identifier_from_token(self.current()?)?;
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
