use crate::{
    domain::{grammar::Term, location::CodeSpan},
    interpreter::error::InterpreterError,
    operator_error, Environment, Value, ValueType,
};

use super::InterpretedExpression;

impl InterpretedExpression for Term {
    fn interpret_expression(&self, state: &Environment) -> Result<Value, InterpreterError> {
        match self {
            Term::Factor(f) => f.interpret_expression(state),
            Term::Addition { left, right } => {
                let l_val = left.interpret_expression(state)?;
                let r_val = right.interpret_expression(state)?;

                match (&l_val.v_type, &r_val.v_type) {
                    (ValueType::Number(l), ValueType::Number(r)) => Ok(Value::new(
                        ValueType::Number(l + r),
                        CodeSpan::merged(l_val.span(), r_val.span()),
                    )),
                    (ValueType::String(l), ValueType::String(r)) => Ok(Value::new(
                        ValueType::String(format!("{}{}", l, r)),
                        CodeSpan::merged(l_val.span(), r_val.span()),
                    )),
                    (_, _) => {
                        operator_error!(l_val, r_val, "+");
                    }
                }
            }
            Term::Subtraction { left, right } => {
                let left_val = left.interpret_expression(state)?;
                let right_val = right.interpret_expression(state)?;

                match (&left_val.v_type, &right_val.v_type) {
                    (ValueType::Number(l), ValueType::Number(r)) => Ok(Value::new(
                        ValueType::Number(l - r),
                        CodeSpan::merged(left_val.span(), right_val.span()),
                    )),
                    (_, _) => {
                        operator_error!(left_val, right_val, "-");
                    }
                }
            }
        }
    }
}
