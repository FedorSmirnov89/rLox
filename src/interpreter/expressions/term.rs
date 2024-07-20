use crate::{
    domain::{grammar::Term, location::CodeSpan},
    interpreter::{error::InterpreterError, Outcome},
    operator_error, value, Environment, Value, ValueType,
};

use super::InterpretedExpression;

impl InterpretedExpression for Term {
    fn interpret_expression(&self, state: &mut Environment) -> Result<Outcome, InterpreterError> {
        match self {
            Term::Factor(f) => f.interpret_expression(state),
            Term::Addition { left, right } => {
                let l_val = value!(left, state);
                let r_val = value!(right, state);

                let res_val = match (&l_val.v_type, &r_val.v_type) {
                    (ValueType::Number(l), ValueType::Number(r)) => Value::new(
                        ValueType::Number(l + r),
                        CodeSpan::merged(l_val.span(), r_val.span()),
                    ),
                    (ValueType::String(l), ValueType::String(r)) => Value::new(
                        ValueType::String(format!("{}{}", l, r)),
                        CodeSpan::merged(l_val.span(), r_val.span()),
                    ),
                    (_, _) => {
                        operator_error!(l_val, r_val, "+")
                    }
                };
                Ok(Outcome::Value(res_val))
            }
            Term::Subtraction { left, right } => {
                let left_val = value!(left, state);
                let right_val = value!(right, state);

                let res_val = match (&left_val.v_type, &right_val.v_type) {
                    (ValueType::Number(l), ValueType::Number(r)) => Value::new(
                        ValueType::Number(l - r),
                        CodeSpan::merged(left_val.span(), right_val.span()),
                    ),
                    (_, _) => {
                        operator_error!(left_val, right_val, "-")
                    }
                };
                Ok(Outcome::Value(res_val))
            }
        }
    }
}
