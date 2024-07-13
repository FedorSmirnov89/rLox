use crate::{
    domain::grammar::Statement,
    interpreter::{error::InterpreterError, expressions::InterpretedExpression, Environment},
    Value,
};

use super::{InterpretedStatement, Outcome};

impl InterpretedStatement for Statement {
    fn interpret_statement(&self, env: &mut Environment) -> Result<Outcome, InterpreterError> {
        match self {
            Statement::Expression(e) => {
                let _value = e.interpret_expression(env)?; // value ignored on non-final expressions
                Ok(Outcome::Void)
            }
            Statement::FinalExpression(e) => {
                let value = e.interpret_expression(env)?;
                env.set_tmp_value(value);
                Ok(Outcome::Void)
            }
            Statement::Print(e) => {
                let value = e.interpret_expression(env)?;
                println!("{}", value);
                Ok(Outcome::Void)
            }
            Statement::Assignment(iden, expr) => {
                let value = expr.interpret_expression(env)?;
                match env.set_var_value(iden.as_ref(), value) {
                    Ok(()) => Ok(Outcome::Void),
                    Err(_) => Err(InterpreterError::identifier_not_defined(iden.clone())),
                }
            }
            Statement::Return(return_expr) => {
                let value = return_expr.interpret_expression(env)?;
                Ok(Outcome::EarlyReturn(value))
            }
            Statement::ReturnEmpty => Ok(Outcome::EarlyReturn(Value::nil())),
        }
    }
}
