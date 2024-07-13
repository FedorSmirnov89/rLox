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

#[test]
fn defined_add_function() {
    // Arrange
    let input = r#"
        fun add(a, b) {
            a + b
        }
        var x = add(1, 2);
    "#;

    let mut test_app = TestApp::spawn();

    // Act - interpret the input
    test_app.process_input(input).unwrap();

    // Assert - we expect 3
    let var = test_app.interpreter_state().get_var_value("x");
    assert!(var.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Number(3.0), var.unwrap().v_type);
}

#[test]
fn function_defintion_respects_scope() {
    // Arrange
    let input = r#"
        fun add(a, b) {
            a + b
        }
        var a = add(1, 2);
        var b;
        {
            fun add(a, b) {
                a - b
            }
            b = add(1, 2);
        };
        var c = add(1, 2);
        "#;
    let mut test_app = TestApp::spawn();

    // Act - interpret the input
    test_app.process_input(input).unwrap();

    // Assert - we expect 3, -1, 3
    let var_a = test_app.interpreter_state().get_var_value("a");
    assert!(var_a.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Number(3.0), var_a.unwrap().v_type);

    let var_b = test_app.interpreter_state().get_var_value("b");
    assert!(var_b.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Number(-1.0), var_b.unwrap().v_type);

    let var_c = test_app.interpreter_state().get_var_value("c");
    assert!(var_c.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Number(3.0), var_c.unwrap().v_type);
}

#[test]
fn wrong_arg_number_on_call() {
    // Arrange
    let input = r#"
        fun add(a, b) {
            a + b
        }
        var x = add(1);
    "#;

    let mut test_app = TestApp::spawn();

    // Act - interpret the input
    let result = test_app.process_input(input);

    // Assert - we expect an error
    assert_err!(result);
}

#[test]
fn early_return() {
    // Arrange
    let input = r#"
    fun my_fun(a, flag){
        a = a + 1;
        if (flag) {
            return a;
        };
        a = a + 1;
        a
    }

    var x = my_fun(1, true);
    var y = my_fun(1, false);
    "#;
    let mut test_app = TestApp::spawn();

    // Act - interpret the input
    test_app.process_input(input).unwrap();

    // Assert
    let var_y = test_app.interpreter_state().get_var_value("y");
    assert!(var_y.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Number(3.0), var_y.unwrap().v_type);

    let var_x = test_app.interpreter_state().get_var_value("x");
    assert!(var_x.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Number(2.0), var_x.unwrap().v_type);
}
