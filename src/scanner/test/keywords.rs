use claim::assert_matches;

use crate::{
    domain::scanning::{Location, Token, TokenType},
    scanner::scan_input,
};

macro_rules! kw_test {
    ($kw: literal, $t_type: expr) => {
        paste::item! {
            #[test]
            fn [<$kw "_recognized">](){
                exact_keyword_recognized($kw, $t_type);
            }

            #[test]
            fn [<$kw "_shorter_is_identifier">](){
                shorter_keyword_is_identifier($kw);
            }

            #[test]
            fn [<$kw "_longer_is_identifier">](){
                longer_keyword_is_identifier($kw);
            }

        }
    };
}

kw_test!("print", TokenType::PRINT);
kw_test!("and", TokenType::AND);
kw_test!("class", TokenType::CLASS);
kw_test!("else", TokenType::ELSE);
kw_test!("false", TokenType::FALSE);
kw_test!("for", TokenType::FOR);
kw_test!("fun", TokenType::FUN);
kw_test!("if", TokenType::IF);
kw_test!("nil", TokenType::NIL);
kw_test!("or", TokenType::OR);
kw_test!("return", TokenType::RETURN);
kw_test!("super", TokenType::SUPER);
kw_test!("this", TokenType::THIS);
kw_test!("true", TokenType::TRUE);
kw_test!("var", TokenType::VAR);
kw_test!("while", TokenType::WHILE);

fn exact_keyword_recognized(s: &str, t_type: TokenType) {
    let input = &format!("a {s} b");
    let output = scan_input(input).unwrap();
    let loc1 = Location {
        column: 1,
        line: 1,
        pos: 0,
    };
    let loc2 = Location {
        column: 1 + "a ".len() as u16,
        line: 1,
        pos: 0 + "a ".len() as u64,
    };
    let loc3 = Location {
        column: 1 + format!("a {s} ").len() as u16,
        line: 1,
        pos: 0 + format!("a {s} ").len() as u64,
    };
    let loc4 = Location {
        column: 1 + format!("a {s} b").len() as u16,
        line: 1,
        pos: 0 + format!("a {s} b").len() as u64,
    };
    let expected = vec![
        Token::identifier("a", loc1),
        Token::identifier(s, loc2),
        Token::identifier("b", loc3),
        Token::eof(loc4),
    ];
    assert_eq!(expected[0], output[0], "first");
    assert_eq!(expected[1], output[1], "second");
    assert_eq!(expected[2], output[2], "third");
    assert_eq!(expected[3], output[3], "fourth");

    assert!(&expected[1].t_type == &t_type);
}

fn shorter_keyword_is_identifier(s: &str) {
    let s = &s[0..s.len() - 1];
    let input = &format!("a {s} b");

    let output = scan_input(input).unwrap();
    let loc1 = Location {
        column: 1,
        line: 1,
        pos: 0,
    };
    let loc2 = Location {
        column: 1 + "a ".len() as u16,
        line: 1,
        pos: 0 + "a ".len() as u64,
    };
    let loc3 = Location {
        column: 1 + format!("a {s} ").len() as u16,
        line: 1,
        pos: 0 + format!("a {s} ").len() as u64,
    };
    let loc4 = Location {
        column: 1 + format!("a {s} b").len() as u16,
        line: 1,
        pos: 0 + format!("a {s} b").len() as u64,
    };
    let expected = vec![
        Token::identifier("a", loc1),
        Token::identifier(s, loc2),
        Token::identifier("b", loc3),
        Token::eof(loc4),
    ];
    assert_eq!(expected[0], output[0], "first");
    assert_eq!(expected[1], output[1], "second");
    assert_eq!(expected[2], output[2], "third");
    assert_eq!(expected[3], output[3], "fourth");

    assert_matches!(&expected[1].t_type, TokenType::Identifier(_));
}

fn longer_keyword_is_identifier(s: &str) {
    let input = &format!("a {s}b");

    let output = scan_input(input).unwrap();
    let loc1 = Location {
        column: 1,
        line: 1,
        pos: 0,
    };
    let loc2 = Location {
        column: 1 + "a ".len() as u16,
        line: 1,
        pos: 0 + "a ".len() as u64,
    };
    let loc3 = Location {
        column: 1 + format!("a {s}b").len() as u16,
        line: 1,
        pos: 0 + format!("a {s}b").len() as u64,
    };
    let expected = vec![
        Token::identifier("a", loc1),
        Token::identifier(format!("{s}b"), loc2),
        Token::eof(loc3),
    ];
    assert_eq!(expected[0], output[0], "first");
    assert_eq!(expected[1], output[1], "second");
    assert_eq!(expected[2], output[2], "third");

    assert_matches!(&expected[1].t_type, TokenType::Identifier(_));
}
