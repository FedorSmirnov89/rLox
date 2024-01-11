use crate::{
    domain::{
        grammar::{Comparison, Equality},
        location::CodeSpan,
    },
    operator_error, Value,
};

use super::{error::InterpreterError, InterpretatedExpression, ValueType};

impl InterpretatedExpression for Equality {
    fn interpret_expression(&self) -> Result<Value, InterpreterError> {
        match self {
            Equality::Comparison(c) => c.interpret_expression(),
            Equality::EqualityCheck { left, right } => operation(left, right, Operator::Equal),
            Equality::InequalityCheck { left, right } => operation(left, right, Operator::NotEqual),
        }
    }
}

enum Operator {
    Equal,
    NotEqual,
}

fn operation(
    left: &Box<Equality>,
    right: &Comparison,
    operator: Operator,
) -> Result<Value, InterpreterError> {
    let left_val = left.interpret_expression()?;
    let right_val = right.interpret_expression()?;

    let b = match (&left_val.v_type, &right_val.v_type) {
        (ValueType::Number(l), ValueType::Number(r)) => match operator {
            Operator::Equal => l == r,
            Operator::NotEqual => l != r,
        },
        (ValueType::Boolean(l), ValueType::Boolean(r)) => match operator {
            Operator::Equal => l == r,
            Operator::NotEqual => l != r,
        },
        (ValueType::String(l), ValueType::String(r)) => match operator {
            Operator::Equal => l == r,
            Operator::NotEqual => l != r,
        },
        (_, _) => {
            let oper_str = match operator {
                Operator::Equal => "==",
                Operator::NotEqual => "!=",
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
