//! Module for the representation of location within the provided
//! source code.

use std::{cmp::Ordering, fmt::Display};

///
/// Represents the start location of a token within the source code.
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Location {
    pub line: u64,
    pub column: u16,
    pub pos: usize,
}

impl Location {
    pub(crate) fn shifted(&self, shift: usize) -> Self {
        Self {
            line: self.line,
            column: self.column + shift as u16,
            pos: self.pos + shift,
        }
    }

    pub(crate) fn shifted_back(&self, shift: usize) -> Self {
        if self.pos < shift {
            panic!(
                "Cannot shift back by {} characters, as the pos is only {}",
                shift, self.pos
            );
        }
        Self {
            line: self.line,
            column: self.column - shift as u16,
            pos: self.pos - shift,
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(line: {}, column: {}, pos: {})",
            self.line, self.column, self.pos
        )
    }
}

impl Default for Location {
    fn default() -> Self {
        Self {
            line: 1,
            column: 1,
            pos: 0,
        }
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.line < other.line {
            Some(Ordering::Less)
        } else if self.line > other.line {
            Some(Ordering::Greater)
        } else {
            if self.column < other.column {
                Some(Ordering::Less)
            } else if self.column > other.column {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Equal)
            }
        }
    }
}

///
/// Represents a span of code, i.e., the code between a start and an end location
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CodeSpan {
    pub start: Location,
    pub end: Location,
}

impl CodeSpan {
    pub(crate) fn extend_to_left(&self, shift: usize) -> Self {
        Self {
            start: self.start.shifted_back(shift),
            ..*self
        }
    }

    pub(crate) fn merged(left: CodeSpan, right: CodeSpan) -> Self {
        Self {
            start: left.start,
            end: right.end,
        }
    }

    pub(crate) fn in_between(left: CodeSpan, right: CodeSpan) -> Self {
        Self {
            start: left.end.shifted(1),
            end: right.start.shifted_back(1),
        }
    }
}

impl Default for CodeSpan {
    fn default() -> Self {
        Self {
            start: Location::default(),
            end: Location::default(),
        }
    }
}

impl Display for CodeSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "span from {start} to {end}",
            start = self.start,
            end = self.end
        )
    }
}

#[cfg(test)]
mod test {
    use crate::domain::location::Location;

    #[test]
    fn location_compare() {
        let loc = Location {
            line: 2,
            column: 5,
            pos: 4,
        };
        let loc_same = Location {
            line: 2,
            column: 5,
            pos: 4,
        };
        let loc_later_1 = Location {
            line: 3,
            column: 1,
            pos: 5,
        };
        let loc_later_2 = Location {
            line: 2,
            column: 6,
            pos: 5,
        };
        let loc_ealier_1 = Location {
            line: 1,
            column: 1,
            pos: 3,
        };
        let loc_ealier_2 = Location {
            line: 2,
            column: 4,
            pos: 3,
        };

        assert_eq!(loc, loc_same);
        assert!(loc < loc_later_1);
        assert!(loc < loc_later_2);
        assert!(loc > loc_ealier_1);
        assert!(loc > loc_ealier_2);
    }
}
