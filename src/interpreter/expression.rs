use anyhow::Result;

use crate::domain::grammar::Expression;

use super::{error::InterpreterError, Interpretation, Value};

impl Interpretation for Expression {
    fn interpret(&self) -> Result<Value, InterpreterError> {
        match self {
            Expression::Equality(eq) => eq.interpret(),
        }
    }
}
