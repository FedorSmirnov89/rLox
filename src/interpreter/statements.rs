use crate::{Value, ValueType};

use super::{error::InterpreterError, Environment};

mod declaration;
mod fun_declaration;
mod statement;
mod var_declaration;

pub(crate) trait InterpretedStatement {
    fn interpret_statement(&self, environment: &mut Environment) -> Result<(), InterpreterError>;
}

///
/// Used to convert a result returned by something which can be used both as a statement and an expression
///
fn into_statement_result(res: Result<Value, InterpreterError>) -> Result<(), InterpreterError> {
    if let ValueType::Nil = res?.v_type {
        Ok(())
    } else {
        Err(InterpreterError::UnusedValueError)
    }
}
