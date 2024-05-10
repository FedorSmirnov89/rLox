//! Module specifying how callees are parsed

use anyhow::Result;

use crate::{
    domain::{
        grammar::{Callee, CalleeIdentifier, Expression},
        scanning::TokenType,
    },
    parser::Parser,
};

impl<'tokens> Parser<'tokens> {
    ///
    /// Tries to parse a callee starting from the current parser position
    ///
    /// If successful, the callee is returned and the parser position is advanced to the next token after the callee
    ///
    /// If unsuccessful, the parser position is not advanced by this operation
    ///
    pub(super) fn callee(&mut self) -> Result<Option<Callee>> {
        let position = self.cur_pos;
        match self.parse_callee() {
            Ok(Some(callee)) => Ok(Some(callee)),
            Ok(None) => {
                self.cur_pos = position;
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    fn parse_callee(&mut self) -> Result<Option<Callee>> {
        let identifier = match self.parse_identifier()? {
            Some(iden) => iden,
            None => return Ok(None),
        };
        let arguments = match self.parse_arguments()? {
            Some(args) => args,
            None => return Ok(None),
        };
        let callee = Callee::new(identifier, arguments);
        Ok(Some(callee))
    }

    fn parse_identifier(&mut self) -> Result<Option<CalleeIdentifier>> {
        let t_type = &self.current()?.t_type;
        dbg!(t_type);

        match self.current()?.t_type {
            TokenType::Identifier(ref s) => {
                self.advance();
                Ok(Some(s.clone().into()))
            }
            _ => Ok(None),
        }
    }

    fn parse_arguments(&mut self) -> Result<Option<Vec<Expression>>> {
        if let TokenType::ParenLeft = self.current()?.t_type {
            self.advance();
        } else {
            return Ok(None);
        };

        let mut expressions = vec![];

        // case with no arguments
        if self.current()?.t_type == TokenType::ParenRight {
            self.advance();
        } else {
            loop {
                let expr = self.expression()?;
                expressions.push(expr);
                match self.current()?.t_type {
                    TokenType::ParenRight => {
                        self.advance();
                        break;
                    }
                    TokenType::Comma => self.advance(),
                    _ => return Ok(None), // incorrect arg list
                }
            }
        }
        Ok(Some(expressions))
    }
}
