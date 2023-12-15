use anyhow::Result;

use crate::{
    domain::grammar::{NumLiteral, Primary, StringLiteral},
    Value,
};

use super::{error::InterpreterError, Interpretation, ValueType};

impl Interpretation for Primary {
    fn interpret(&self) -> Result<Value, InterpreterError> {
        let val = match self {
            Primary::Number(NumLiteral { value, span }) => {
                Value::new(ValueType::Number(*value), *span)
            }
            Primary::String(StringLiteral { value, span }) => {
                Value::new(ValueType::string(value), *span)
            }
            Primary::True(span) => Value::new(ValueType::Boolean(true), *span),
            Primary::False(span) => Value::new(ValueType::Boolean(false), *span),
            Primary::Nil(span) => Value::new(ValueType::Nil, *span),
            Primary::GroupedExpression(expr) => expr.interpret()?,
            Primary::Identifier(_) => todo!(),
        };
        Ok(val)
    }
}
