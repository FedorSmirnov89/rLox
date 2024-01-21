use anyhow::Result;

use crate::{
    domain::grammar::Expression, interpreter::error::InterpreterError, Environment, Value,
};

use super::InterpretedExpression;

impl InterpretedExpression for Expression {
    fn interpret_expression(&self, state: &Environment) -> Result<Value, InterpreterError> {
        match self {
            Expression::LogicOr(l_or) => l_or.interpret_expression(state),
        }
    }
}
