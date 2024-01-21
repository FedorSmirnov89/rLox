use super::{error::InterpreterError, Environment};

mod block;
mod declaration;
mod for_loop;
mod if_then_else;
mod it_then;
mod statement;
mod var_declaration;
mod while_loop;

pub(crate) trait InterpretedStatement {
    fn interpret_statement(&self, environment: &mut Environment) -> Result<(), InterpreterError>;
}
