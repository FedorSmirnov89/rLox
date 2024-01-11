use anyhow::Result;

use crate::domain::grammar::Expression;

use super::{error::InterpreterError, InterpretatedExpression, Value};

impl InterpretatedExpression for Expression {
    fn interpret_expression(&self) -> Result<Value, InterpreterError> {
        match self {
            Expression::Equality(eq) => eq.interpret_expression(),
        }
    }
}
