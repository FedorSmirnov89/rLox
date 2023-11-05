use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq)]
pub struct Token {
    pub t_type: TokenType,
    location: Location,
}

impl Token {
    fn new(t_type: TokenType, location: Location) -> Self {
        Self { t_type, location }
    }

    pub fn identifier(chars: impl Into<String>, location: Location) -> Self {
        let chars = chars.into();
        dbg!(&chars);

        let t_type = match chars.as_str() {
            "print" => TokenType::PRINT,
            "and" => TokenType::AND,
            "class" => TokenType::CLASS,
            "else" => TokenType::ELSE,
            "false" => TokenType::FALSE,
            "for" => TokenType::FOR,
            "fun" => TokenType::FUN,
            "if" => TokenType::IF,
            "nil" => TokenType::NIL,
            "or" => TokenType::OR,
            "return" => TokenType::RETURN,
            "super" => TokenType::SUPER,
            "this" => TokenType::THIS,
            "true" => TokenType::TRUE,
            "var" => TokenType::VAR,
            "while" => TokenType::WHILE,
            _ => TokenType::Identifier(chars),
        };

        Self { t_type, location }
    }

    pub fn string(chars: impl Into<String>, location: Location) -> Self {
        let chars = chars.into();
        Self {
            t_type: TokenType::String(chars),
            location,
        }
    }

    pub fn number(n: impl Into<String>, location: Location) -> Self {
        let n = n.into();
        let n = n.parse().expect("string {n} cannot be parsed as a number");
        Self {
            t_type: TokenType::Number(n),
            location,
        }
    }

    pub fn one_char(t_type: TokenType, location: Location) -> Self {
        if t_type.is_one_char() {
            Self::new(t_type, location)
        } else {
            unreachable!("Should only be called for one-char tokens");
        }
    }

    pub fn one_two_char(t_type: TokenType, location: Location) -> Self {
        if t_type.is_one_two_char() {
            Self::new(t_type, location)
        } else {
            unreachable!("Should only be called for one-two-char tokens");
        }
    }

    pub fn eof(location: Location) -> Self {
        Self {
            t_type: TokenType::EOF,
            location,
        }
    }

    pub fn t_type(&self) -> &TokenType {
        &self.t_type
    }

    pub fn location(&self) -> Location {
        self.location
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Location {
    pub line: u64,
    pub column: u16,
    pub pos: u64,
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

#[derive(PartialEq, Clone)]
pub enum TokenType {
    // Single-character tokens
    BraceLeft,
    BraceRight,
    ParenLeft,
    ParenRight,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,

    // One or two character tokens
    EqualEqual,
    BangEqual,
    LessEqual,
    GreaterEqual,
    Equal,
    Bang,
    Less,
    Greater,
    Division,

    // Literals
    Identifier(String),
    Number(f64),
    String(String),

    // Keywords
    PRINT,
    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    FUN,
    IF,
    NIL,
    OR,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    // EOF
    EOF,
}

impl TokenType {
    fn is_one_char(&self) -> bool {
        match self {
            TokenType::BraceLeft => true,
            TokenType::BraceRight => true,
            TokenType::ParenLeft => true,
            TokenType::ParenRight => true,
            TokenType::Comma => true,
            TokenType::Dot => true,
            TokenType::Minus => true,
            TokenType::Plus => true,
            TokenType::Semicolon => true,
            TokenType::Star => true,
            _ => false,
        }
    }

    fn is_one_two_char(&self) -> bool {
        match self {
            TokenType::EqualEqual => true,
            TokenType::BangEqual => true,
            TokenType::LessEqual => true,
            TokenType::GreaterEqual => true,
            TokenType::Equal => true,
            TokenType::Bang => true,
            TokenType::Less => true,
            TokenType::Greater => true,
            TokenType::Division => true,

            _ => false,
        }
    }
}

impl Debug for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt(self, f)
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt(self, f)
    }
}

fn fmt(t: &TokenType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let tmp;
    let s = match t {
        TokenType::BraceLeft => "{",
        TokenType::BraceRight => "}",
        TokenType::ParenLeft => "(",
        TokenType::ParenRight => ")",
        TokenType::Comma => ",",
        TokenType::Dot => ".",
        TokenType::Minus => "-",
        TokenType::Plus => "+",
        TokenType::Semicolon => ";",
        TokenType::Star => "*",

        TokenType::Equal => "=",
        TokenType::Bang => "!",
        TokenType::Less => "<",
        TokenType::Greater => ">",
        TokenType::EqualEqual => "==",
        TokenType::BangEqual => "!=",
        TokenType::LessEqual => "<=",
        TokenType::GreaterEqual => ">=",
        TokenType::Division => "/",

        TokenType::Identifier(id) => {
            tmp = format!("ID: {id}");
            &tmp
        }
        TokenType::EOF => "EOF",
        TokenType::Number(n) => {
            tmp = format!("NUM: {num}", num = n.to_string());
            &tmp
        }
        TokenType::String(s) => {
            tmp = format!("String: '{s}'");
            &tmp
        }

        TokenType::PRINT => "KW: PRINT",
        TokenType::AND => "KW: AND",
        TokenType::CLASS => "KW: CLASS",
        TokenType::ELSE => "KW: ELSE",
        TokenType::FALSE => "KW: FALSE",
        TokenType::FOR => "KW: FOR",
        TokenType::FUN => "KW: FUN",
        TokenType::IF => "KW: IF",
        TokenType::NIL => "KW: NIL",
        TokenType::OR => "KW: OR",
        TokenType::RETURN => "KW: RETURN",
        TokenType::SUPER => "KW: SUPER",
        TokenType::THIS => "KW: THIS",
        TokenType::TRUE => "KW: TRUE",
        TokenType::VAR => "KW: VAR",
        TokenType::WHILE => "KW: WHILE",
    };
    write!(f, "{s}")
}
