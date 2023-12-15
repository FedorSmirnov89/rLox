use crate::domain::location::Location;
use crate::domain::scanning::{Token, TokenType};
use crate::scanner::scan_input;
use crate::{operator, splitting_test};

use super::{
    test_one_char_not_spaced, test_one_char_spaced, test_two_chars_not_spaced,
    test_two_chars_spaced,
};

splitting_test!(division, spaced);
splitting_test!(division, not spaced);

#[test]
fn comments_are_ignored() {
    let input = "// here is a comment line\nabc\n bcd";
    let output = scan_input(input).unwrap();
    let loc1 = Location {
        column: 1,
        line: 2,
        pos: 26,
    };
    let loc2 = Location {
        column: 2,
        line: 3,
        pos: 31,
    };
    let loc3 = Location {
        column: 5,
        line: 3,
        pos: 34,
    };
    let expected = vec![
        Token::identifier("abc", loc1),
        Token::identifier("bcd", loc2),
        Token::eof(loc3),
    ];
    assert_eq!(expected[0], output[0], "first");
    assert_eq!(expected[1], output[1], "second");
    assert_eq!(expected[2], output[2], "third");
}

#[test]
fn comments_are_ignored_multi_line() {
    let input =
        "// here is a comment line\nabc// and here is another comment\n bcd // and a final one";
    let output = scan_input(input).unwrap();
    let loc1 = Location {
        column: 1,
        line: 2,
        pos: 26,
    };
    let loc2 = Location {
        column: 2,
        line: 3,
        pos: 61,
    };
    let loc3 = Location {
        column: 24,
        line: 3,
        pos: 83,
    };
    let expected = vec![
        Token::identifier("abc", loc1),
        Token::identifier("bcd", loc2),
        Token::eof(loc3),
    ];
    assert_eq!(expected[0], output[0], "first");
    assert_eq!(expected[1], output[1], "second");
    assert_eq!(expected[2], output[2], "third");
}
