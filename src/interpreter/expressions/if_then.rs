use crate::{
    domain::grammar::control_flow::IfThen,
    interpreter::{error::InterpreterError, expressions::InterpretedExpression},
    Environment, Value,
};

impl InterpretedExpression for IfThen {
    fn interpret_expression(
        &self,
        env: &mut Environment,
    ) -> Result<crate::Value, InterpreterError> {
        let condition_val = self.condition.interpret_expression(env)?;
        let condition_is_true = InterpreterError::unwrap_bool(condition_val, "if condition")?;
        let value = if condition_is_true {
            self.then.interpret_expression(env)?
        } else {
            Value::nil()
        };
        Ok(value)
    }
}
