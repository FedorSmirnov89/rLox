mod display;

use std::ops::Deref;

mod expressions;
mod statements;

pub(crate) use expressions::*;
pub(crate) use statements::*;

#[derive(Debug)]
pub(crate) struct Program(pub(crate) Vec<Declaration>);

impl Deref for Program {
    type Target = [Declaration];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
