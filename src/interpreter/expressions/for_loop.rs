use crate::{
    domain::grammar::control_flow::DesugeredFor, interpreter::error::InterpreterError, Environment,
};

use super::InterpretedExpression;

impl InterpretedExpression for DesugeredFor {
    fn interpret_expression(
        &self,
        env: &mut Environment,
    ) -> Result<crate::Value, InterpreterError> {
        self.for_block.interpret_expression(env)
    }
}
