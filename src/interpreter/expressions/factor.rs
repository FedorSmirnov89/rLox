use crate::{
    domain::{grammar::Factor, location::CodeSpan},
    interpreter::{error::InterpreterError, Outcome},
    operator_error, value, Environment, Value, ValueType,
};

use super::InterpretedExpression;

impl InterpretedExpression for Factor {
    fn interpret_expression(&self, state: &mut Environment) -> Result<Outcome, InterpreterError> {
        match self {
            Factor::Unary(u) => u.interpret_expression(state),
            Factor::Multiplication { left, right } => {
                let left_val = value!(left, state);
                let right_val = value!(right, state);

                match (&left_val.v_type, &right_val.v_type) {
                    (ValueType::Number(l), ValueType::Number(r)) => {
                        let val = Value::new(
                            ValueType::Number(l * r),
                            CodeSpan::merged(left_val.span(), right_val.span()),
                        );
                        Ok(Outcome::Value(val))
                    }
                    (_, _) => {
                        operator_error!(left_val, right_val, "*");
                    }
                }
            }
            Factor::Division { left, right } => {
                let left_val = value!(left, state);
                let right_val = value!(right, state);

                match (&left_val.v_type, &right_val.v_type) {
                    (ValueType::Number(l), ValueType::Number(r)) => {
                        let val = Value::new(
                            ValueType::Number(l / r),
                            CodeSpan::merged(left_val.span(), right_val.span()),
                        );
                        Ok(Outcome::Value(val))
                    }
                    (_, _) => {
                        operator_error!(left_val, right_val, "/");
                    }
                }
            }
        }
    }
}
