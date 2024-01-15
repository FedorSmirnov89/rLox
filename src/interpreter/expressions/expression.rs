use anyhow::Result;

use crate::{domain::grammar::Expression, interpreter::error::InterpreterError, State, Value};

use super::InterpretedExpression;

impl InterpretedExpression for Expression {
    fn interpret_expression(&self, state: &State) -> Result<Value, InterpreterError> {
        match self {
            Expression::Equality(eq) => eq.interpret_expression(state),
        }
    }
}
