use anyhow::Result;
use rlox::{Environment, Interpreter, Value};

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

    fn process_input(&mut self, input: &str) -> Result<Option<Value>, Vec<anyhow::Error>> {
        self.interpreter.interpret_src_str(input)
    }

    fn interpreter_state(&self) -> &Environment {
        &self.interpreter.environment()
    }
}
