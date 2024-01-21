use rlox::ValueType;

use crate::TestApp;

#[test]
fn basic() {
    // Arrange
    let mut app = TestApp::spawn();
    let input = r#"
    var a = 0;
    while a < 10 {
        a = a + 1;
    }
    "#;

    // Act
    app.process_input(input).unwrap();

    // Assert
    let var = app.interpreter_state().get_var_value("a");
    assert!(var.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Number(10.0), var.unwrap().v_type);
}

#[test]
fn block_not_run_for_false_condition() {
    // Arrange
    let mut app = TestApp::spawn();
    let input = r#"
    var a = 0;
    while a < 0 {
        a = a + 1;
    }
    "#;

    // Act
    app.process_input(input).unwrap();

    // Assert
    let var = app.interpreter_state().get_var_value("a");
    assert!(var.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Number(0.0), var.unwrap().v_type);
}

#[test]
fn while_block_has_own_scope() {
    // Arrange
    let mut app = TestApp::spawn();
    let input = r#"
    var a = 0;
    var b = 0;
    while a < 10 {
        a = a + 1;
        var b = 1;
        b = b + 1;
    }
    "#;

    // Act
    app.process_input(input).unwrap();

    // Assert
    let var = app.interpreter_state().get_var_value("a");
    assert!(var.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Number(10.0), var.unwrap().v_type);
    let var = app.interpreter_state().get_var_value("b");
    assert!(var.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Number(0.0), var.unwrap().v_type);
}
