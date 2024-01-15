use rlox::ValueType;

use crate::TestApp;

#[test]
fn declared_var_can_be_used_as_expression() {
    // Arrange
    let input = r#"
        var a = 1;
        var b = a + 2;
    "#;
    let mut test_app = TestApp::spawn();

    // Act - interpret the input
    test_app.process_input(input).unwrap();

    // Assert - check  the value of b
    let var = test_app.interpreter_state().get_var_value("b");
    assert!(var.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Number(3.0), var.unwrap().v_type);
}

#[test]
fn accessing_undeclared_var_is_error() {
    // Arrange
    let input = r#"
        var a = 1;
        var b = a + c;
    "#;
    let mut test_app = TestApp::spawn();

    // Act - interpret the input
    let result = test_app.process_input(input);

    // Assert - check  the value of b
    assert!(result.is_err());
}
