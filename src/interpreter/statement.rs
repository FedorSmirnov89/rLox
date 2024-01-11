use crate::domain::grammar::Statement;

use super::{InterpretatedExpression, InterpretedStatement};

impl InterpretedStatement for Statement {
    fn interpret_statement(
        &self,
        state: &mut super::State,
    ) -> anyhow::Result<(), super::error::InterpreterError> {
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
