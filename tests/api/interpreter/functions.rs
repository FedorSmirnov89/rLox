//! Module containing the tests for function-related functionality

use claim::assert_err;
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

#[test]
fn meaning_of_life_wrong_arg_num() {
    // Arrange
    let input = r#"
        var x = meaning_of_life(1);
    "#;

    let mut test_app = TestApp::spawn();

    // Act - interpret the input
    let result = test_app.process_input(input);

    // Assert - we expect an error
    assert_err!(result);
}

#[test]
fn function_return_used_for_math() {
    // Arrange
    let input = r#"
        var x = meaning_of_life() - 21;
    "#;

    let mut test_app = TestApp::spawn();

    // Act - interpret the input
    test_app.process_input(input).unwrap();

    // Assert - we expect 42
    let var = test_app.interpreter_state().get_var_value("x");
    assert!(var.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Number(21.0), var.unwrap().v_type);
}

#[test]
fn is_meaning_of_life() {
    // Arrange
    let input = r#"
        var x = is_meaning_of_life(42);
    "#;

    let mut test_app = TestApp::spawn();

    // Act - interpret the input

    // unwrap or print error
    test_app.process_input(input).unwrap_or_else(|e| {
        panic!("{e:?}");
    });

    // Assert - we expect true
    let var = test_app.interpreter_state().get_var_value("x");
    assert!(var.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Boolean(true), var.unwrap().v_type);
}

#[test]
fn is_meaning_of_life_wrong_arg_type() {
    // Arrange
    let input = r#"
        var x = is_meaning_of_life("42");
    "#;

    let mut test_app = TestApp::spawn();

    // Act - interpret the input
    let result = test_app.process_input(input);

    // Assert - we expect an error
    assert_err!(result);
}
