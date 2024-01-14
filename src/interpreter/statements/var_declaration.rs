use crate::{domain::grammar::VarDeclaration, interpreter::error::InterpreterError, State, Value};

use super::InterpretedStatement;

impl InterpretedStatement for VarDeclaration {
    fn interpret_statement(&self, state: &mut State) -> Result<(), InterpreterError> {
        let iden = match self {
            VarDeclaration::Declare(i) => i,
            VarDeclaration::DeclareAndAssign(_, _) => todo!(),
        };

        let value = match self {
            VarDeclaration::Declare(_) => Value::nil(),
            VarDeclaration::DeclareAndAssign(_, _) => todo!(),
        };

        state.set_var_value(iden, value);
        Ok(())
    }
}
