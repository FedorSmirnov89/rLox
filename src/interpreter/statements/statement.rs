use crate::{
    domain::grammar::Statement,
    interpreter::{error::InterpreterError, expressions::InterpretedExpression, Environment},
};

use super::InterpretedStatement;

impl InterpretedStatement for Statement {
    fn interpret_statement(&self, state: &mut Environment) -> anyhow::Result<(), InterpreterError> {
        match self {
            Statement::Expression(e) => {
                let value = e.interpret_expression(state)?;
                state.set_tmp_value(value);
                Ok(())
            }
            Statement::Print(e) => {
                let value = e.interpret_expression(state)?;
                println!("{}", value);
                Ok(())
            }
        }
    }
}
