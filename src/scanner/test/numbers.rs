use crate::{
    domain::{
        location::Location,
        scanning::{Token, TokenType},
    },
    scanner::scan_input,
};

#[test]
fn integer_spaced() {
    let input = r#"a = 42"#;
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
        column: 7,
        line: 1,
        pos: 6,
    };
    let expected = vec![
        Token::keyword_or_identifier("a", loc1),
        Token::one_two_char(TokenType::Equal, loc2),
        Token::number("42.0", loc3),
        Token::eof(loc4),
    ];
    assert_eq!(expected[0], output[0], "first");
    assert_eq!(expected[1], output[1], "second");
    assert_eq!(expected[2], output[2], "third");
    assert_eq!(expected[3], output[3], "fourth");
}

#[test]
fn integer_spaced_not_ending() {
    let input = r#"a = 42 < b"#;
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
        column: 8,
        line: 1,
        pos: 7,
    };

    let loc5 = Location {
        column: 10,
        line: 1,
        pos: 9,
    };
    let loc6 = Location {
        column: 11,
        line: 1,
        pos: 10,
    };
    let expected = vec![
        Token::keyword_or_identifier("a", loc1),
        Token::one_two_char(TokenType::Equal, loc2),
        Token::number("42.0", loc3),
        Token::one_two_char(TokenType::Less, loc4),
        Token::keyword_or_identifier("b", loc5),
        Token::eof(loc6),
    ];
    assert_eq!(expected[0], output[0], "first");
    assert_eq!(expected[1], output[1], "second");
    assert_eq!(expected[2], output[2], "third");
    assert_eq!(expected[3], output[3], "fourth");
    assert_eq!(expected[4], output[4], "fifth");
    assert_eq!(expected[5], output[5], "sixth");
}

#[test]
fn integer_not_spaced() {
    let input = r#"a=42"#;
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
        column: 5,
        line: 1,
        pos: 4,
    };
    let expected = vec![
        Token::keyword_or_identifier("a", loc1),
        Token::one_two_char(TokenType::Equal, loc2),
        Token::number("42.0", loc3),
        Token::eof(loc4),
    ];
    assert_eq!(expected[0], output[0], "first");
    assert_eq!(expected[1], output[1], "second");
    assert_eq!(expected[2], output[2], "third");
    assert_eq!(expected[3], output[3], "fourth");
}

#[test]
fn float() {
    let input = r#"a = 42.24"#;
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
        column: 10,
        line: 1,
        pos: 9,
    };
    let expected = vec![
        Token::keyword_or_identifier("a", loc1),
        Token::one_two_char(TokenType::Equal, loc2),
        Token::number("42.24", loc3),
        Token::eof(loc4),
    ];
    assert_eq!(expected[0], output[0], "first");
    assert_eq!(expected[1], output[1], "second");
    assert_eq!(expected[2], output[2], "third");
    assert_eq!(expected[3], output[3], "fourth");
}
