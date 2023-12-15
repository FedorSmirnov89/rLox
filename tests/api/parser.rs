use claim::assert_err;

use crate::TestApp;

#[test]
fn parser_syntax_error_missing_closing_parenthesis() {
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
