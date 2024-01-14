use rlox::ValueType;

use crate::TestApp;

#[test]
fn declared_variable_is_set_to_nil_if_no_assign() {
    // Arrange
    let input = "var a;";
    let mut test_app = TestApp::spawn();

    // Act - interpret the input
    test_app.process_input(input).unwrap();

    // Assert - check that variable is present and its value is nil
    let var = test_app.interpreter_state().get_var_value("a");
    assert!(var.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Nil, var.unwrap().v_type);
}

#[test]
fn declared_and_assign_var_is_in_state() {
    // Arrange
    let input = "var a = 1;";
    let mut test_app = TestApp::spawn();

    // Act - interpret the input
    test_app.process_input(input).unwrap();

    // Assert - check that variable is present and its value is nil
    let var = test_app.interpreter_state().get_var_value("a");
    assert!(var.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Number(1.0), var.unwrap().v_type);
}
