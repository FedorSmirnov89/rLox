use crate::{
    domain::{
        location::Location,
        scanning::{Token, TokenType},
    },
    scanner::scan_input,
};

use super::{assert_result_locations_match, assert_result_types_match};

macro_rules! one_char_test {
    ($c: literal, $name: literal, $t_type: expr) => {
        paste::item! {
            #[test]
            fn [<$name "_spaced">](){
                one_char_spaced($c, $t_type);
            }

            #[test]
            fn [<$name "_not_spaced">](){
                one_char_not_spaced($c, $t_type);
            }
        }
    };
}

one_char_test!('{', "brace_left", TokenType::BraceLeft);
one_char_test!('}', "brace_right", TokenType::BraceRight);
one_char_test!('(', "parent_left", TokenType::ParenLeft);
one_char_test!(')', "parent_right", TokenType::ParenRight);
one_char_test!(',', "comma", TokenType::Comma);
one_char_test!('.', "dot", TokenType::Dot);
one_char_test!('-', "minus", TokenType::Minus);
one_char_test!('+', "plus", TokenType::Plus);
one_char_test!(';', "semi_colon", TokenType::Semicolon);
one_char_test!('*', "star", TokenType::Star);

fn one_char_spaced(c: char, t_type: TokenType) {
    let input = &format!("a {c} b");
    let output = scan_input(input).unwrap();

    let loc1 = Location {
        column: 1,
        line: 1,
        pos: 0,
    };
    let loc2 = Location {
        column: 1 + "a ".len() as u16,
        line: 1,
        pos: 0 + "a ".len(),
    };
    let loc3 = Location {
        column: 1 + "a ( ".len() as u16,
        line: 1,
        pos: 0 + "a ( ".len(),
    };
    let loc4 = Location {
        column: 1 + "a ( b".len() as u16,
        line: 1,
        pos: 0 + "a ( b".len(),
    };
    let expected = vec![
        Token::identifier("a", loc1),
        Token::one_char(t_type, loc2),
        Token::identifier("b", loc3),
        Token::eof(loc4),
    ];

    assert_result_types_match(&output, &expected);
    assert_result_locations_match(&output, &expected);
}

fn one_char_not_spaced(c: char, t_type: TokenType) {
    let input = &format!("a{c}b");
    let output = scan_input(input).unwrap();

    let loc1 = Location {
        column: 1,
        line: 1,
        pos: 0,
    };
    let loc2 = Location {
        column: 1 + "a".len() as u16,
        line: 1,
        pos: 0 + "a".len(),
    };
    let loc3 = Location {
        column: 1 + "a(".len() as u16,
        line: 1,
        pos: 0 + "a(".len(),
    };
    let loc4 = Location {
        column: 1 + "a(b".len() as u16,
        line: 1,
        pos: 0 + "a(b".len(),
    };
    let expected = vec![
        Token::identifier("a", loc1),
        Token::one_char(t_type, loc2),
        Token::identifier("b", loc3),
        Token::eof(loc4),
    ];

    assert_result_types_match(&output, &expected);
    assert_result_locations_match(&output, &expected);
}

#[test]
fn tokens_different_braces() {
    let input = "({abc})";
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
        column: 6,
        line: 1,
        pos: 5,
    };
    let loc5 = Location {
        column: 7,
        line: 1,
        pos: 6,
    };
    let loc6 = Location {
        column: 8,
        line: 1,
        pos: 7,
    };
    let expected = vec![
        Token::one_char(TokenType::ParenLeft, loc1),
        Token::one_char(TokenType::BraceLeft, loc2),
        Token::identifier("abc", loc3),
        Token::one_char(TokenType::BraceRight, loc4),
        Token::one_char(TokenType::ParenRight, loc5),
        Token::eof(loc6),
    ];
    assert_result_locations_match(&output, &expected);
    assert_result_types_match(&output, &expected);
}

#[test]
fn tokens_left_bracket() {
    let input = "(abc)";
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
        Token::one_char(TokenType::ParenLeft, loc1),
        Token::identifier("abc", loc2),
        Token::one_char(TokenType::ParenRight, loc3),
        Token::eof(loc4),
    ];
    assert_result_locations_match(&output, &expected);
    assert_result_types_match(&output, &expected);
}

#[test]
fn tokens_braces() {
    let input = "{ab}";
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
        Token::one_char(TokenType::BraceLeft, loc1),
        Token::identifier("ab", loc2),
        Token::one_char(TokenType::BraceRight, loc3),
        Token::eof(loc4),
    ];
    assert_result_types_match(&output, &expected);
    assert_result_locations_match(&output, &expected);
}
