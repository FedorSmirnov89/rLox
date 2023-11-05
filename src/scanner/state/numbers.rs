use anyhow::Result;

use crate::domain::scanning::{Location, Token};

use super::{switch_state, ClipBoard};

pub(super) fn add_char_num_pre(
    mut n: String,
    c: char,
    cur_location: Location,
) -> Result<(ClipBoard, Location, Vec<Token>)> {
    match c {
        '.' => {
            n.push('.');
            Ok((ClipBoard::NumberPostDot(n), cur_location, vec![]))
        }
        _ if c.is_digit(10) => {
            n.push(c);
            Ok((ClipBoard::NumberPreDot(n), cur_location, vec![]))
        }
        _ => {
            let location = cur_location.advance_str(&n);
            let token = Token::number(n, cur_location);
            switch_state(c, location, Some(token))
        }
    }
}

pub(super) fn add_char_num_post(
    mut n: String,
    c: char,
    cur_location: Location,
) -> Result<(ClipBoard, Location, Vec<Token>)> {
    match c {
        _ if c.is_digit(10) => {
            n.push(c);
            Ok((ClipBoard::NumberPostDot(n), cur_location, vec![]))
        }
        _ => {
            let location = cur_location.advance_str(&n);
            let token = Token::number(n, cur_location);
            switch_state(c, location, Some(token))
        }
    }
}
