use crate::{
    domain::grammar::control_flow::IfThen,
    interpreter::{error::InterpreterError, expressions::InterpretedExpression, Outcome},
    value, Environment,
};

impl InterpretedExpression for IfThen {
    fn interpret_expression(&self, env: &mut Environment) -> Result<Outcome, InterpreterError> {
        let condition_expr = &self.condition;
        let condition_val = value!(condition_expr, env);
        let condition_is_true = InterpreterError::unwrap_bool(condition_val, "if condition")?;
        if condition_is_true {
            self.then.interpret_expression(env)
        } else {
            Ok(Outcome::Void)
        }
    }
}
