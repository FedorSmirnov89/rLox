use std::fmt::Display;

use anyhow::Result;

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
    pub span: CodeSpan,
}

impl Value {
    pub fn new(v_type: ValueType, span: CodeSpan) -> Self {
        Self { v_type, span }
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
    pub(crate) fn interpret(
        &mut self,
        program: Program,
    ) -> Result<Option<Value>, Vec<InterpreterError>> {
        let state = &mut self.state;
        if program.len() != 1 {
            unimplemented!("Only working with one statement at a time for now");
        }

        let declaration = &program[0];
        match declaration.interpret_statement(state) {
            Ok(()) => Ok(state.get_value().cloned()),
            Err(e) => Err(vec![e]),
        }
    }
}

///
/// The state of the interpreter:
///
/// - The current values of the global variables
///
#[derive(Debug, Default)]
pub(crate) struct State {
    value: Option<Value>,
}

impl State {
    pub fn set_value(&mut self, val: Value) {
        self.value = Some(val)
    }

    pub fn get_value(&self) -> Option<&Value> {
        self.value.as_ref()
    }
}
