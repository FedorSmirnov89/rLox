use crate::{
    domain::grammar::VarDeclaration,
    interpreter::{error::InterpreterError, expressions::InterpretedExpression},
    Environment, Value,
};

use super::InterpretedStatement;

impl InterpretedStatement for VarDeclaration {
    fn interpret_statement(&self, state: &mut Environment) -> Result<(), InterpreterError> {
        let iden = match self {
            VarDeclaration::Declare(i) => i,
            VarDeclaration::DeclareAndAssign(i, _) => i,
        };

        let value = match self {
            VarDeclaration::Declare(_) => Value::nil(),
            VarDeclaration::DeclareAndAssign(_, e) => e.interpret_expression(state)?,
        };

        state.set_var_value(iden, value);
        Ok(())
    }
}
