use claim::assert_err;
use rlox::ValueType;

use crate::TestApp;

#[test]
fn if_block_done_when_condition_true() {
    // Arrange
    let mut app = TestApp::spawn();
    let input = r#"
    var condition = true;
    var a = 1;
    if condition {
        a = 2;
    }
    "#;

    // Act
    app.process_input(input).unwrap();

    // Assert
    let var = app.interpreter_state().get_var_value("a");
    assert!(var.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Number(2.0), var.unwrap().v_type);
}

#[test]
fn if_block_not_done_when_condition_false() {
    // Arrange
    let mut app = TestApp::spawn();
    let input = r#"
    var condition = false;
    var a = 1;
    if condition {
        a = 2;
    }
    "#;

    // Act
    app.process_input(input).unwrap();

    // Assert
    let var = app.interpreter_state().get_var_value("a");
    assert!(var.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Number(1.0), var.unwrap().v_type);
}

#[test]
fn err_if_condition_not_bool() {
    // Arrange
    let mut app = TestApp::spawn();
    let input = r#"
    var condition = 1 + 2;
    var a = 1;
    if condition {
        a = 2;
    }
    "#;

    // Act
    let result = app.process_input(input);

    // Assert
    assert_err!(result);
}

#[test]
fn if_else_block_done_when_condition_true() {
    // Arrange
    let mut app = TestApp::spawn();
    let input = r#"
    var condition = true;
    var a = 1;
    if condition {
        a = 2;
    } else {
        a = 3;
    }
    "#;

    // Act
    app.process_input(input).unwrap();

    // Assert
    let var = app.interpreter_state().get_var_value("a");
    assert!(var.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Number(2.0), var.unwrap().v_type);
}

#[test]
fn if_else_block_done_when_condition_false() {
    // Arrange
    let mut app = TestApp::spawn();
    let input = r#"
    var condition = false;
    var a = 1;
    if condition {
        a = 2;
    } else {
        a = 3;
    }
    "#;

    // Act
    app.process_input(input).unwrap();

    // Assert
    let var = app.interpreter_state().get_var_value("a");
    assert!(var.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Number(3.0), var.unwrap().v_type);
}

#[test]
fn if_blocks_create_own_scope() {
    // Arrange
    let mut app = TestApp::spawn();
    let input = r#"
    var a = 1;
    var b = 1;
    if a < 10 {
        var b = 2;
        b = b + 1;
        a = b;
    }
    "#;

    // Act
    app.process_input(input).unwrap();

    // Assert
    let var = app.interpreter_state().get_var_value("a");
    assert!(var.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Number(3.0), var.unwrap().v_type);
    let var = app.interpreter_state().get_var_value("b");
    assert!(var.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Number(1.0), var.unwrap().v_type);
}
