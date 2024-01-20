use crate::{
    domain::grammar::Unary, interpreter::error::InterpreterError, operator_error, Environment,
    Value, ValueType,
};

use super::InterpretedExpression;

impl InterpretedExpression for Unary {
    fn interpret_expression(&self, state: &Environment) -> Result<Value, InterpreterError> {
        match self {
            Unary::Primary(p) => p.interpret_expression(state),
            Unary::LogicalNegation(u) => {
                let val = u.interpret_expression(state)?;
                match &val.v_type {
                    ValueType::Boolean(b) => Ok(Value::new(
                        ValueType::Boolean(!b),
                        val.span().extend_to_left(1),
                    )),
                    _ => {
                        operator_error!(val, "!");
                    }
                }
            }
            Unary::ArithmNegation(u) => {
                let val = u.interpret_expression(state)?;
                match &val.v_type {
                    ValueType::Number(n) => Ok(Value::new(
                        ValueType::Number(-n),
                        val.span().extend_to_left(1),
                    )),
                    _ => {
                        operator_error!(val, "-");
                    }
                }
            }
        }
    }
}
