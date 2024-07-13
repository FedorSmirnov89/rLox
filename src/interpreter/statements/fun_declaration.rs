use crate::{domain::grammar::FunDeclaration, interpreter::error::InterpreterError, Environment};

use super::{InterpretedStatement, Outcome};

impl InterpretedStatement for FunDeclaration {
    fn interpret_statement(
        &self,
        environment: &mut Environment,
    ) -> Result<Outcome, InterpreterError> {
        let key = self.name.clone();
        environment
            .declare_fun(key, self.clone())
            .map_err(|_| InterpreterError::FunctionAlreadyDeclared(self.name.clone()))?;
        Ok(Outcome::Void)
    }
}
