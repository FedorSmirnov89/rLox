use crate::{
    domain::grammar::VarDeclaration,
    interpreter::{error::InterpreterError, expressions::InterpretedExpression},
    Environment,
};

use super::InterpretedStatement;

impl InterpretedStatement for VarDeclaration {
    fn interpret_statement(&self, state: &mut Environment) -> Result<(), InterpreterError> {
        let iden = match self {
            VarDeclaration::Declare(i) => i,
            VarDeclaration::DeclareAndAssign(i, _) => i,
        };
        state.declare_var(iden.as_ref());

        match self {
            VarDeclaration::DeclareAndAssign(_, e) => {
                let val = e.interpret_expression(state)?;
                state
                    .set_var_value(iden.as_ref(), val)
                    .expect("variable was just declared");
            }
            VarDeclaration::Declare(_) => {}
        }
        Ok(())
    }
}
