use crate::{
    domain::grammar::Unary,
    interpreter::{error::InterpreterError, Outcome},
    operator_error, value, Environment, Value, ValueType,
};

use super::InterpretedExpression;

impl InterpretedExpression for Unary {
    fn interpret_expression(&self, state: &mut Environment) -> Result<Outcome, InterpreterError> {
        match self {
            Unary::Primary(p) => p.interpret_expression(state),
            Unary::LogicalNegation(u) => {
                let val = value!(u, state);
                match &val.v_type {
                    ValueType::Boolean(b) => {
                        let res_val =
                            Value::new(ValueType::Boolean(!b), val.span().extend_to_left(1));
                        Ok(Outcome::Value(res_val))
                    }
                    _ => {
                        operator_error!(val, "!");
                    }
                }
            }
            Unary::ArithmNegation(u) => {
                let val = value!(u, state);
                match &val.v_type {
                    ValueType::Number(n) => {
                        let res_val =
                            Value::new(ValueType::Number(-n), val.span().extend_to_left(1));
                        Ok(Outcome::Value(res_val))
                    }
                    _ => {
                        operator_error!(val, "-");
                    }
                }
            }
            Unary::Call(callee) => {
                let val = state
                    .call(callee)
                    .map_err(|call_err| InterpreterError::CallError(call_err))?;
                Ok(Outcome::Value(val))
            }
        }
    }
}
