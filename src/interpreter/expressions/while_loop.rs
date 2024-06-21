use crate::{
    domain::grammar::control_flow::While,
    interpreter::{error::InterpreterError, expressions::InterpretedExpression},
    Environment,
};

impl InterpretedExpression for While {
    fn interpret_expression(
        &self,
        env: &mut Environment,
    ) -> Result<crate::Value, InterpreterError> {
        let mut cond_bool = self.get_cond_bool(env)?;
        let while_block = &self.block;

        let mut return_value = crate::Value::nil();
        while cond_bool {
            return_value = while_block.interpret_expression(env)?;
            cond_bool = self.get_cond_bool(env)?;
        }
        Ok(return_value)
    }
}

impl While {
    fn get_cond_bool(&self, env: &mut Environment) -> Result<bool, InterpreterError> {
        let condition = &self.condition;
        let condition_val = condition.interpret_expression(env)?;
        let cond_bool = InterpreterError::unwrap_bool(condition_val, "while condition")?;
        Ok(cond_bool)
    }
}
