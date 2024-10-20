use std::fmt::Display;

pub mod reserved;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Num(u64),           // number
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
            Self::Num(n) => write!(f, "num({n})"),
            Self::Eof => write!(f, "EOF"),
        }
    }
}
