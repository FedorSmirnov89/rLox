use anyhow::{bail, Result};

use crate::domain::{location::Location, scanning::Token};

use super::{state_changed, switch_state, ClipBoard};

pub(super) fn add_char_identifier(
    cur_state: ClipBoard,
    c: char,
    cur_location: Location,
) -> Result<(ClipBoard, Location, Vec<Token>)> {
    let state_changed = state_changed(&cur_state, c);
    let ClipBoard::Identifier(mut chars) = cur_state else {
        bail!("called with wrong state");
    };

    if state_changed {
        let loc_after_string = cur_location.advance_str(&chars);
        let token = Token::identifier(chars, cur_location);
        switch_state(c, loc_after_string, Some(token))
    } else {
        chars.push(c);
        Ok((ClipBoard::Identifier(chars), cur_location, vec![]))
    }
}
