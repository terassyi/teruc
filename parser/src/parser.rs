use std::collections::VecDeque;

use token::Token;

use crate::{
    ast::{Node, NodeKind},
    error::Error,
};

#[derive(Debug)]
pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into(),
        }
    }

    pub fn parse(&mut self) -> Result<Node, Error> {
        self.expr()
    }

    // primary = num | "(" expr ")"
    fn primary(&mut self) -> Result<Node, Error> {
        if let Some(t) = self.tokens.front() {
            if t.eq(&Token::OpenParen) {
                // continue to parse expr
                self.tokens.pop_front(); // consume
                let node = self.expr()?;
                if let Some(t) = self.tokens.front() {
                    if t.eq(&Token::CloseParen) {
                        self.tokens.pop_front(); // consume
                        Ok(node)
                    } else {
                        Err(Error::UnexpectedToken(Token::CloseParen, *t))
                    }
                } else {
                    Err(Error::InvalidTermination)
                }
            } else {
                // expect number
                let node = Node::num_from_token(*t)?;
                self.tokens.pop_front(); // consume
                Ok(node)
            }
        } else {
            Err(Error::InvalidTermination)
        }
    }

    // expr = mul ("+" mul | "-" mul)*
    fn expr(&mut self) -> Result<Node, Error> {
        let mut node = self.mul()?;

        while let Some(p) = self.tokens.front() {
            match p {
                Token::Add => {
                    self.tokens.pop_front(); // consume
                    node = Node::new(
                        NodeKind::Add,
                        Some(Box::new(node)),
                        Some(Box::new(self.mul()?)),
                    )
                }
                Token::Sub => {
                    self.tokens.pop_front(); // consume
                    node = Node::new(
                        NodeKind::Sub,
                        Some(Box::new(node)),
                        Some(Box::new(self.mul()?)),
                    )
                }
                _ => return Ok(node),
            }
        }
        Ok(node)
    }

    // mul = unary ("*" unary | "/" unary)*
    fn mul(&mut self) -> Result<Node, Error> {
        let mut node = self.unary()?;

        while let Some(p) = self.tokens.front() {
            match p {
                Token::Mul => {
                    self.tokens.pop_front(); // consume
                    node = Node::new(
                        NodeKind::Mul,
                        Some(Box::new(node)),
                        Some(Box::new(self.unary()?)),
                    );
                }
                Token::Div => {
                    self.tokens.pop_front(); // consume
                    node = Node::new(
                        NodeKind::Div,
                        Some(Box::new(node)),
                        Some(Box::new(self.unary()?)),
                    );
                }
                _ => return Ok(node),
            }
        }

        Ok(node)
    }

    // unary = ("+" | "-")? primary
    fn unary(&mut self) -> Result<Node, Error> {
        if let Some(p) = self.tokens.front() {
            match p {
                Token::Add => {
                    self.tokens.pop_front();
                    self.primary()
                }
                Token::Sub => {
                    self.tokens.pop_front();
                    Ok(Node::new(
                        NodeKind::Sub,
                        Some(Box::new(Node::new_num(0))),
                        Some(Box::new(self.primary()?)),
                    ))
                }
                _ => self.primary(),
            }
        } else {
            Err(Error::InvalidTermination)
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use token::Token;

    use crate::ast::{Node, NodeKind};

    use super::Parser;

    #[rstest(
        input,
        expect,
        case(
            vec![Token::Num(0)],
            Node::new(NodeKind::Num(0), None, None)
        ),
        case(
            vec![Token::Num(0), Token::Add, Token::Num(1)],
            Node::new(NodeKind::Add, Some(Box::new(Node::new_num(0))), Some(Box::new(Node::new_num(1))))
        ),
        case(
            vec![Token::Num(0), Token::Add, Token::OpenParen, Token::Num(2), Token::Add, Token::Num(1), Token::CloseParen],
            Node::new(NodeKind::Add, Some(Box::new(Node::new_num(0))), Some(Box::new(Node::new(NodeKind::Add, Some(Box::new(Node::new_num(2))), Some(Box::new(Node::new_num(1)))))))
        ),
        case(
            vec![Token::Num(0), Token::Add, Token::OpenParen, Token::Num(2), Token::Add, Token::Num(1), Token::CloseParen, Token::Mul, Token::Num(1)],
            Node::new(NodeKind::Add, Some(Box::new(Node::new_num(0))), Some(Box::new(Node::new(NodeKind::Mul, Some(Box::new(Node::new(NodeKind::Add, Some(Box::new(Node::new_num(2))), Some(Box::new(Node::new_num(1)))))), Some(Box::new(Node::new_num(1)))))))
        ),
        case(
            vec![Token::Sub, Token::Num(2)],
            Node::new(NodeKind::Sub, Some(Box::new(Node::new_num(0))), Some(Box::new(Node::new_num(2))))
        ),
    )]
    fn test_parser_parse(input: Vec<Token>, expect: Node) {
        let mut parser = Parser::new(input);
        let node = parser.parse().unwrap();
        assert_eq!(expect, node);
    }
}
