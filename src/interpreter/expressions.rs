use crate::{Environment, Value};

use super::error::InterpreterError;

mod block;
mod comparison;
mod equality;
mod expression;
mod factor;
mod for_loop;
mod if_then;
mod if_then_else;
mod logical;
mod primary;
mod term;
mod unary;
mod while_loop;

pub(crate) trait InterpretedExpression {
    fn interpret_expression(&self, env: &mut Environment) -> Result<Value, InterpreterError>;
}
