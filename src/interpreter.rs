use std::fmt::Display;

use anyhow::Result;

use crate::domain::{
    grammar::{Program, Statement},
    location::CodeSpan,
};

use self::error::InterpreterError;

pub mod error;

mod comparison;
mod equality;
mod expression;
mod factor;
mod primary;
mod term;
mod unary;

#[derive(Debug, PartialEq)]
pub struct Value {
    pub v_type: ValueType,
    pub span: CodeSpan,
}

impl Value {
    pub fn new(v_type: ValueType, span: CodeSpan) -> Self {
        Self { v_type, span }
    }
}

#[derive(Debug, PartialEq)]
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

pub(crate) fn interpret(program: Program) -> Result<Value, Vec<InterpreterError>> {
    if program.len() != 1 {
        unimplemented!("Only working with one statement at a time for now");
    }

    let expr = match &program[0] {
        Statement::Expression(e) => e,
        Statement::Print(_) => todo!("print statement"),
    };

    match expr.interpret() {
        Ok(v) => Ok(v),
        Err(e) => Err(vec![e]),
    }
}

trait Interpretation {
    fn interpret(&self) -> Result<Value, InterpreterError>;
}
