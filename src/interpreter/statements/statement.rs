use crate::{
    domain::grammar::Statement,
    interpreter::{error::InterpreterError, expressions::InterpretedExpression, Environment},
};

use super::InterpretedStatement;

impl InterpretedStatement for Statement {
    fn interpret_statement(&self, env: &mut Environment) -> anyhow::Result<(), InterpreterError> {
        match self {
            Statement::Expression(e) => {
                let value = e.interpret_expression(env)?;
                env.set_tmp_value(value);
                Ok(())
            }
            Statement::Print(e) => {
                let value = e.interpret_expression(env)?;
                println!("{}", value);
                Ok(())
            }
            Statement::Assignment(iden, expr) => {
                let value = expr.interpret_expression(&env)?;
                match env.set_var_value(iden.as_ref(), value) {
                    Ok(()) => Ok(()),
                    Err(_) => Err(InterpreterError::identifier_not_defined(iden.clone())),
                }
            }
        }
    }
}
