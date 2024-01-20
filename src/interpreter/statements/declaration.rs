use crate::{
    domain::grammar::Declaration,
    interpreter::{error::InterpreterError, Environment},
};

use super::InterpretedStatement;

impl InterpretedStatement for Declaration {
    fn interpret_statement(&self, env: &mut Environment) -> Result<(), InterpreterError> {
        match self {
            Declaration::Declaration(var_decl) => var_decl.interpret_statement(env),
            Declaration::Statement(s) => s.interpret_statement(env),
            Declaration::Block(block) => block.interpret_statement(env),
        }
    }
}
