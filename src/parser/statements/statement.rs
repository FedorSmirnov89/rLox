use anyhow::Result;

use crate::{
    domain::{grammar::Statement, scanning::TokenType},
    parser::Parser,
};

impl<'tokens> Parser<'tokens> {
    ///
    /// Reads out an expression; Checks that it is followed by a semicolon. Also advances the current
    ///
    pub(crate) fn statement(&mut self) -> Result<Statement> {
        let statement = if let TokenType::PRINT =
            self.current().expect("current pos is out of bounds").t_type
        {
            self.advance();
            let expr = self.expression()?;
            Statement::Print(expr)
        } else {
            let expr = self.expression()?;
            Statement::Expression(expr)
        };
        self.expect(&TokenType::Semicolon)?;
        self.advance();
        Ok(statement)
    }
}
