use crate::{domain::grammar::Unary, operator_error, Value};

use super::{error::InterpreterError, Interpretation, ValueType};

impl Interpretation for Unary {
    fn interpret(&self) -> Result<Value, InterpreterError> {
        match self {
            Unary::Primary(p) => p.interpret(),
            Unary::LogicalNegation(u) => {
                let val = u.interpret()?;
                match &val.v_type {
                    ValueType::Boolean(b) => Ok(Value::new(
                        ValueType::Boolean(!b),
                        val.span.extend_to_left(1),
                    )),
                    _ => {
                        operator_error!(val, "!");
                    }
                }
            }
            Unary::ArithmNegation(u) => {
                let val = u.interpret()?;
                match &val.v_type {
                    ValueType::Number(n) => Ok(Value::new(
                        ValueType::Number(-n),
                        val.span.extend_to_left(1),
                    )),
                    _ => {
                        operator_error!(val, "-");
                    }
                }
            }
        }
    }
}
