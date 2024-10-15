use std::fmt::Display;

pub mod reserved;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Add,
    Sub,
    Mul,
    Div,
    OpenParen,
    CloseParen,
    Num(u64),
    Eof,
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
            Self::Num(n) => write!(f, "num({n})"),
            Self::Eof => write!(f, "EOF"),
        }
    }
}
