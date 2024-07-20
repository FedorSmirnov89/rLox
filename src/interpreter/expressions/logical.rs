use crate::{
    domain::{
        grammar::{LogicAnd, LogicOr},
        location::CodeSpan,
    },
    interpreter::{error::InterpreterError, Outcome},
    operator_error, value, Environment, Value, ValueType,
};

use super::InterpretedExpression;

impl InterpretedExpression for LogicOr {
    fn interpret_expression(&self, env: &mut Environment) -> Result<Outcome, InterpreterError> {
        let (left, right) = match self {
            LogicOr::LogicAnd(l_and) => return l_and.interpret_expression(env),
            LogicOr::Or { left, right } => (left, right),
        };

        let left_val = value!(left, env);
        if let ValueType::Boolean(true) = left_val.v_type {
            return Ok(Outcome::Value(left_val));
        }
        let right_val = value!(right, env);

        match (&left_val.v_type, &right_val.v_type) {
            (ValueType::Boolean(l), ValueType::Boolean(r)) => {
                let val = Value::new(
                    ValueType::Boolean(*l || *r),
                    CodeSpan::merged(left_val.span(), right_val.span()),
                );
                Ok(Outcome::Value(val))
            }
            (_, _) => {
                operator_error!(left_val, right_val, "||");
            }
        }
    }
}

impl InterpretedExpression for LogicAnd {
    fn interpret_expression(&self, env: &mut Environment) -> Result<Outcome, InterpreterError> {
        let (left, right) = match self {
            LogicAnd::Equality(eq) => return eq.interpret_expression(env),
            LogicAnd::And { left, right } => (left, right),
        };

        let left_val = value!(left, env);
        if let ValueType::Boolean(false) = left_val.v_type {
            return Ok(Outcome::Value(left_val));
        }
        let right_val = value!(right, env);

        match (&left_val.v_type, &right_val.v_type) {
            (ValueType::Boolean(l), ValueType::Boolean(r)) => {
                let val = Value::new(
                    ValueType::Boolean(*l && *r),
                    CodeSpan::merged(left_val.span(), right_val.span()),
                );
                Ok(Outcome::Value(val))
            }
            (_, _) => {
                operator_error!(left_val, right_val, "&&");
            }
        }
    }
}
