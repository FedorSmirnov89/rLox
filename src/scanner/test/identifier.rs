use crate::{
    domain::{location::Location, scanning::Token},
    scanner::scan_input,
};

#[test]
fn tokens_two_identifier() {
    let input = "abc\n bcd";
    let output = scan_input(input).unwrap();
    let loc1 = Location {
        column: 1,
        line: 1,
        pos: 0,
    };
    let loc2 = Location {
        column: 2,
        line: 2,
        pos: 5,
    };
    let loc3 = Location {
        column: 5,
        line: 2,
        pos: 8,
    };
    let expected = vec![
        Token::keyword_or_identifier("abc", loc1),
        Token::keyword_or_identifier("bcd", loc2),
        Token::eof(loc3),
    ];
    assert_eq!(expected[0], output[0], "first");
    assert_eq!(expected[1], output[1], "second");
    assert_eq!(expected[2], output[2], "third");

    let c_at_pos = input.chars().nth(5).unwrap();
    assert_eq!(c_at_pos, 'b');
}

#[test]
fn tokens_identifier() {
    let input = "abc";
    let output = scan_input(input).unwrap();
    let loc1 = Location {
        column: 1,
        line: 1,
        pos: 0,
    };
    let loc2 = Location {
        column: 4,
        line: 1,
        pos: 3,
    };
    let expected = vec![Token::keyword_or_identifier("abc", loc1), Token::eof(loc2)];
    assert_eq!(expected, output);
}
