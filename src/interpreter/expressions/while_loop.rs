use crate::{
    domain::grammar::control_flow::While,
    interpreter::{error::InterpreterError, expressions::InterpretedExpression, Outcome},
    Environment,
};

impl InterpretedExpression for While {
    fn interpret_expression(&self, env: &mut Environment) -> Result<Outcome, InterpreterError> {
        let mut cond_bool = self.get_cond_bool(env)?;
        let while_block = &self.block;

        let mut return_value = Outcome::Void;
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
        // again, in this case I feel like the while condition exression should
        // not be returning anything other than a boolean
        let condition_val = match condition.interpret_expression(env)? {
            Outcome::Value(val) => val,
            Outcome::Return(_) | Outcome::Void => {
                return Err(InterpreterError::OtherError(
                    "while condition does not resolve to bool".to_string(),
                ))?
            }
        };
        let cond_bool = InterpreterError::unwrap_bool(condition_val, "while condition")?;
        Ok(cond_bool)
    }
}
