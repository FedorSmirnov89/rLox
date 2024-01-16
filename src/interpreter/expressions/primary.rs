use anyhow::Result;

use crate::{
    domain::grammar::{NumLiteral, Primary, StringLiteral},
    interpreter::error::InterpreterError,
    Environment, Value, ValueType,
};

use super::InterpretedExpression;

impl InterpretedExpression for Primary {
    fn interpret_expression(&self, state: &Environment) -> Result<Value, InterpreterError> {
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
            Primary::GroupedExpression(expr) => expr.interpret_expression(state)?,
            Primary::Identifier(iden) => match state.get_var_value(iden.as_ref()) {
                Some(v) => v.clone(),
                None => {
                    return Err(InterpreterError::identifier_not_defined(
                        iden.as_ref(),
                        iden.span,
                    ))
                }
            },
        };
        Ok(val)
    }
}
