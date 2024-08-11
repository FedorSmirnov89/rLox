use anyhow::Result;
use rlox::{Environment, Interpreter, LoxError, Value};

mod interpreter;
mod parser;
mod scanner;

struct TestApp {
    interpreter: Interpreter,
}

impl TestApp {
    fn spawn() -> Self {
        Self {
            interpreter: Interpreter::default(),
        }
    }

    fn process_input(&mut self, input: &str) -> Result<Option<Value>, Vec<LoxError>> {
        self.interpreter.interpret_src_str(input)
    }

    fn interpreter_state(&self) -> &Environment {
        &self.interpreter.environment()
    }
}
