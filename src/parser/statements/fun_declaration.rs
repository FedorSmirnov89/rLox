use crate::{
    domain::grammar::FunDeclaration,
    parser::{errors::ParserError, Parser},
};

impl<'tokens> Parser<'tokens> {
    pub(crate) fn fun_declaration(&mut self) -> Result<FunDeclaration, ParserError> {
        self.advance(); // consume the fun token

        let current = self.current()?;
        let name = self.parse_identifier()?.ok_or_else(|| {
            ParserError::other_err("failed to parse function identifier", current.location())
        })?;
        let current = self.current()?;
        let arguments = self.parse_parameters()?.ok_or_else(|| {
            ParserError::other_err("failed to parse function arguments", current.location())
        })?;
        let body = self.block()?;
        let fun_declaration = FunDeclaration {
            name: name,
            arguments,
            body,
        };
        Ok(fun_declaration)
    }
}
