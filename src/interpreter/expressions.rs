use crate::{Environment, Value};

use super::error::InterpreterError;

mod comparison;
mod equality;
mod expression;
mod factor;
mod primary;
mod term;
mod unary;

pub(crate) trait InterpretedExpression {
    fn interpret_expression(&self, env: &Environment) -> Result<Value, InterpreterError>;
}
