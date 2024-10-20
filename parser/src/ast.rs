use std::fmt::Display;

use token::Token;

use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Num(u64),
}

impl TryFrom<Token> for NodeKind {
    type Error = Error;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Add => Ok(NodeKind::Add),
            Token::Sub => Ok(NodeKind::Sub),
            Token::Mul => Ok(NodeKind::Mul),
            Token::Div => Ok(NodeKind::Div),
            Token::Equal => Ok(NodeKind::Equal),
            Token::NotEqual => Ok(NodeKind::NotEqual),
            Token::LessThan => Ok(NodeKind::LessThan),
            Token::LessThanOrEqual => Ok(NodeKind::LessThanOrEqual),
            Token::GreaterThan => Ok(NodeKind::GreaterThan),
            Token::GreaterThanOrEqual => Ok(NodeKind::GreaterThanOrEqual),
            Token::Num(n) => Ok(NodeKind::Num(n)),
            _ => Err(Error::InvalidToken(value)),
        }
    }
}

impl Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeKind::Add => write!(f, "Add"),
            NodeKind::Sub => write!(f, "Sub"),
            NodeKind::Mul => write!(f, "Mul"),
            NodeKind::Div => write!(f, "Div"),
            NodeKind::Equal => write!(f, "Equal"),
            NodeKind::NotEqual => write!(f, "NotEqual"),
            NodeKind::LessThan => write!(f, "LessThan"),
            NodeKind::LessThanOrEqual => write!(f, "LessThanOrEqual"),
            NodeKind::GreaterThan => write!(f, "GreaterThan"),
            NodeKind::GreaterThanOrEqual => write!(f, "GreaterThanOrEqual"),
            NodeKind::Num(n) => write!(f, "Num({n})"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
}

impl Node {
    pub fn new(kind: NodeKind, lhs: Option<Box<Node>>, rhs: Option<Box<Node>>) -> Self {
        Self { kind, lhs, rhs }
    }

    pub fn new_num(n: u64) -> Self {
        Self {
            kind: NodeKind::Num(n),
            lhs: None,
            rhs: None,
        }
    }

    pub fn num_from_token(token: Token) -> Result<Node, Error> {
        if let Token::Num(n) = token {
            Ok(Node {
                kind: NodeKind::Num(n),
                lhs: None,
                rhs: None,
            })
        } else {
            Err(Error::UnexpectedToken(Token::Num(0), token))
        }
    }

    fn num(&self) -> Option<u64> {
        match self.kind {
            NodeKind::Num(n) => Some(n),
            _ => None,
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let NodeKind::Num(_) = self.kind {
            write!(f, "Node({})", self.kind)
        } else {
            let mut s = format!("Node({}", self.kind);
            if let Some(lhs) = &self.lhs {
                s = s + &format!(", lhs={}", lhs);
            } else {
                s += ", lhs=()";
            }
            if let Some(rhs) = &self.rhs {
                s = s + &format!(", rhs={}", rhs);
            } else {
                s += ", rhs=()";
            }
            s += ")";
            write!(f, "{}", s)
        }
    }
}
