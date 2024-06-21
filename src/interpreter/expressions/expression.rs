use anyhow::Result;

use crate::{
    domain::grammar::Expression, interpreter::error::InterpreterError, Environment, Value,
};

use super::InterpretedExpression;

impl InterpretedExpression for Expression {
    fn interpret_expression(&self, env: &mut Environment) -> Result<Value, InterpreterError> {
        match self {
            Expression::LogicOr(l_or) => l_or.interpret_expression(env),
            Expression::IfThen(if_then) => if_then.interpret_expression(env),
            Expression::IfThenElse(if_then_else) => if_then_else.interpret_expression(env),
            Expression::While(while_loop) => while_loop.interpret_expression(env),
            Expression::For(desugered_for) => desugered_for.interpret_expression(env),
            Expression::Block(block) => block.interpret_expression(env),
        }
    }
}
