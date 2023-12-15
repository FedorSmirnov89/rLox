use crate::{
    domain::{
        grammar::{Comparison, Term},
        location::CodeSpan,
    },
    operator_error, Value,
};

use super::{error::InterpreterError, Interpretation, ValueType};

impl Interpretation for Comparison {
    fn interpret(&self) -> Result<Value, InterpreterError> {
        match self {
            Comparison::Term(t) => t.interpret(),
            Comparison::Greater { left, right } => comparison(left, right, Operator::Greater),
            Comparison::GreaterEqual { left, right } => {
                comparison(left, right, Operator::GreaterEqual)
            }
            Comparison::Less { left, right } => comparison(left, right, Operator::Less),
            Comparison::LessEqual { left, right } => comparison(left, right, Operator::LessEqual),
        }
    }
}

enum Operator {
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

fn comparison(
    left: &Box<Comparison>,
    right: &Term,
    operator: Operator,
) -> Result<Value, InterpreterError> {
    let left_val = left.interpret()?;
    let right_val = right.interpret()?;

    let b = match (&left_val.v_type, &right_val.v_type) {
        (ValueType::Number(l), ValueType::Number(r)) => match operator {
            Operator::Greater => l > r,
            Operator::GreaterEqual => l >= r,
            Operator::Less => l < r,
            Operator::LessEqual => l <= r,
        },
        (_, _) => {
            let oper_str = match operator {
                Operator::Greater => ">",
                Operator::GreaterEqual => ">=",
                Operator::Less => "<",
                Operator::LessEqual => "<=",
            };
            operator_error!(left_val, right_val, oper_str);
        }
    };
    let value = Value::new(
        ValueType::Boolean(b),
        CodeSpan::merged(left_val.span, right_val.span),
    );
    Ok(value)
}
