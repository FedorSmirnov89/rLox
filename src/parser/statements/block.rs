use anyhow::Result;

use crate::{
    domain::{
        grammar::{Block, Declaration},
        scanning::TokenType,
    },
    parser::Parser,
};

impl<'tokens> Parser<'tokens> {
    pub(crate) fn at_start_of_block(&self) -> Result<bool> {
        Ok(self.current()?.t_type == TokenType::BraceLeft)
    }

    pub(crate) fn at_end_of_block(&self) -> Result<bool> {
        Ok(self.current()?.t_type == TokenType::BraceRight)
    }

    pub(crate) fn block(&mut self) -> Result<Declaration> {
        let block = self.read_block_content()?;
        Ok(Declaration::Block(block))
    }

    pub(crate) fn read_block_content(&mut self) -> Result<Block> {
        self.advance(); // go past opening brace
        let mut statements = vec![];
        while !self.at_end_of_block()? {
            statements.push(self.declaration()?);
        }
        self.advance(); // go past closing brace
        Ok(statements.into())
    }
}
