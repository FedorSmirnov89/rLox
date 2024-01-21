use rlox::ValueType;

use crate::TestApp;

#[test]
fn basic() {
    // Arrange
    let mut app = TestApp::spawn();
    let input = r#"
        var a = 0;
        for {var i = 0;} {i < 10} {i = i + 1;} {
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
fn skip_block() {
    // Arrange
    let mut app = TestApp::spawn();
    let input = r#"
        var a = 0;
        for {var i = 10;} {i < 10} {i = i + 1;} {
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
fn longer_init() {
    // Arrange
    let mut app = TestApp::spawn();
    let input = r#"
        var a = 0;
        for {
            var b = 5;
            var i = b - a + 1;}
         {i < 10} {i = i + 1;} {
            a = a + 1;
        }
    "#;

    // Act
    app.process_input(input).unwrap();

    // Assert
    let var = app.interpreter_state().get_var_value("a");
    assert!(var.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Number(4.0), var.unwrap().v_type);
}

#[test]
fn shadowing_in_block() {
    // Arrange
    let mut app = TestApp::spawn();
    let input = r#"
        var a = 0;
        for {var i = 0;} {i < 10} {i = i + 1;} {
            var a = 1;
        }
    "#;

    // Act
    app.process_input(input).unwrap();

    // Assert
    let var = app.interpreter_state().get_var_value("a");
    assert!(var.is_some(), "declared variable not in state");
    assert_eq!(ValueType::Number(0.0), var.unwrap().v_type);
}
