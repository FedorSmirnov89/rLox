use anyhow::Result;

use crate::{
    domain::{
        grammar::{Statement, StringLiteral},
        scanning::TokenType,
    },
    matches_t_type,
    parser::Parser,
};

impl<'tokens> Parser<'tokens> {
    ///
    /// Reads out an expression; Checks that it is followed by a semicolon. Also advances the current
    ///
    pub(crate) fn statement(&mut self) -> Result<Statement> {
        let statement = match self.current_statement()? {
            StatementType::Print => self.print_statement()?,
            StatementType::Expression => self.expression_statement()?,
            StatementType::Assignment => self.assignment_statement()?,
        };

        Ok(statement)
    }

    fn consume_semicolon(&mut self) -> Result<()> {
        self.expect(&TokenType::Semicolon, "semicolon after statement")?;
        self.advance();
        Ok(())
    }

    fn print_statement(&mut self) -> Result<Statement> {
        self.advance();
        let expr = self.expression()?;
        self.consume_semicolon()?;
        Ok(Statement::Print(expr))
    }

    fn assignment_statement(&mut self) -> Result<Statement> {
        let literal = StringLiteral::identifier_from_token(self.current()?)?;
        self.advance();
        self.advance();
        let expr = self.expression()?;
        self.consume_semicolon()?;
        Ok(Statement::Assignment(literal, expr))
    }

    fn expression_statement(&mut self) -> Result<Statement> {
        dbg!("on an expression statement");
        let expr = self.expression()?;
        if self.not_finished() {
            self.consume_semicolon()?;
            Ok(Statement::Expression(expr))
        } else {
            Ok(Statement::FinalExpression(expr))
        }
    }

    fn current_statement(&self) -> Result<StatementType> {
        if self.on_print_statement()? {
            Ok(StatementType::Print)
        } else if self.on_assignment_statement()? {
            Ok(StatementType::Assignment)
        } else {
            Ok(StatementType::Expression)
        }
    }

    fn on_assignment_statement(&self) -> Result<bool> {
        let current_t_type = self.current()?.t_type();
        let next_t_type = self.next()?.t_type();
        match (current_t_type, next_t_type) {
            (TokenType::Identifier(_), TokenType::Equal) => Ok(true),
            _ => Ok(false),
        }
    }

    fn on_print_statement(&self) -> Result<bool> {
        let current = self.current()?;
        Ok(matches_t_type!(current, &TokenType::PRINT))
    }
}

enum StatementType {
    Print,
    Assignment,
    Expression,
}
