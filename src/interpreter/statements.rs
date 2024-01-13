use super::{error::InterpreterError, State};

mod declaration;
mod statement;
mod var_declaration;

pub(crate) trait InterpretedStatement {
    fn interpret_statement(&self, state: &mut State) -> Result<(), InterpreterError>;
}
