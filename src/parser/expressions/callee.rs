//! Module specifying how callees are parsed

use anyhow::Result;

use crate::{
    domain::grammar::{Callee, CalleeIdentifier},
    parser::{errors::ParserError, Parser},
};

impl<'tokens> Parser<'tokens> {
    ///
    /// Tries to parse a callee starting from the current parser position
    ///
    /// If successful, the callee is returned and the parser position is advanced to the next token after the callee
    ///
    /// If unsuccessful, the parser position is not advanced by this operation
    ///
    pub(super) fn callee(&mut self) -> Result<Option<Callee>, ParserError> {
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

    fn parse_callee(&mut self) -> Result<Option<Callee>, ParserError> {
        let identifier = match self.parse_callee_identifier()? {
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

    fn parse_callee_identifier(&mut self) -> Result<Option<CalleeIdentifier>, ParserError> {
        Ok(self.parse_identifier()?.map(|iden_str| iden_str.into()))
    }
}
