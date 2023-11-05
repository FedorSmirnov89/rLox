use std::fmt::Display;

use crate::domain::scanning::{Location, Token, TokenType};

use super::scan_input;

mod division_comments;
mod identifier;
mod keywords;
mod numbers;
mod one_character;
mod one_two_characters;
mod strings;

#[macro_export]
macro_rules! operator {
    (bang) => {
        ("!", TokenType::Bang)
    };
    (equal) => {
        ("=", TokenType::Equal)
    };
    (less) => {
        ("<", TokenType::Less)
    };
    (greater) => {
        (">", TokenType::Greater)
    };
    (less_equal) => {
        ("<=", TokenType::LessEqual)
    };
    (greater_equal) => {
        (">=", TokenType::GreaterEqual)
    };
    (bang_equal) => {
        ("!=", TokenType::BangEqual)
    };
    (equal_equal) => {
        ("==", TokenType::EqualEqual)
    };
    (division) => {
        ("/", TokenType::Division)
    };
}

#[macro_export]
macro_rules! splitting_test {
    ($oper: expr, spaced) => {
        paste::item! {
            #[test]
            fn [<$oper "_spaced" >]  () {
                let macro_input = operator!($oper);
                if macro_input.0.len() == 1{
                    test_one_char_spaced(macro_input.0, macro_input.1);
                }else if macro_input.0.len() == 2{
                    test_two_chars_spaced(macro_input.0, macro_input.1);
                }
            }
        }
    };
    ($oper: expr, not spaced) => {
        paste::item! {
            #[test]
            fn [<$oper "_not_spaced" >]  () {
                let macro_input = operator!($oper);
                if macro_input.0.len() == 1{
                    test_one_char_not_spaced(macro_input.0, macro_input.1);
                }else if macro_input.0.len() == 2{
                    test_two_chars_not_spaced(macro_input.0, macro_input.1);
                }
            }
        }
    };
}

fn assert_result_locations_match(output: &[Token], expected: &[Token]) {
    for (actual, expected) in output.into_iter().zip(expected.into_iter()) {
        assert_eq!(actual.location(), expected.location())
    }
}

fn assert_result_types_match(output: &[Token], expected: &[Token]) {
    for (actual, expected) in output.into_iter().zip(expected.into_iter()) {
        assert_eq!(
            actual.t_type(),
            expected.t_type(),
            "token type does not match"
        )
    }
}

fn test_two_chars_spaced(c: impl Display, t_type: TokenType) {
    let input = &format!("a {c} b");
    let output = scan_input(input).unwrap();
    let loc1 = Location {
        column: 1,
        line: 1,
        pos: 0,
    };
    let loc2 = Location {
        column: 3,
        line: 1,
        pos: 2,
    };
    let loc3 = Location {
        column: 6,
        line: 1,
        pos: 5,
    };
    let loc4 = Location {
        column: 7,
        line: 1,
        pos: 6,
    };
    let expected = vec![
        Token::identifier("a", loc1),
        Token::one_two_char(t_type, loc2),
        Token::identifier("b", loc3),
        Token::eof(loc4),
    ];
    assert_result_types_match(&output, &expected);
    assert_result_locations_match(&output, &expected);
}

fn test_one_char_spaced(c: impl Display, t_type: TokenType) {
    let input = &format!("a {c} b");
    let output = scan_input(input).unwrap();
    let loc1 = Location {
        column: 1,
        line: 1,
        pos: 0,
    };
    let loc2 = Location {
        column: 3,
        line: 1,
        pos: 2,
    };
    let loc3 = Location {
        column: 5,
        line: 1,
        pos: 4,
    };
    let loc4 = Location {
        column: 6,
        line: 1,
        pos: 5,
    };
    let expected = vec![
        Token::identifier("a", loc1),
        Token::one_two_char(t_type, loc2),
        Token::identifier("b", loc3),
        Token::eof(loc4),
    ];
    assert_result_types_match(&output, &expected);
    assert_result_locations_match(&output, &expected);
}

fn test_two_chars_not_spaced(c: impl Display, t_type: TokenType) {
    let input = &format!("a{c}b");
    let output = scan_input(input).unwrap();
    let loc1 = Location {
        column: 1,
        line: 1,
        pos: 0,
    };
    let loc2 = Location {
        column: 2,
        line: 1,
        pos: 1,
    };
    let loc3 = Location {
        column: 4,
        line: 1,
        pos: 3,
    };
    let loc4 = Location {
        column: 5,
        line: 1,
        pos: 4,
    };
    let expected = vec![
        Token::identifier("a", loc1),
        Token::one_two_char(t_type, loc2),
        Token::identifier("b", loc3),
        Token::eof(loc4),
    ];
    assert_result_types_match(&output, &expected);
    assert_result_locations_match(&output, &expected);
}

fn test_one_char_not_spaced(c: impl Display, t_type: TokenType) {
    let input = &format!("a{c}b");
    let output = scan_input(input).unwrap();
    let loc1 = Location {
        column: 1,
        line: 1,
        pos: 0,
    };
    let loc2 = Location {
        column: 2,
        line: 1,
        pos: 1,
    };
    let loc3 = Location {
        column: 3,
        line: 1,
        pos: 2,
    };
    let loc4 = Location {
        column: 4,
        line: 1,
        pos: 3,
    };
    let expected = vec![
        Token::identifier("a", loc1),
        Token::one_two_char(t_type, loc2),
        Token::identifier("b", loc3),
        Token::eof(loc4),
    ];
    assert_result_types_match(&output, &expected);
    assert_result_locations_match(&output, &expected);
}
