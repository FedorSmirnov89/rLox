use crate::{domain::grammar::Block, interpreter::error::InterpreterError, Environment};

use super::InterpretedStatement;

impl InterpretedStatement for Block {
    fn interpret_statement(&self, environment: &mut Environment) -> Result<(), InterpreterError> {
        environment.new_inner_scope();
        for decl in self.as_ref() {
            decl.interpret_statement(environment)?;
        }
        environment.teardown_inner_scope();
        Ok(())
    }
}
