use anyhow::Result;

use crate::domain::{location::Location, scanning::Token};

use super::{switch_state, ClipBoard};

pub(super) fn add_char_empty(
    c: char,
    cur_location: Location,
) -> Result<(ClipBoard, Location, Vec<Token>)> {
    switch_state(c, cur_location, None)
}
