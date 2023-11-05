use anyhow::Result;

use crate::domain::scanning::{Location, Token, TokenType};

use self::{
    empty::add_char_empty,
    identifier::add_char_identifier,
    numbers::{add_char_num_post, add_char_num_pre},
    one_two_characters::{
        add_char_bang, add_char_division, add_char_equal, add_char_greater, add_char_less,
    },
};

use super::ProcResult;

mod empty;
mod identifier;
mod numbers;
mod one_two_characters;

#[derive(Default)]
enum ClipBoard {
    #[default]
    Empty,

    Bang,
    Equal,
    Greater,
    Less,

    Division,
    Comment,

    String(String),

    NumberPreDot(String),
    NumberPostDot(String),

    Identifier(String),
}

pub(super) struct State {
    memory: Option<ClipBoard>,
    location: Location,
}

impl Default for State {
    fn default() -> Self {
        Self {
            memory: Some(ClipBoard::default()),
            location: Default::default(),
        }
    }
}

impl Location {
    fn advance_col(self) -> Self {
        Self {
            column: self.column + 1,
            pos: self.pos + 1,
            ..self
        }
    }

    fn advance_line(self) -> Self {
        Self {
            line: self.line + 1,
            column: 1,
            pos: self.pos + 1,
        }
    }

    fn advance_str(self, s: &str) -> Self {
        let s_len = s.len();
        Self {
            column: self.column + s_len as u16,
            pos: self.pos + s_len as u64,
            ..self
        }
    }
}

impl State {
    pub(super) fn add_char(&mut self, c: char) -> ProcResult {
        match c {
            '@' | '#' | '^' => return ProcResult::Error(self.illegal_character_error(c)),
            _ => {}
        }

        let (memory, location, tokens) = match self.next_state(c) {
            Ok((m, l, t)) => (m, l, t),
            Err(e) => return ProcResult::Error(e),
        };

        self.memory = Some(memory);
        self.location = location;

        if tokens.is_empty() {
            ProcResult::Empty
        } else {
            ProcResult::Tokens(tokens)
        }
    }

    fn next_state(&mut self, c: char) -> Result<(ClipBoard, Location, Vec<Token>)> {
        let memory = self.memory.take().expect("memory not set");
        match memory {
            ClipBoard::Empty => add_char_empty(c, self.location),
            m @ ClipBoard::Identifier(_) => add_char_identifier(m, c, self.location),
            ClipBoard::Bang => add_char_bang(c, self.location),
            ClipBoard::Less => add_char_less(c, self.location),
            ClipBoard::Greater => add_char_greater(c, self.location),
            ClipBoard::Equal => add_char_equal(c, self.location),
            ClipBoard::Division => add_char_division(c, self.location),
            ClipBoard::Comment => add_char_comment(c, self.location),
            ClipBoard::String(s) => add_char_string(s, c, self.location),
            ClipBoard::NumberPreDot(n) => add_char_num_pre(n, c, self.location),
            ClipBoard::NumberPostDot(n) => add_char_num_post(n, c, self.location),
        }
    }

    pub(super) fn extract_token(&mut self) -> Result<Option<Token>> {
        let clipboard = self.memory.take().expect("memory not set");
        self.memory = Some(ClipBoard::Empty);
        let opt_token = match clipboard {
            ClipBoard::Empty | ClipBoard::Comment => None,
            ClipBoard::Identifier(chars) => {
                let location = self.location;
                self.location = self.location.advance_str(&chars);
                Some(Token::identifier(chars, location))
            }
            ClipBoard::Bang => self.extract_one_two_char(TokenType::Bang),
            ClipBoard::Less => self.extract_one_two_char(TokenType::Less),
            ClipBoard::Greater => self.extract_one_two_char(TokenType::Greater),
            ClipBoard::Equal => self.extract_one_two_char(TokenType::Equal),
            ClipBoard::Division => self.extract_one_two_char(TokenType::Division),
            ClipBoard::String(s) => return Err(self.unfinished_string_error(s)),
            ClipBoard::NumberPreDot(n) => self.extract_num(n),
            ClipBoard::NumberPostDot(n) => self.extract_num(n),
        };
        Ok(opt_token)
    }

    fn extract_one_two_char(&mut self, t_type: TokenType) -> Option<Token> {
        let location = self.location;
        self.location = self.location.advance_col();
        Some(Token::one_two_char(t_type, location))
    }

    fn extract_num(&mut self, n: String) -> Option<Token> {
        let location = self.location;
        self.location = self.location.advance_str(&n);
        Some(Token::number(n, location))
    }

    fn unfinished_string_error(&self, s: String) -> anyhow::Error {
        anyhow::anyhow!(
            "string starting at line {line}, column {column} not completed by closing '\"'; string start: {s}",
            line = self.location.line,
            column = self.location.column
        )
    }

    fn illegal_character_error(&self, c: char) -> anyhow::Error {
        anyhow::anyhow!(
            "found illegal character: '{c}' at line {line}, column {col}",
            line = self.location.line,
            col = self.location.column
        )
    }

    pub(super) fn eof(&self) -> Token {
        Token::eof(self.location)
    }
}

fn add_char_comment(c: char, cur_location: Location) -> Result<(ClipBoard, Location, Vec<Token>)> {
    match c {
        '\n' => Ok((ClipBoard::Empty, cur_location.advance_line(), vec![])),
        _ => Ok((ClipBoard::Comment, cur_location.advance_col(), vec![])),
    }
}

fn add_char_string(
    mut s: String,
    c: char,
    cur_location: Location,
) -> Result<(ClipBoard, Location, Vec<Token>)> {
    match c {
        '"' => Ok((
            ClipBoard::Empty,
            cur_location.advance_str(&s).advance_col().advance_col(),
            vec![Token::string(s, cur_location)],
        )),
        _ => {
            s.push(c);
            Ok((ClipBoard::String(s), cur_location, vec![]))
        }
    }
}

fn state_changed(cur_state: &ClipBoard, c: char) -> bool {
    match c {
        ' ' | '\t' | '\r' => true,
        '!' | '<' | '>' | '=' | '/' | '"' => true,
        '(' | ')' | '{' | '}' | '-' | ';' | '*' | '+' | ',' | '.' => true,
        '\n' => true,
        _ if c.is_digit(10) => match cur_state {
            ClipBoard::NumberPostDot(_) | ClipBoard::NumberPreDot(_) => false,
            _ => true,
        },
        _ => match cur_state {
            ClipBoard::Identifier(_) => false,
            _ => true,
        },
    }
}

fn switch_state(
    c: char,
    location: Location,
    extracted: Option<Token>,
) -> Result<(ClipBoard, Location, Vec<Token>)> {
    let (next_state, location, mut switch_tokens) = match c {
        ' ' | '\t' | '\r' => (ClipBoard::Empty, location.advance_col(), vec![]),
        '!' | '<' | '>' | '=' | '/' | '"' => {
            switch_state_one_two_char(state_change_token_type(c), location)
        }
        '(' | ')' | '{' | '}' | '-' | ';' | '*' | '+' | ',' | '.' => {
            switch_state_empty_one_char(c, location)
        }
        '\n' => (ClipBoard::Empty, location.advance_line(), vec![]),
        _ if c.is_digit(10) => (ClipBoard::NumberPreDot(c.into()), location, vec![]),
        _ => (ClipBoard::Identifier(c.into()), location, vec![]),
    };

    let mut tokens = if let Some(e) = extracted {
        vec![e]
    } else {
        vec![]
    };
    tokens.append(&mut switch_tokens);
    Ok((next_state, location, tokens))
}

fn switch_state_one_two_char(
    new_state: ClipBoard,
    cur_location: Location,
) -> (ClipBoard, Location, Vec<Token>) {
    (new_state, cur_location, vec![])
}

fn switch_state_empty_one_char(
    c: char,
    cur_location: Location,
) -> (ClipBoard, Location, Vec<Token>) {
    let token = one_char_token(c, cur_location);
    (ClipBoard::Empty, cur_location.advance_col(), vec![token])
}

fn one_char_token(c: char, cur_location: Location) -> Token {
    match c {
        '(' => Token::one_char(TokenType::ParenLeft, cur_location),
        ')' => Token::one_char(TokenType::ParenRight, cur_location),
        '{' => Token::one_char(TokenType::BraceLeft, cur_location),
        '}' => Token::one_char(TokenType::BraceRight, cur_location),
        '-' => Token::one_char(TokenType::Minus, cur_location),
        '+' => Token::one_char(TokenType::Plus, cur_location),
        ';' => Token::one_char(TokenType::Semicolon, cur_location),
        '*' => Token::one_char(TokenType::Star, cur_location),
        ',' => Token::one_char(TokenType::Comma, cur_location),
        '.' => Token::one_char(TokenType::Dot, cur_location),
        _ => unreachable!(),
    }
}

fn state_change_token_type(c: char) -> ClipBoard {
    match c {
        '!' => ClipBoard::Bang,
        '<' => ClipBoard::Less,
        '>' => ClipBoard::Greater,
        '=' => ClipBoard::Equal,
        '/' => ClipBoard::Division,
        '"' => ClipBoard::String(String::new()),
        _ => unreachable!(),
    }
}
