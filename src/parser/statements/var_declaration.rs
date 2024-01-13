use anyhow::Result;

use crate::{domain::grammar::VarDeclaration, parser::Parser};

impl<'tokens> Parser<'tokens> {
    pub(crate) fn var_declaration(&mut self) -> Result<VarDeclaration> {
        todo!()
    }
}
