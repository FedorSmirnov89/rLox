use std::str::Chars;

use anyhow::Result;

use crate::domain::scanning::Token;

use self::state::State;

mod state;

pub(super) fn scan_input(input: &str) -> Result<Vec<Token>, Vec<anyhow::Error>> {
    Scanner::new(input).scan()
}

struct Scanner<'input> {
    char_iterator: Chars<'input>,
    state: State,
}

enum ProcResult {
    Tokens(Vec<Token>),
    Error(anyhow::Error),
    Empty,
    Finished,
}

impl From<anyhow::Error> for ProcResult {
    fn from(value: anyhow::Error) -> Self {
        ProcResult::Error(value)
    }
}

impl<'input> Scanner<'input> {
    fn new(input: &'input str) -> Self {
        Scanner {
            char_iterator: input.chars(),
            state: State::default(),
        }
    }

    fn scan(mut self) -> Result<Vec<Token>, Vec<anyhow::Error>> {
        let mut tokens = vec![];
        let mut errors = vec![];

        loop {
            match self.process_char() {
                ProcResult::Tokens(mut t) => tokens.append(&mut t),
                ProcResult::Error(e) => errors.push(e),
                ProcResult::Empty => continue,
                ProcResult::Finished => break,
            }
        }

        if errors.is_empty() {
            tokens.push(self.state.eof());
            Ok(tokens)
        } else {
            Err(errors)
        }
    }

    fn process_char(&mut self) -> ProcResult {
        if let Some(c) = self.char_iterator.next() {
            self.state.add_char(c)
        } else {
            match self.state.extract_token() {
                Ok(Some(t)) => ProcResult::Tokens(vec![t]),
                Ok(None) => ProcResult::Finished,
                Err(e) => ProcResult::Error(e),
            }
        }
    }
}

#[cfg(test)]
mod test;
