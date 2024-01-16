use crate::{
    domain::grammar::Declaration,
    interpreter::{error::InterpreterError, Environment},
};

use super::InterpretedStatement;

impl InterpretedStatement for Declaration {
    fn interpret_statement(&self, state: &mut Environment) -> Result<(), InterpreterError> {
        match self {
            Declaration::Declaration(var_decl) => var_decl.interpret_statement(state),
            Declaration::Statement(s) => s.interpret_statement(state),
        }
    }
}
