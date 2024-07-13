use crate::Value;

use super::{error::InterpreterError, Environment};

mod declaration;
mod fun_declaration;
mod statement;
mod var_declaration;

pub(crate) trait InterpretedStatement {
    fn interpret_statement(
        &self,
        environment: &mut Environment,
    ) -> Result<Outcome, InterpreterError>;
}

#[must_use]
pub(crate) enum Outcome {
    EarlyReturn(Value),
    Void,
}
