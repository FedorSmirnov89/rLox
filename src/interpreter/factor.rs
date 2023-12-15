use crate::{
    domain::{grammar::Factor, location::CodeSpan},
    operator_error, Value,
};

use super::{error::InterpreterError, Interpretation, ValueType};

impl Interpretation for Factor {
    fn interpret(&self) -> Result<Value, InterpreterError> {
        match self {
            Factor::Unary(u) => u.interpret(),
            Factor::Multiplication { left, right } => {
                let left_val = left.interpret()?;
                let right_val = right.interpret()?;

                match (&left_val.v_type, &right_val.v_type) {
                    (ValueType::Number(l), ValueType::Number(r)) => Ok(Value::new(
                        ValueType::Number(l * r),
                        CodeSpan::merged(left_val.span, right_val.span),
                    )),
                    (_, _) => {
                        operator_error!(left_val, right_val, "*");
                    }
                }
            }
            Factor::Division { left, right } => {
                let left_val = left.interpret()?;
                let right_val = right.interpret()?;

                match (&left_val.v_type, &right_val.v_type) {
                    (ValueType::Number(l), ValueType::Number(r)) => Ok(Value::new(
                        ValueType::Number(l / r),
                        CodeSpan::merged(left_val.span, right_val.span),
                    )),
                    (_, _) => {
                        operator_error!(left_val, right_val, "/");
                    }
                }
            }
        }
    }
}
