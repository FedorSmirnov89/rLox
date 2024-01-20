use super::{error::InterpreterError, Environment};

mod block;
mod declaration;
mod statement;
mod var_declaration;

pub(crate) trait InterpretedStatement {
    fn interpret_statement(&self, environment: &mut Environment) -> Result<(), InterpreterError>;
}
