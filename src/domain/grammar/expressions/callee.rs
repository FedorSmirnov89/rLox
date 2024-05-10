//! Module defining the `Callee` enum, which represents the different types of callees that can be called with a possibly empty list of arguments

use std::fmt::Display;

use super::Expression;

#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) struct Callee {
    identifier: CalleeIdentifier,
    arguments: Vec<Expression>,
}

impl Callee {
    pub(crate) fn new(identifier: CalleeIdentifier, arguments: Vec<Expression>) -> Self {
        Callee {
            identifier,
            arguments,
        }
    }
}

impl Display for Callee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{iden} (", iden = self.identifier)?;
        for (i, arg) in self.arguments.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{arg}", arg = arg)?;
        }
        write!(f, ")")
    }
}

///
/// Models different ways of identifying a callee
///
#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) enum CalleeIdentifier {
    Identifier(String),
}

impl From<String> for CalleeIdentifier {
    fn from(value: String) -> Self {
        CalleeIdentifier::Identifier(value)
    }
}

impl Display for CalleeIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalleeIdentifier::Identifier(iden_str) => write!(f, "{iden_str}"),
        }
    }
}
