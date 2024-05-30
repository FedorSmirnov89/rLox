use crate::{
    domain::grammar::Declaration,
    interpreter::{error::InterpreterError, expressions::InterpretedExpression, Environment},
};

use super::{into_statement_result, InterpretedStatement};

impl InterpretedStatement for Declaration {
    fn interpret_statement(&self, env: &mut Environment) -> Result<(), InterpreterError> {
        match self {
            Declaration::Declaration(var_decl) => var_decl.interpret_statement(env),
            Declaration::Statement(s) => s.interpret_statement(env),
            Declaration::Block(block) => into_statement_result(block.interpret_expression(env)),
        }
    }
}
