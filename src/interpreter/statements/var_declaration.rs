use crate::{
    domain::grammar::VarDeclaration,
    interpreter::{error::InterpreterError, expressions::InterpretedExpression},
    value, Environment,
};

use super::{InterpretedStatement, Outcome};

impl InterpretedStatement for VarDeclaration {
    fn interpret_statement(&self, state: &mut Environment) -> Result<Outcome, InterpreterError> {
        let iden = match self {
            VarDeclaration::Declare(i) => i,
            VarDeclaration::DeclareAndAssign(i, _) => i,
        };
        state.declare_var(iden.as_ref());

        match self {
            VarDeclaration::DeclareAndAssign(_, e) => {
                let val = value!(e, state);
                state
                    .set_var_value(iden.as_ref(), val)
                    .expect("variable was just declared");
            }
            VarDeclaration::Declare(_) => {}
        }
        Ok(Outcome::Void)
    }
}
