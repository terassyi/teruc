use std::fmt::Display;

use reserved::RESERVED_CHARS;

pub mod reserved;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Add,                // +
    Sub,                // -
    Mul,                // *
    Div,                // /
    OpenParen,          // (
    CloseParen,         // )
    LessThan,           // <
    GreaterThan,        // >
    LessThanOrEqual,    // <=
    GreaterThanOrEqual, // >=
    Equal,              // ==
    NotEqual,           // !=
    Assignment,         // = // not used yet
    Not,                // ! // not used yet
    Semicolon,          // ;
    Num(u64),           // number
    Identifier(String), // identifier
    Eof,                // EOF
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::OpenParen => write!(f, "("),
            Self::CloseParen => write!(f, ")"),
            Self::LessThan => write!(f, "<"),
            Self::GreaterThan => write!(f, ">"),
            Self::LessThanOrEqual => write!(f, "<="),
            Self::GreaterThanOrEqual => write!(f, ">="),
            Self::Equal => write!(f, "=="),
            Self::NotEqual => write!(f, "!="),
            Self::Assignment => write!(f, "="),
            Self::Not => write!(f, "!"),
            Self::Semicolon => write!(f, ";"),
            Self::Num(n) => write!(f, "num({n})"),
            Self::Identifier(i) => write!(f, "identifier({i})"),
            Self::Eof => write!(f, "EOF"),
        }
    }
}

pub fn is_reserved(c: char) -> bool {
    RESERVED_CHARS.iter().any(|p| c == *p)
}
