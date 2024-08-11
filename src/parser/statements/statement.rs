use anyhow::Result;

use crate::{
    domain::{grammar::Statement, scanning::TokenType},
    matches_t_type,
    parser::{errors::ParserError, Parser},
};

impl<'tokens> Parser<'tokens> {
    ///
    /// Reads out an expression; Checks that it is followed by a semicolon. Also advances the current
    ///
    pub(crate) fn statement(&mut self) -> Result<Statement, ParserError> {
        let statement = match self.current_statement()? {
            StatementType::Print => self.print_statement()?,
            StatementType::Expression => self.expression_statement()?,
            StatementType::Assignment => self.assignment_statement()?,
            StatementType::Return => self.return_statement()?,
        };

        Ok(statement)
    }

    fn return_statement(&mut self) -> Result<Statement, ParserError> {
        self.advance(); // consume the return
        if self.on_semicolon() {
            self.advance(); // consume the semicolon
            Ok(Statement::ReturnEmpty)
        } else {
            let expr = self.expression()?;
            self.consume_semicolon()?;
            Ok(Statement::Return(expr))
        }
    }

    fn on_return_statement(&self) -> Result<bool, ParserError> {
        let current = self.current()?;
        Ok(matches_t_type!(current, &TokenType::RETURN))
    }

    fn consume_semicolon(&mut self) -> Result<(), ParserError> {
        self.expect(&TokenType::Semicolon, "semicolon after statement")?;
        self.advance();
        Ok(())
    }

    fn print_statement(&mut self) -> Result<Statement, ParserError> {
        self.advance();
        let expr = self.expression()?;
        self.consume_semicolon()?;
        Ok(Statement::Print(expr))
    }

    fn assignment_statement(&mut self) -> Result<Statement, ParserError> {
        let literal = Self::identifier_from_token(self.current()?)?;
        self.advance();
        self.advance();
        let expr = self.expression()?;
        self.consume_semicolon()?;
        Ok(Statement::Assignment(literal, expr))
    }

    fn expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expr = self.expression()?;
        if self.not_finished() && self.not_on_closing_of_block() {
            self.consume_semicolon()?;
            Ok(Statement::Expression(expr))
        } else {
            Ok(Statement::FinalExpression(expr))
        }
    }

    fn current_statement(&self) -> Result<StatementType, ParserError> {
        if self.on_return_statement()? {
            Ok(StatementType::Return)
        } else if self.on_print_statement()? {
            Ok(StatementType::Print)
        } else if self.on_assignment_statement()? {
            Ok(StatementType::Assignment)
        } else {
            Ok(StatementType::Expression)
        }
    }

    fn on_assignment_statement(&self) -> Result<bool, ParserError> {
        let current_t_type = self.current()?.t_type();
        let next_t_type = self.next()?.t_type();
        match (current_t_type, next_t_type) {
            (TokenType::Identifier(_), TokenType::Equal) => Ok(true),
            _ => Ok(false),
        }
    }

    fn on_print_statement(&self) -> Result<bool, ParserError> {
        let current = self.current()?;
        Ok(matches_t_type!(current, &TokenType::PRINT))
    }
}

enum StatementType {
    Print,
    Assignment,
    Expression,
    Return,
}
