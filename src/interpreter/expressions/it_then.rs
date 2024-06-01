use crate::{
    domain::grammar::IfThen,
    interpreter::{error::InterpreterError, expressions::InterpretedExpression},
    Environment,
};

use super::InterpretedStatement;

impl InterpretedStatement for IfThen {
    fn interpret_statement(&self, env: &mut Environment) -> Result<(), InterpreterError> {
        let condition_val = self.condition.interpret_expression(env)?;
        let condition_is_true = InterpreterError::unwrap_bool(condition_val, "if condition")?;
        if condition_is_true {
            self.then.interpret_statement(env)?;
        }
        Ok(())
    }
}
