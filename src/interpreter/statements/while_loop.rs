use crate::{
    domain::grammar::While,
    interpreter::{error::InterpreterError, expressions::InterpretedExpression},
    Environment,
};

use super::InterpretedStatement;

impl InterpretedStatement for While {
    fn interpret_statement(&self, env: &mut Environment) -> Result<(), InterpreterError> {
        let mut cond_bool = self.get_cond_bool(env)?;
        let while_block = &self.block;

        while cond_bool {
            while_block.interpret_statement(env)?;
            cond_bool = self.get_cond_bool(env)?;
        }
        Ok(())
    }
}

impl While {
    fn get_cond_bool(&self, env: &Environment) -> Result<bool, InterpreterError> {
        let condition = &self.condition;
        let condition_val = condition.interpret_expression(env)?;
        let cond_bool = InterpreterError::unwrap_bool(condition_val, "while condition")?;
        Ok(cond_bool)
    }
}
