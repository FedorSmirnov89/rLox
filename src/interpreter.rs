use std::{collections::HashMap, fmt::Display};

use anyhow::{bail, Result};

use crate::{
    domain::{grammar::Program, location::CodeSpan},
    Interpreter,
};

use self::{error::InterpreterError, statements::InterpretedStatement};

pub mod error;

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

///
/// The state of the interpreter:
///
/// - The current values of the global variables
///
#[derive(Debug, Default)]
pub struct Environment {
    values: HashMap<String, Value>,
    tmp_value: Option<Value>,
}

impl Environment {
    pub fn set_tmp_value(&mut self, val: Value) {
        self.tmp_value = Some(val)
    }

    pub fn get_tmp_value(&self) -> Option<&Value> {
        self.tmp_value.as_ref()
    }

    pub fn declare_var(&mut self, iden: impl Into<String>) {
        let key = iden.into();
        self.values.insert(key, Value::nil());
    }

    pub fn set_var_value(&mut self, iden: impl Into<String>, val: Value) -> Result<()> {
        let key = iden.into();
        if !self.values.contains_key(&key) {
            bail!("variable not declared")
        }
        self.values.insert(key, val);
        Ok(())
    }

    pub fn get_var_value(&self, iden: &str) -> Option<&Value> {
        self.values.get(iden)
    }
}
