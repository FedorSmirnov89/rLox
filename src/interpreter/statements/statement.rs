use crate::{
    domain::grammar::Statement,
    interpreter::{error::InterpreterError, expressions::InterpretedExpression, State},
};

use super::InterpretedStatement;

impl InterpretedStatement for Statement {
    fn interpret_statement(&self, state: &mut State) -> anyhow::Result<(), InterpreterError> {
        match self {
            Statement::Expression(e) => {
                let value = e.interpret_expression()?;
                state.set_value(value);
                Ok(())
            }
            Statement::Print(e) => {
                let value = e.interpret_expression()?;
                println!("{}", value);
                Ok(())
            }
        }
    }
}