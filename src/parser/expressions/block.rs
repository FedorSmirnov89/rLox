use anyhow::Result;

use crate::{
    domain::{
        grammar::{Block, Declaration, Statement},
        scanning::TokenType,
    },
    parser::{errors::ParserError, Parser},
};

impl<'tokens> Parser<'tokens> {
    pub(crate) fn at_start_of_block(&self) -> Result<bool, ParserError> {
        Ok(self.current()?.t_type == TokenType::BraceLeft)
    }

    pub(crate) fn at_end_of_block(&self) -> Result<bool, ParserError> {
        Ok(self.current()?.t_type == TokenType::BraceRight)
    }

    pub(crate) fn block(&mut self) -> Result<Block, ParserError> {
        self.read_block_content()
    }

    pub(crate) fn read_block_content(&mut self) -> Result<Block, ParserError> {
        self.advance(); // go past opening brace
        let mut statements = vec![];
        let mut final_expression = None;
        while !self.at_end_of_block()? {
            let statement = self.declaration()?;
            if let Declaration::Statement(Statement::FinalExpression(expr)) = statement {
                final_expression = Some(expr);
                break;
            } else {
                statements.push(statement);
            }
        }
        self.advance(); // go past closing brace
        Ok(Block {
            statements,
            final_expression,
        })
        // Ok(statements.into())
    }
}
