use claim::{assert_err, assert_ok};
use rlox::ValueType;

use crate::TestApp;

macro_rules! expected {
    (num $n: expr) => {
        ValueType::Number($n)
    };
    (str $s: expr) => {
        ValueType::string($s)
    };
    (nil) => {
        ValueType::Nil
    };
    (true) => {
        ValueType::Boolean(true)
    };
    (false) => {
        ValueType::Boolean(false)
    };
}

macro_rules! it_interpreter {
    (name: $fn_name: literal | input: $input: literal | $expected: expr) => {
        paste::item! {
            #[test]
            fn  [<$fn_name>]() {
                // Arrange
                let input = format!("{};", $input);
                let mut test_app = TestApp::spawn();
                // Act
                let output = test_app.process_input(&input);
                // Assert
                assert_ok!(&output);
                assert_eq!($expected, output.unwrap().v_type);
            }
        }
    };
}

macro_rules! it_interpreter_err {
    (name: $fn_name: literal | input: $input: literal) => {
        paste::item! {
            #[test]
            fn  [<$fn_name "_err">]() {
                // Arrange
                let input = format!("{};", $input);
                let mut test_app = TestApp::spawn();
                // Act
                let output = test_app.process_input(&input);
                // Assert
                assert_err!(&output);
            }
        }
    };
}

// single values

// primary

it_interpreter! { name: "single_number" | input: "42" | expected!(num 42.0)}
it_interpreter! { name: "single_string" | input: r#""a""# | expected!(str "a")}
it_interpreter! { name: "single_nil" | input: "nil" | expected!(nil)}
it_interpreter! { name: "single_true" | input: "true" | expected!(true)}
it_interpreter! { name: "single_false" | input: "false" | expected!(false)}

// grouping

it_interpreter! { name: "grouping_single" | input: "(42)" | expected!(num 42.0)}

// unary

it_interpreter! { name: "unary_negation_arith" | input: "-42" | expected!(num -42.0)}
it_interpreter_err! { name: "unary_negation_arith" | input: r#"-"42""# }
it_interpreter! { name: "unary_negation_log" | input: "!true" | expected!(false)}
it_interpreter_err! { name: "unary_negation_log" | input: "!42" }

// factor

it_interpreter! { name: "factor_mult" | input: "2 * 3" | expected!(num 6.0)}
it_interpreter_err! { name: "factor_mult" | input: "7 * true"}
it_interpreter! { name: "factor_div" | input: "6 / 3" | expected!(num 2.0)}
it_interpreter_err! { name: "factor_div" | input: r#"7 / "a""#}

// term

it_interpreter! { name: "term_add" | input: "2 + 3" | expected!(num 5.0)}
it_interpreter_err! { name: "term_add" | input: "7 + true"}
it_interpreter! { name: "term_sub" | input: "6 - 3" | expected!(num 3.0)}
it_interpreter_err! { name: "term_sub" | input: r#"7 - "a""#}

it_interpreter!(name: "term_add_str" | input: r#""a " + "b""# | expected!(str "a b"));
it_interpreter_err! { name: "term_add_str" | input: r#"7 + "a""#}

// comparison

it_interpreter! { name: "comparison_lt" | input: "2 < 3" | expected!(true)}
it_interpreter_err! { name: "comparison_lt" | input: "7 < true"}
it_interpreter! { name: "comparison_gt" | input: "6 > 3" | expected!(true)}
it_interpreter_err! { name: "comparison_gt" | input: r#"7 > "a""#}
it_interpreter! { name: "comparison_lte" | input: "2 <= 1" | expected!(false)}
it_interpreter_err! { name: "comparison_lte" | input: "7 <= true"}
it_interpreter! { name: "comparison_gte" | input: "6 >= 3" | expected!(true)}
it_interpreter_err! { name: "comparison_gte" | input: r#"7 >= "a""#}

// equality

it_interpreter! { name: "equality_eq" | input: "2 == 3" | expected!(false)}
it_interpreter_err! { name: "equality_eq" | input: "7 == true"}
it_interpreter! { name: "equality_neq" | input: "6 != 3" | expected!(true)}
it_interpreter_err! { name: "equality_neq" | input: r#"7 != "a""#}
it_interpreter!(name: "equality_eq_str" | input: r#""a" == "a""# | expected!(true));
it_interpreter_err! { name: "equality_eq_str" | input: r#"7 == "a""#}

// more complex expressions

it_interpreter! { name: "complex_expr_1" | input: "2 + 3 * 4" | expected!(num 14.0)}
it_interpreter! { name: "complex_expr_2" | input: "3*4 < 10 " | expected!(false)}
it_interpreter! { name: "complex_expr_3" | input: "(3 +2) *4 < 13 == true" | expected!(false)}
