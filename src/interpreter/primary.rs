use anyhow::Result;

use crate::{
    domain::grammar::{NumLiteral, Primary, StringLiteral},
    Value,
};

use super::{error::InterpreterError, InterpretatedExpression, ValueType};

impl InterpretatedExpression for Primary {
    fn interpret_expression(&self) -> Result<Value, InterpreterError> {
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
            Primary::GroupedExpression(expr) => expr.interpret_expression()?,
            Primary::Identifier(_) => todo!(),
        };
        Ok(val)
    }
}
