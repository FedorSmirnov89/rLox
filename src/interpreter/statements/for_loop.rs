use crate::{domain::grammar::DesugeredFor, interpreter::error::InterpreterError, Environment};

use super::InterpretedStatement;

impl InterpretedStatement for DesugeredFor {
    fn interpret_statement(&self, env: &mut Environment) -> Result<(), InterpreterError> {
        self.for_block.interpret_statement(env)?;
        Ok(())
    }
}
