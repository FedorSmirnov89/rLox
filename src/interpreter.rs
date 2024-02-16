use anyhow::Result;

use crate::{
    domain::grammar::{Declaration, Expression, Program, Statement},
    parser,
    scanner::scan_input,
};

use self::{error::InterpreterError, statements::InterpretedStatement};

pub mod environment;
pub mod error;
pub mod value;

pub use environment::*;
pub use value::{Value, ValueType};

mod expressions;
mod statements;

#[derive(Default)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    ///
    /// Interprets the given source string while mutating the current state of the interpreter
    ///
    pub fn interpret_src_str(
        &mut self,
        source_str: &str,
    ) -> Result<Option<Value>, Vec<anyhow::Error>> {
        println!("interpreting the following: '{source_str}'");
        let tokens = scan_input(source_str)?;
        let program = parser::parse(tokens)?;

        if let Some(expr) = single_expression(&program) {
            print_expr_ast(expr);
        }

        match self.interpret(program) {
            Ok(value) => Ok(value),
            Err(errors) => {
                let mut interpreter_errors = vec![];
                for error in errors {
                    let e = anyhow::anyhow!(error.msg(source_str));
                    interpreter_errors.push(e);
                }
                Err(interpreter_errors)
            }
        }
    }
    ///
    /// Interprets the given program. If the last statement is an expression, the value of that
    /// expression is returned.
    ///
    pub(crate) fn interpret(
        &mut self,
        program: Program,
    ) -> Result<Option<Value>, Vec<InterpreterError>> {
        let environment = &mut self.environment;
        let mut errors = vec![];
        for decl in program.into_iter() {
            match decl.interpret_statement(environment) {
                Ok(()) => (),
                Err(e) => errors.push(e),
            }
        }
        if errors.is_empty() {
            Ok(environment.get_tmp_value().cloned())
        } else {
            Err(errors)
        }
    }

    pub fn environment(&self) -> &Environment {
        &self.environment
    }
}

fn single_expression(program: &Program) -> Option<Expression> {
    if program.len() != 1 {
        return None;
    }

    match &program[0] {
        Declaration::Statement(Statement::Expression(e)) => Some(e.clone()),
        _ => None,
    }
}

fn print_expr_ast(expr: Expression) {
    println!("here is the AST we got: ");
    println!("{}", expr);
}
