use crate::Environment;

use super::{error::InterpreterError, Outcome};

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

#[macro_export]
macro_rules! value {
    ($outcome: ident, $state: ident) => {
        match $outcome.interpret_expression($state)? {
            Outcome::Value(val) => val,
            early_return @ Outcome::Return(_) => return Ok(early_return),
            void_outcome @ Outcome::Void => return Ok(void_outcome),
        }
    };
}

pub(crate) trait InterpretedExpression {
    fn interpret_expression(&self, env: &mut Environment) -> Result<Outcome, InterpreterError>;
}
