use super::{error::InterpreterError, Environment, Outcome};

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
