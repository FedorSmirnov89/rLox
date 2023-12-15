use claim::assert_err;

use crate::TestApp;

#[test]
fn scanner_returns_error_for_string_with_invalid_char() {
    // Arrange
    let input = "abc @bla";
    let mut test_app = TestApp::spawn();
    // Act
    let output = test_app.process_input(input);
    // Assert
    assert_err!(output);
}

#[test]
fn scanner_returns_multiple_errors_for_multiple_invalid_chars() {
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
