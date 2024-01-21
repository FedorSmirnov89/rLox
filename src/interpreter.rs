use std::fmt::Display;

use anyhow::Result;

use crate::{
    domain::{grammar::Program, location::CodeSpan},
    Interpreter,
};

use self::{error::InterpreterError, statements::InterpretedStatement};

pub mod environment;
pub mod error;

pub use environment::*;

mod expressions;
mod statements;

#[derive(Debug, PartialEq, Clone)]
pub struct Value {
    pub v_type: ValueType,
    span: Option<CodeSpan>,
}

impl Value {
    pub fn new(v_type: ValueType, span: CodeSpan) -> Self {
        Self {
            v_type,
            span: Some(span),
        }
    }

    pub fn nil() -> Self {
        Self {
            v_type: ValueType::Nil,
            span: None,
        }
    }

    pub fn span(&self) -> CodeSpan {
        self.span.unwrap()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValueType {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl ValueType {
    pub fn variant_name(&self) -> &'static str {
        match self {
            ValueType::Number(_) => "Number",
            ValueType::String(_) => "String",
            ValueType::Boolean(_) => "Boolean",
            ValueType::Nil => "Nil",
        }
    }
}

impl ValueType {
    pub fn string(s: impl Into<String>) -> Self {
        ValueType::String(s.into())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{v_type}", v_type = self.v_type)
    }
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Number(n) => write!(f, "{n}"),
            ValueType::String(s) => write!(f, "'{s}'"),
            ValueType::Boolean(b) => {
                let bool_str = match b {
                    true => "TRUE",
                    false => "FALSE",
                };
                write!(f, "{bool_str}")
            }
            ValueType::Nil => write!(f, "NIL"),
        }
    }
}

impl Interpreter {
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
