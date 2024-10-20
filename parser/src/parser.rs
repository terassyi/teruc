use std::collections::VecDeque;

use token::Token;

use crate::{
    ast::{Node, NodeKind},
    error::Error,
};

/*
expr       = equality
equality   = relational ("==" relational | "!=" relational)*
relational = add ("<" add | "<=" add | ">" add | ">=" add)*
add        = mul ("+" mul | "-" mul)*
mul        = unary ("*" unary | "/" unary)*
unary      = ("+" | "-")? primary
primary    = num | "(" expr ")"
 */

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

    // expr = equality
    fn expr(&mut self) -> Result<Node, Error> {
        self.equality()
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

    // equality = relational ("==" relational | "!=" relational)*
    fn equality(&mut self) -> Result<Node, Error> {
        let mut node = self.relational()?;
        while let Some(p) = self.tokens.front() {
            match p {
                Token::Equal => {
                    self.tokens.pop_front(); // consume
                    node = Node::new(
                        NodeKind::Equal,
                        Some(Box::new(node)),
                        Some(Box::new(self.relational()?)),
                    )
                }
                Token::NotEqual => {
                    self.tokens.pop_front(); // consume
                    node = Node::new(
                        NodeKind::NotEqual,
                        Some(Box::new(node)),
                        Some(Box::new(self.relational()?)),
                    )
                }
                _ => return Ok(node),
            }
        }
        Ok(node)
    }

    // relational = add ("<" add | "<=" add | ">" add | ">=" add)*
    fn relational(&mut self) -> Result<Node, Error> {
        let mut node = self.add()?;
        while let Some(p) = self.tokens.front() {
            match p {
                Token::LessThan => {
                    self.tokens.pop_front(); // consume
                    node = Node::new(
                        NodeKind::LessThan,
                        Some(Box::new(node)),
                        Some(Box::new(self.add()?)),
                    )
                }
                // GreaterThan(lhs, rhs) is translate to LessThan(rhs, lhs)
                Token::GreaterThan => {
                    self.tokens.pop_front(); // consume

                    // node = Node::new(
                    //     NodeKind::GreaterThan,
                    //     Some(Box::new(node)),
                    //     Some(Box::new(self.add()?)),
                    // )
                    node = Node::new(
                        NodeKind::LessThan,
                        Some(Box::new(self.add()?)),
                        Some(Box::new(node)),
                    )
                }
                Token::LessThanOrEqual => {
                    self.tokens.pop_front(); // consume
                    node = Node::new(
                        NodeKind::LessThanOrEqual,
                        Some(Box::new(node)),
                        Some(Box::new(self.add()?)),
                    )
                }
                // GreaterThanOrEqual(lhs, rhs) is translate to LessThanOrEqual(rhs, lhs)
                Token::GreaterThanOrEqual => {
                    self.tokens.pop_front(); // consume

                    // node = Node::new(
                    //     NodeKind::GreaterThanOrEqual,
                    //     Some(Box::new(node)),
                    //     Some(Box::new(self.add()?)),
                    // )
                    node = Node::new(
                        NodeKind::LessThanOrEqual,
                        Some(Box::new(self.add()?)),
                        Some(Box::new(node)),
                    )
                }
                _ => return Ok(node),
            }
        }
        Ok(node)
    }

    // add = mul ("+" mul | "-" mul)*
    fn add(&mut self) -> Result<Node, Error> {
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
        case(
            vec![Token::Num(2), Token::Equal, Token::Num(2)],
            Node::new(NodeKind::Equal, Some(Box::new(Node::new_num(2))), Some(Box::new(Node::new_num(2))))
        ),
        case(
            vec![Token::Num(2), Token::LessThan, Token::Num(2)],
            Node::new(NodeKind::LessThan, Some(Box::new(Node::new_num(2))), Some(Box::new(Node::new_num(2))))
        ),
        case(
            vec![Token::Num(2), Token::Equal, Token::Num(2), Token::LessThanOrEqual, Token::Num(1)],
            Node::new(NodeKind::Equal,Some(Box::new(Node::new_num(2))), Some(Box::new(Node::new(NodeKind::LessThanOrEqual, Some(Box::new(Node::new_num(2))), Some(Box::new(Node::new_num(1)))))))
        ),
        case(
            vec![Token::Num(2), Token::Equal, Token::Num(2), Token::GreaterThan, Token::Num(1)],
            Node::new(NodeKind::Equal,Some(Box::new(Node::new_num(2))), Some(Box::new(Node::new(NodeKind::LessThan, Some(Box::new(Node::new_num(1))), Some(Box::new(Node::new_num(2)))))))
        ),
    )]
    fn test_parser_parse(input: Vec<Token>, expect: Node) {
        let mut parser = Parser::new(input);
        let node = parser.parse().unwrap();
        assert_eq!(expect, node);
    }
}
