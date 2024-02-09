use crate::{
    domain::grammar::IfThenElse,
    interpreter::{error::InterpreterError, expressions::InterpretedExpression},
    Environment,
};

use super::InterpretedStatement;

impl InterpretedStatement for IfThenElse {
    fn interpret_statement(&self, env: &mut Environment) -> Result<(), InterpreterError> {
        let condition_val = self.if_then.condition.interpret_expression(env)?;
        let condition = InterpreterError::unwrap_bool(condition_val, "if then block")?;
        let then = &self.if_then.then;
        let else_bloc = &self.else_block;
        if condition {
            then.interpret_statement(env)?;
        } else {
            else_bloc.interpret_statement(env)?;
        }
        Ok(())
    }
}
