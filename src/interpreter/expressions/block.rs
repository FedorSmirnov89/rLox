use crate::{
    domain::grammar::Block,
    interpreter::{error::InterpreterError, statements::InterpretedStatement},
    Environment, Value,
};

use super::InterpretedExpression;

impl InterpretedExpression for Block {
    fn interpret_expression(
        &self,
        env: &mut Environment,
    ) -> Result<crate::Value, InterpreterError> {
        env.new_inner_scope();
        let inner_result = self.interpret_statements_in_inner_scope(env);
        env.teardown_inner_scope();
        inner_result
    }
}

impl Block {
    fn interpret_statements_in_inner_scope(
        &self,
        env: &mut Environment,
    ) -> Result<Value, InterpreterError> {
        // interpret all statetements
        for statement in &self.statements {
            statement.interpret_statement(env)?;
        }
        if let Some(exp) = &self.final_expression {
            exp.interpret_expression(env)
        } else {
            Ok(Value::nil())
        }
    }
}
