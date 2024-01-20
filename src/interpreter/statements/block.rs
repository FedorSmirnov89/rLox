use crate::{domain::grammar::Block, interpreter::error::InterpreterError, Environment};

use super::InterpretedStatement;

impl InterpretedStatement for Block {
    fn interpret_statement(&self, environment: &mut Environment) -> Result<(), InterpreterError> {
        environment.new_inner_scope();
        let inner_result = self.interpret_statements_in_inner_scope(environment);
        environment.teardown_inner_scope();
        inner_result
    }
}

impl Block {
    fn interpret_statements_in_inner_scope(
        &self,
        env: &mut Environment,
    ) -> Result<(), InterpreterError> {
        for decl in self.as_ref() {
            decl.interpret_statement(env)?;
        }
        Ok(())
    }
}
