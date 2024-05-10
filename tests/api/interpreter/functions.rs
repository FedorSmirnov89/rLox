//! Module containing the tests for function-related functionality

use rlox::ValueType;

use crate::TestApp;

#[test]
fn meaning_of_life() {
    // Arrange
    let input = r#"
        var x = meaning_of_life();
    "#;

    let mut test_app = TestApp::spawn();

    // Act - interpret the input
    test_app.process_input(input).unwrap();

    // Assert - we expect 42
    let var = test_app.interpreter_state().get_var_value("x");
    assert!(var.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Number(42.0), var.unwrap().v_type);
}
