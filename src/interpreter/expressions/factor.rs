use crate::{
    domain::{grammar::Factor, location::CodeSpan},
    interpreter::error::InterpreterError,
    operator_error, Environment, Value, ValueType,
};

use super::InterpretedExpression;

impl InterpretedExpression for Factor {
    fn interpret_expression(&self, state: &Environment) -> Result<Value, InterpreterError> {
        match self {
            Factor::Unary(u) => u.interpret_expression(state),
            Factor::Multiplication { left, right } => {
                let left_val = left.interpret_expression(state)?;
                let right_val = right.interpret_expression(state)?;

                match (&left_val.v_type, &right_val.v_type) {
                    (ValueType::Number(l), ValueType::Number(r)) => Ok(Value::new(
                        ValueType::Number(l * r),
                        CodeSpan::merged(left_val.span(), right_val.span()),
                    )),
                    (_, _) => {
                        operator_error!(left_val, right_val, "*");
                    }
                }
            }
            Factor::Division { left, right } => {
                let left_val = left.interpret_expression(state)?;
                let right_val = right.interpret_expression(state)?;

                match (&left_val.v_type, &right_val.v_type) {
                    (ValueType::Number(l), ValueType::Number(r)) => Ok(Value::new(
                        ValueType::Number(l / r),
                        CodeSpan::merged(left_val.span(), right_val.span()),
                    )),
                    (_, _) => {
                        operator_error!(left_val, right_val, "/");
                    }
                }
            }
        }
    }
}
