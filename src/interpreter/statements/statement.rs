use crate::{
    domain::grammar::Statement,
    interpreter::{error::InterpreterError, expressions::InterpretedExpression, Environment},
    value, Value,
};

use super::{InterpretedStatement, Outcome};

impl InterpretedStatement for Statement {
    fn interpret_statement(&self, env: &mut Environment) -> Result<Outcome, InterpreterError> {
        match self {
            Statement::Expression(e) => {
                match e.interpret_expression(env)? {
                    early_return @ Outcome::Return(_) => Ok(early_return),
                    Outcome::Value(_) | Outcome::Void => Ok(Outcome::Void), // value ignored on non-final expressions
                }
            }
            Statement::FinalExpression(e) => {
                let value = value!(e, env);
                env.set_tmp_value(value);
                Ok(Outcome::Void)
            }
            Statement::Print(e) => {
                let value = value!(e, env);
                println!("{}", value);
                Ok(Outcome::Void)
            }
            Statement::Assignment(iden, expr) => {
                let value = value!(expr, env);
                match env.set_var_value(iden.as_ref(), value) {
                    Ok(()) => Ok(Outcome::Void),
                    Err(_) => Err(InterpreterError::identifier_not_defined(iden.clone())),
                }
            }
            Statement::Return(return_expr) => {
                let value = value!(return_expr, env);
                Ok(Outcome::Return(value))
            }
            Statement::ReturnEmpty => Ok(Outcome::Return(Value::nil())),
        }
    }
}
