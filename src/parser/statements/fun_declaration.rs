use anyhow::{anyhow, Result};

use crate::{domain::grammar::FunDeclaration, parser::Parser};

impl<'tokens> Parser<'tokens> {
    pub(crate) fn fun_declaration(&mut self) -> Result<FunDeclaration> {
        self.advance(); // consume the fun token

        let name = self
            .parse_identifier()?
            .ok_or_else(|| anyhow!("failed to read function identifier"))?;
        let arguments = self
            .parse_parameters()?
            .ok_or_else(|| anyhow!("failed to read function arguments"))?;
        let body = self.block()?;
        let fun_declaration = FunDeclaration {
            name: name,
            arguments,
            body,
        };
        Ok(fun_declaration)
    }
}
