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

        if self.current()?.t_type == TokenType::Equal {
            self.advance();
            let expr = self.expression()?;
            self.expect(&TokenType::Semicolon, "semicolon after rhs of declaration")?;
            self.advance();
            Ok(VarDeclaration::DeclareAndAssign(iden, expr))
        } else {
            self.expect(
                &TokenType::Semicolon,
                "semicolon after declaration without assignment",
            )?;
            self.advance();
            Ok(VarDeclaration::Declare(iden))
        }
    }
}
