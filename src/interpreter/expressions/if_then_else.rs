use crate::{
    domain::grammar::control_flow::IfThenElse,
    interpreter::{error::InterpreterError, expressions::InterpretedExpression},
    Environment,
};

impl InterpretedExpression for IfThenElse {
    fn interpret_expression(
        &self,
        env: &mut Environment,
    ) -> Result<crate::Value, InterpreterError> {
        let condition_val = self.if_then.condition.interpret_expression(env)?;
        let condition = InterpreterError::unwrap_bool(condition_val, "if then block")?;
        let then = &self.if_then.then;
        let else_bloc = &self.else_block;
        if condition {
            then.interpret_expression(env)
        } else {
            else_bloc.interpret_expression(env)
        }
    }
}
