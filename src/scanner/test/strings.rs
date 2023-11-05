use claim::assert_err;

use crate::{
    domain::scanning::{Location, Token, TokenType},
    scanner::scan_input,
};

#[test]
fn string_assignment() {
    let input = r#"a = "my great string""#;
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
        column: 22,
        line: 1,
        pos: 21,
    };
    let expected = vec![
        Token::identifier("a", loc1),
        Token::one_two_char(TokenType::Equal, loc2),
        Token::string("my great string", loc3),
        Token::eof(loc4),
    ];
    assert_eq!(expected[0], output[0], "first");
    assert_eq!(expected[1], output[1], "second");
    assert_eq!(expected[2], output[2], "third");
    assert_eq!(expected[3], output[3], "fourth");
}

#[test]
fn string_end_error() {
    let input = r#"a="my great string"#;
    let output = scan_input(input);
    assert_err!(output);
}

#[test]
fn string_not_spaced() {
    let input = r#"a="my great string""#;
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
        column: 20,
        line: 1,
        pos: 19,
    };
    let expected = vec![
        Token::identifier("a", loc1),
        Token::one_two_char(TokenType::Equal, loc2),
        Token::string("my great string", loc3),
        Token::eof(loc4),
    ];
    assert_eq!(expected[0], output[0], "first");
    assert_eq!(expected[1], output[1], "second");
    assert_eq!(expected[2], output[2], "third");
    assert_eq!(expected[3], output[3], "fourth");
}
