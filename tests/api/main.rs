use anyhow::Result;
use claim::assert_err;
use rlox::Interpreter;

struct TestApp {
    interpreter: Interpreter,
}

impl TestApp {
    fn spawn() -> Self {
        Self {
            interpreter: Interpreter::default(),
        }
    }

    fn process_input(&mut self, input: &str) -> Result<(), Vec<anyhow::Error>> {
        self.interpreter.interpret_src_str(input)
    }
}

#[test]
fn interpreter_returns_error_for_string_with_invalid_char() {
    // Arrange
    let input = "abc @bla";
    let mut test_app = TestApp::spawn();
    // Act
    let output = test_app.process_input(input);
    // Assert
    assert_err!(output);
}

#[test]
fn interpreter_returns_multiple_errors_for_multiple_invalid_chars() {
    // Arrange
    let input = "abc @bla\n# blup";
    let mut test_app = TestApp::spawn();
    // Act
    let output = test_app.process_input(input);
    // Assert
    assert_err!(&output);
    if let Err(errors) = output {
        assert_eq!(2, errors.len());
    }
}

#[test]
fn interpreter_syntax_error_missing_closing_parenthesis() {
    // Arrange
    let input = "(42";
    let mut test_app = TestApp::spawn();
    // Act
    let output = test_app.process_input(input);
    // Assert
    assert_err!(&output);
    if let Err(errors) = output {
        assert_eq!(1, errors.len());
    }
}
