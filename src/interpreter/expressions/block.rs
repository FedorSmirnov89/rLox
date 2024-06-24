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
            dbg!("interpreting statemet");
            statement.interpret_statement(env)?;
        }
        if let Some(exp) = &self.final_expression {
            dbg!("interpreting final expression");
            let final_value = exp.interpret_expression(env);
            dbg!(&final_value);
            final_value
        } else {
            Ok(Value::nil())
        }
    }
}
