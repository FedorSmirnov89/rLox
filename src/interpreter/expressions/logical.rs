use crate::{
    domain::{
        grammar::{LogicAnd, LogicOr},
        location::CodeSpan,
    },
    interpreter::error::InterpreterError,
    operator_error, Environment, Value, ValueType,
};

use super::InterpretedExpression;

impl InterpretedExpression for LogicOr {
    fn interpret_expression(&self, env: &Environment) -> Result<Value, InterpreterError> {
        let (left, right) = match self {
            LogicOr::LogicAnd(l_and) => return l_and.interpret_expression(env),
            LogicOr::Or { left, right } => (left, right),
        };

        let left_val = left.interpret_expression(env)?;
        if let ValueType::Boolean(true) = left_val.v_type {
            return Ok(left_val);
        }
        let right_val = right.interpret_expression(env)?;

        match (&left_val.v_type, &right_val.v_type) {
            (ValueType::Boolean(l), ValueType::Boolean(r)) => Ok(Value::new(
                ValueType::Boolean(*l || *r),
                CodeSpan::merged(left_val.span(), right_val.span()),
            )),
            (_, _) => {
                operator_error!(left_val, right_val, "||");
            }
        }
    }
}

impl InterpretedExpression for LogicAnd {
    fn interpret_expression(&self, env: &Environment) -> Result<Value, InterpreterError> {
        let (left, right) = match self {
            LogicAnd::Equality(eq) => return eq.interpret_expression(env),
            LogicAnd::And { left, right } => (left, right),
        };

        let left_val = left.interpret_expression(env)?;
        if let ValueType::Boolean(false) = left_val.v_type {
            return Ok(left_val);
        }
        let right_val = right.interpret_expression(env)?;

        match (&left_val.v_type, &right_val.v_type) {
            (ValueType::Boolean(l), ValueType::Boolean(r)) => Ok(Value::new(
                ValueType::Boolean(*l && *r),
                CodeSpan::merged(left_val.span(), right_val.span()),
            )),
            (_, _) => {
                operator_error!(left_val, right_val, "&&");
            }
        }
    }
}
