use crate::{
    domain::grammar::Block,
    interpreter::{error::InterpreterError, statements::InterpretedStatement, Outcome},
    Environment,
};

use super::InterpretedExpression;

impl InterpretedExpression for Block {
    fn interpret_expression(&self, env: &mut Environment) -> Result<Outcome, InterpreterError> {
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
    ) -> Result<Outcome, InterpreterError> {
        // interpret all statetements
        for statement in &self.statements {
            let outcome = statement.interpret_statement(env)?;
            if matches!(outcome, Outcome::Return(_)) {
                return Ok(outcome);
            }
        }
        if let Some(exp) = &self.final_expression {
            exp.interpret_expression(env)
        } else {
            Ok(Outcome::Void)
        }
    }
}
