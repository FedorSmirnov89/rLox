use anyhow::Result;

use crate::domain::scanning::{Location, Token, TokenType};

use super::{switch_state, ClipBoard};

pub(super) fn add_char_bang(
    c: char,
    cur_location: Location,
) -> Result<(ClipBoard, Location, Vec<Token>)> {
    add_one_two_char_equal(c, cur_location, TokenType::Bang, TokenType::BangEqual)
}

pub(super) fn add_char_greater(
    c: char,
    cur_location: Location,
) -> Result<(ClipBoard, Location, Vec<Token>)> {
    add_one_two_char_equal(c, cur_location, TokenType::Greater, TokenType::GreaterEqual)
}

pub(super) fn add_char_less(
    c: char,
    cur_location: Location,
) -> Result<(ClipBoard, Location, Vec<Token>)> {
    add_one_two_char_equal(c, cur_location, TokenType::Less, TokenType::LessEqual)
}

pub(super) fn add_char_equal(
    c: char,
    cur_location: Location,
) -> Result<(ClipBoard, Location, Vec<Token>)> {
    add_one_two_char_equal(c, cur_location, TokenType::Equal, TokenType::EqualEqual)
}

pub(super) fn add_char_division(
    c: char,
    cur_location: Location,
) -> Result<(ClipBoard, Location, Vec<Token>)> {
    let f_trigger = |_l: Location| {
        Ok((
            ClipBoard::Comment,
            cur_location.advance_col().advance_col(),
            vec![],
        ))
    };

    add_one_two_char(c, '/', cur_location, TokenType::Division, f_trigger)
}

fn add_one_two_char_equal(
    c: char,
    cur_location: Location,
    t_type_one: TokenType,
    t_type_two: TokenType,
) -> Result<(ClipBoard, Location, Vec<Token>)> {
    let f_trigger = |location: Location| {
        Ok((
            ClipBoard::Empty,
            cur_location.advance_col().advance_col(),
            vec![Token::one_two_char(t_type_two, location)],
        ))
    };

    add_one_two_char(c, '=', cur_location, t_type_one, f_trigger)
}

fn add_one_two_char<F>(
    c: char,
    c_trigger: char,
    cur_location: Location,
    t_type_one: TokenType,
    f_trigger: F,
) -> Result<(ClipBoard, Location, Vec<Token>)>
where
    F: FnOnce(Location) -> Result<(ClipBoard, Location, Vec<Token>)>,
{
    match c {
        _ if c == c_trigger => f_trigger(cur_location),
        _ => {
            let token = Token::one_two_char(t_type_one, cur_location);
            switch_state(c, cur_location.advance_col(), Some(token))
        }
    }
}
