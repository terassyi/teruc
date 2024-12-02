use std::collections::VecDeque;

use token::Token;

use crate::{
    ast::{LocalVars, Node, NodeKind},
    error::Error,
};

/*
program    = stmt*
stmt       = expr ";"
                | "{" stmt* "}"
                | "if" "(" expr ")" stmt ("else" stmt)?
                | "while" "(" expr ")" stmt
                | "for" "(" expr? ";" expr? ";" expr? ")" stmt
                | "return" expr ";"
expr       = assign
assign     = equality ("=" assign)?
equality   = relational ("==" relational | "!=" relational)*
relational = add ("<" add | "<=" add | ">" add | ">=" add)*
add        = mul ("+" mul | "-" mul)*
mul        = unary ("*" unary | "/" unary)*
unary      = ("+" | "-")? primary
primary    = num
                | ident ("(" num* ")")?
                | "(" expr ")"
 */

#[derive(Debug)]
pub struct Parser {
    tokens: VecDeque<Token>,
    pub nodes: Vec<Node>,
    local_val_offset: u32,
    local_vars: LocalVars,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into(),
            nodes: Vec::new(),
            local_val_offset: 1,
            local_vars: LocalVars::new(),
        }
    }

    pub fn parse(&mut self) -> Result<(), Error> {
        self.program()
    }

    // program = stmt*
    fn program(&mut self) -> Result<(), Error> {
        while let Some(_t) = self.tokens.front() {
            let node = self.stmt()?;
            self.nodes.push(node);
        }
        Ok(())
    }

    /* stmt = expr ";"
                | "{" stmt* "}"
                | "if" "(" expr ")" stmt ("else" stmt)?
                | "while" "(" expr ")" stmt
                | "for" "(" expr? ";" expr? ";" expr? ")" stmt
                | "return" expr ";"
    */
    fn stmt(&mut self) -> Result<Node, Error> {
        let node = if let Some(t) = self.tokens.front() {
            match t {
                Token::Return => {
                    self.consume(Token::Return)?;
                    let node = Node::new(NodeKind::Return, Some(Box::new(self.expr()?)), None);
                    self.consume(Token::Semicolon)?;
                    node
                }
                Token::If => {
                    self.consume(Token::If)?;
                    self.consume(Token::OpenParen)?;
                    let lhs = self.expr()?;
                    self.consume(Token::CloseParen)?;
                    let mut rhs = self.stmt()?;

                    if let Some(t) = self.tokens.front() {
                        if t.eq(&Token::Else) {
                            // else
                            self.consume(Token::Else)?;
                            rhs = Node::new(
                                NodeKind::Else,
                                Some(Box::new(rhs)),
                                Some(Box::new(self.stmt()?)),
                            );
                        }
                    }
                    Node::new(NodeKind::If, Some(Box::new(lhs)), Some(Box::new(rhs)))
                }
                Token::While => {
                    self.consume(Token::While)?;
                    self.consume(Token::OpenParen)?;
                    let lhs = self.expr()?;
                    self.consume(Token::CloseParen)?;
                    let rhs = self.stmt()?;

                    Node::new(NodeKind::While, Some(Box::new(lhs)), Some(Box::new(rhs)))
                }
                Token::For => {
                    // for (A; B; C) D
                    self.consume(Token::For)?;
                    self.consume(Token::OpenParen)?;
                    // A
                    let lhs = match self.tokens.front() {
                        Some(Token::Semicolon) => None,
                        Some(_) => Some(self.expr()?),
                        None => return Err(Error::InvalidTermination),
                    }
                    .map(Box::new);
                    self.consume(Token::Semicolon)?;

                    // B
                    let if_lhs = match self.tokens.front() {
                        Some(Token::Semicolon) => None,
                        Some(_) => Some(self.expr()?),
                        None => return Err(Error::InvalidTermination),
                    }
                    .map(Box::new);
                    self.consume(Token::Semicolon)?;

                    // C
                    let expr = match self.tokens.front() {
                        Some(Token::Semicolon) => None,
                        Some(_) => Some(self.expr()?),
                        None => return Err(Error::InvalidTermination),
                    };
                    self.consume(Token::CloseParen)?;

                    // D
                    let stmt = self.stmt()?;
                    let mut block_nodes = vec![stmt];
                    if let Some(node) = expr {
                        block_nodes.push(node);
                    }

                    let rhs = Node::new(
                        NodeKind::If,
                        if_lhs,
                        Some(Box::new(Node::new(
                            NodeKind::Block(block_nodes),
                            None,
                            None,
                        ))),
                    );
                    Node::new(NodeKind::For, lhs, Some(Box::new(rhs)))
                }
                Token::OpenBrace => {
                    // block
                    self.consume(Token::OpenBrace)?;
                    let mut nodes = Vec::new();
                    while let Some(t) = self.tokens.front() {
                        if t.eq(&Token::CloseBrace) {
                            // end of block
                            self.consume(Token::CloseBrace)?;
                            break;
                        } else {
                            let node = self.stmt()?;
                            nodes.push(node);
                        }
                    }
                    Node::new(NodeKind::Block(nodes), None, None)
                }
                _ => {
                    let node = self.expr()?;
                    self.consume(Token::Semicolon)?;
                    node
                }
            }
        } else {
            // let node = self.expr()?;
            // self.consume(Token::Semicolon)?;
            // node
            return Err(Error::InvalidTermination);
        };

        // if let Some(t) = self.tokens.front() {
        //     if t.ne(&Token::Semicolon) {
        //         return Err(Error::UnexpectedToken(Token::Semicolon, t.clone()));
        //     }
        // }
        // let _ = self.tokens.pop_front();
        Ok(node)
    }

    // expr = assign
    fn expr(&mut self) -> Result<Node, Error> {
        self.assign()
    }

    // assign = equality ("=" assign)?
    fn assign(&mut self) -> Result<Node, Error> {
        let mut node = self.equality()?;

        if let Some(t) = self.tokens.front() {
            if t.eq(&Token::Assignment) {
                self.tokens.pop_front();
                node = Node::new(
                    NodeKind::Assignment,
                    Some(Box::new(node)),
                    Some(Box::new(self.assign()?)),
                );
            }
        }

        Ok(node)
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
                    self.consume(Token::Add)?;
                    node = Node::new(
                        NodeKind::Add,
                        Some(Box::new(node)),
                        Some(Box::new(self.mul()?)),
                    )
                }
                Token::Sub => {
                    self.consume(Token::Sub)?;
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
                    self.consume(Token::Mul)?;
                    node = Node::new(
                        NodeKind::Mul,
                        Some(Box::new(node)),
                        Some(Box::new(self.unary()?)),
                    );
                }
                Token::Div => {
                    self.consume(Token::Div)?;
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
                    self.consume(Token::Add)?;
                    self.primary()
                }
                Token::Sub => {
                    self.consume(Token::Sub)?;
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

    /*
    primary = num
                | ident ("(" ")")?
                | "(" expr ")"
    */
    fn primary(&mut self) -> Result<Node, Error> {
        if let Some(t) = self.tokens.front() {
            if t.eq(&Token::OpenParen) {
                // continue to parse expr
                self.consume(Token::OpenParen)?;
                let node = self.expr()?;
                if let Some(t) = self.tokens.front() {
                    if t.eq(&Token::CloseParen) {
                        self.consume(Token::CloseParen)?;
                        Ok(node)
                    } else {
                        Err(Error::UnexpectedToken(Token::CloseParen, t.clone()))
                    }
                } else {
                    Err(Error::InvalidTermination)
                }
            } else if let Some(t) = self.tokens.pop_front() {
                match t {
                    Token::Num(n) => Ok(Node::new_num(n)),
                    Token::Identifier(s) => {
                        // func
                        if let Some(tt) = self.tokens.front() {
                            if tt.eq(&Token::OpenParen) {
                                self.consume(Token::OpenParen)?;
                                // args
                                let mut args = vec![];
                                while let Some(ttt) = self.tokens.pop_front() {
                                    if ttt.eq(&Token::CloseParen) {
                                        break;
                                    }
                                    match ttt {
                                        Token::CloseParen => break,
                                        Token::Num(n) => {
                                            args.push(Node::new_num(n));
                                        }
                                        _ => {
                                            return Err(Error::UnexpectedToken(Token::Num(0), ttt))
                                        }
                                    }
                                }
                                if args.len() > 6 {
                                    return Err(Error::TooManyArguments(s));
                                }
                                return Ok(Node::new(NodeKind::Func(s, args), None, None));
                                // TODO: implement lhs and rhs
                            }
                        }
                        // ident
                        if let Some(offset) = self.find_local_var(&s) {
                            Ok(Node::new_local_var(s, offset))
                        } else {
                            let offset = self.local_val_offset * 8;
                            self.local_val_offset += 1;
                            self.local_vars.insert(s.clone(), offset);
                            Ok(Node::new_local_var(s, offset))
                        }
                    }
                    _ => Err(Error::InvalidToken(t)),
                }
            } else {
                Err(Error::InvalidTermination)
            }
        } else {
            Err(Error::InvalidTermination)
        }
    }

    fn find_local_var(&self, name: &str) -> Option<u32> {
        self.local_vars.get(name).copied()
    }

    fn consume(&mut self, expect: Token) -> Result<(), Error> {
        if let Some(t) = self.tokens.front() {
            if t.eq(&expect) {
                self.tokens.pop_front();
                Ok(())
            } else {
                Err(Error::UnexpectedToken(expect, t.clone()))
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
            vec![Token::Num(0), Token::Semicolon],
            vec![Node::new(NodeKind::Num(0), None, None)]
        ),
        case(
            vec![Token::Num(0), Token::Add, Token::Num(1), Token::Semicolon],
            vec![Node::new(NodeKind::Add, Some(Box::new(Node::new_num(0))), Some(Box::new(Node::new_num(1))))]
        ),
        case(
            vec![Token::Num(0), Token::Add, Token::OpenParen, Token::Num(2), Token::Add, Token::Num(1), Token::CloseParen, Token::Semicolon],
            vec![Node::new(NodeKind::Add, Some(Box::new(Node::new_num(0))), Some(Box::new(Node::new(NodeKind::Add, Some(Box::new(Node::new_num(2))), Some(Box::new(Node::new_num(1)))))))]
        ),
        case(
            vec![Token::Num(0), Token::Add, Token::OpenParen, Token::Num(2), Token::Add, Token::Num(1), Token::CloseParen, Token::Mul, Token::Num(1), Token::Semicolon],
            vec![Node::new(NodeKind::Add, Some(Box::new(Node::new_num(0))), Some(Box::new(Node::new(NodeKind::Mul, Some(Box::new(Node::new(NodeKind::Add, Some(Box::new(Node::new_num(2))), Some(Box::new(Node::new_num(1)))))), Some(Box::new(Node::new_num(1)))))))]
        ),
        case(
            vec![Token::Sub, Token::Num(2), Token::Semicolon],
            vec![Node::new(NodeKind::Sub, Some(Box::new(Node::new_num(0))), Some(Box::new(Node::new_num(2))))]
        ),
        case(
            vec![Token::Num(2), Token::Equal, Token::Num(2), Token::Semicolon],
            vec![Node::new(NodeKind::Equal, Some(Box::new(Node::new_num(2))), Some(Box::new(Node::new_num(2))))]
        ),
        case(
            vec![Token::Num(2), Token::LessThan, Token::Num(2), Token::Semicolon],
            vec![Node::new(NodeKind::LessThan, Some(Box::new(Node::new_num(2))), Some(Box::new(Node::new_num(2))))]
        ),
        case(
            vec![Token::Num(2), Token::Equal, Token::Num(2), Token::LessThanOrEqual, Token::Num(1), Token::Semicolon],
            vec![Node::new(NodeKind::Equal,Some(Box::new(Node::new_num(2))), Some(Box::new(Node::new(NodeKind::LessThanOrEqual, Some(Box::new(Node::new_num(2))), Some(Box::new(Node::new_num(1)))))))]
        ),
        case(
            vec![Token::Num(2), Token::Equal, Token::Num(2), Token::GreaterThan, Token::Num(1), Token::Semicolon],
            vec![Node::new(NodeKind::Equal,Some(Box::new(Node::new_num(2))), Some(Box::new(Node::new(NodeKind::LessThan, Some(Box::new(Node::new_num(1))), Some(Box::new(Node::new_num(2)))))))]
        ),
        case(
            vec![Token::Num(2), Token::Equal, Token::Num(2), Token::GreaterThan, Token::Num(1), Token::Semicolon],
            vec![Node::new(NodeKind::Equal,Some(Box::new(Node::new_num(2))), Some(Box::new(Node::new(NodeKind::LessThan, Some(Box::new(Node::new_num(1))), Some(Box::new(Node::new_num(2)))))))]
        ),
        case(
            vec![Token::Num(2), Token::Equal, Token::Num(2), Token::GreaterThan, Token::Num(1), Token::Semicolon, Token::Num(2), Token::Equal, Token::Num(2), Token::GreaterThan, Token::Num(1), Token::Semicolon],
            vec![
                Node::new(NodeKind::Equal,Some(Box::new(Node::new_num(2))), Some(Box::new(Node::new(NodeKind::LessThan, Some(Box::new(Node::new_num(1))), Some(Box::new(Node::new_num(2))))))),
                Node::new(NodeKind::Equal,Some(Box::new(Node::new_num(2))), Some(Box::new(Node::new(NodeKind::LessThan, Some(Box::new(Node::new_num(1))), Some(Box::new(Node::new_num(2))))))),
            ]
        ),
        case(
            vec![Token::Identifier("a".to_string()), Token::Assignment, Token::Num(0), Token::Semicolon],
            vec![Node::new(NodeKind::Assignment, Some(Box::new(Node::new_local_var("a".to_string(), 8))), Some(Box::new(Node::new_num(0))))],
        ),
        case(
            vec![Token::Identifier("a".to_string()), Token::Assignment, Token::Num(0), Token::Semicolon, Token::Identifier("b".to_string()), Token::Assignment, Token::Num(1), Token::Semicolon],
            vec![
                Node::new(NodeKind::Assignment, Some(Box::new(Node::new_local_var("a".to_string(), 8))), Some(Box::new(Node::new_num(0)))),
                Node::new(NodeKind::Assignment, Some(Box::new(Node::new_local_var("b".to_string(), 8 * 2))), Some(Box::new(Node::new_num(1))))
            ],
        ),
        case(
            vec![Token::Identifier("a".to_string()), Token::Assignment, Token::Num(0), Token::Semicolon, Token::Identifier("b".to_string()), Token::Assignment, Token::Num(1), Token::Semicolon, Token::Identifier("c".to_string()), Token::Assignment, Token::Identifier("a".to_string()), Token::Add, Token::Identifier("b".to_string()), Token::Semicolon],
            vec![
                Node::new(NodeKind::Assignment, Some(Box::new(Node::new_local_var("a".to_string(), 8))), Some(Box::new(Node::new_num(0)))),
                Node::new(NodeKind::Assignment, Some(Box::new(Node::new_local_var("b".to_string(), 8 * 2))), Some(Box::new(Node::new_num(1)))),
                Node::new(NodeKind::Assignment, Some(Box::new(Node::new_local_var("c".to_string(), 8 * 3))), Some(Box::new(Node::new(NodeKind::Add, Some(Box::new(Node::new_local_var("a".to_string(), 8))), Some(Box::new(Node::new_local_var("b".to_string(), 8 * 2)))))))
            ],
        ),
        case(
            vec![Token::Identifier("a".to_string()), Token::Assignment, Token::Num(0), Token::Semicolon, Token::Identifier("b".to_string()), Token::Assignment, Token::Num(1), Token::Semicolon, Token::Return, Token::Identifier("a".to_string()), Token::Add, Token::Identifier("b".to_string()), Token::Semicolon],
            vec![
                Node::new(NodeKind::Assignment, Some(Box::new(Node::new_local_var("a".to_string(), 8))), Some(Box::new(Node::new_num(0)))),
                Node::new(NodeKind::Assignment, Some(Box::new(Node::new_local_var("b".to_string(), 8 * 2))), Some(Box::new(Node::new_num(1)))),
                Node::new(NodeKind::Return, Some(Box::new(Node::new(NodeKind::Add, Some(Box::new(Node::new_local_var("a".to_string(), 8))), Some(Box::new(Node::new_local_var("b".to_string(), 8 * 2)))))), None)
            ],
        ),
        case(
            vec![Token::If, Token::OpenParen, Token::Num(1), Token::CloseParen, Token::Return, Token::Num(1), Token::Semicolon],
            vec![Node::new(NodeKind::If, Some(Box::new(Node::new_num(1))), Some(Box::new(Node::new(NodeKind::Return, Some(Box::new(Node::new_num(1))), None))))]
        ),
        case(
            vec![
                Token::Identifier("a".to_string()), Token::Assignment, Token::Num(1), Token::Semicolon,
                Token::If, Token::OpenParen, Token::Identifier("a".to_string()), Token::Equal, Token::Num(0), Token::CloseParen,
                Token::Return, Token::Num(0), Token::Semicolon,
                Token::Else,
                Token::Return, Token::Num(1), Token::Semicolon,
                ],
            vec![
                Node::new(NodeKind::Assignment, Some(Box::new(Node::new_local_var("a".to_string(), 8))), Some(Box::new(Node::new_num(1)))),
                Node::new(
                    NodeKind::If,
                    Some(Box::new(Node::new(NodeKind::Equal, Some(Box::new(Node::new_local_var("a".to_string(), 8))), Some(Box::new(Node::new_num(0)))))),
                    Some(Box::new(Node::new(NodeKind::Else, Some(Box::new(Node::new(NodeKind::Return, Some(Box::new(Node::new_num(0))), None))), Some(Box::new(Node::new(NodeKind::Return, Some(Box::new(Node::new_num(1))), None))))))
                    )
                ],
        ),
        case(
            vec![
                Token::Identifier("a".to_string()), Token::Assignment, Token::Num(0), Token::Semicolon,
                Token::While, Token::OpenParen, Token::Identifier("a".to_string()), Token::Equal, Token::Num(10), Token::CloseParen,
                Token::Identifier("a".to_string()), Token::Assignment, Token::Identifier("a".to_string()), Token::Add, Token::Num(1), Token::Semicolon,
                Token::Return, Token::Identifier("a".to_string()), Token::Semicolon,
            ],
            vec![
                Node::new(NodeKind::Assignment, Some(Box::new(Node::new_local_var("a".to_string(), 8))), Some(Box::new(Node::new_num(0)))),
                Node::new(NodeKind::While,
                    Some(Box::new(Node::new(NodeKind::Equal, Some(Box::new(Node::new_local_var("a".to_string(), 8))), Some(Box::new(Node::new_num(10)))))),
                    Some(Box::new(Node::new(NodeKind::Assignment,
                        Some(Box::new(Node::new_local_var("a".to_string(), 8))),
                        Some(Box::new(Node::new(NodeKind::Add, Some(Box::new(Node::new_local_var("a".to_string(), 8))), Some(Box::new(Node::new_num(1))))))))),
                ),
                Node::new(NodeKind::Return, Some(Box::new(Node::new_local_var("a".to_string(), 8))), None),
            ],
        ),
        case(
            vec![Token::OpenBrace, Token::CloseBrace],
            vec![Node::new(NodeKind::Block(vec![]), None, None)],
        ),
        case(
            vec![
                Token::OpenBrace,
                Token::Identifier("a".to_string()), Token::Assignment, Token::Num(0), Token::Semicolon,
                Token::Identifier("b".to_string()), Token::Assignment, Token::Num(1), Token::Semicolon,
                Token::Return, Token::Identifier("a".to_string()), Token::Add, Token::Identifier("b".to_string()), Token::Semicolon,
                Token::CloseBrace
            ],
            vec![Node::new(NodeKind::Block(vec![
                Node::new(NodeKind::Assignment, Some(Box::new(Node::new_local_var("a".to_string(), 8))), Some(Box::new(Node::new_num(0)))),
                Node::new(NodeKind::Assignment, Some(Box::new(Node::new_local_var("b".to_string(), 8 * 2))), Some(Box::new(Node::new_num(1)))),
                Node::new(NodeKind::Return, Some(Box::new(Node::new(NodeKind::Add, Some(Box::new(Node::new_local_var("a".to_string(), 8))), Some(Box::new(Node::new_local_var("b".to_string(), 8 * 2)))))), None),
            ]), None, None)],
        ),
        case(
            vec![Token::Identifier("foo".to_string()), Token::OpenParen, Token::CloseParen, Token::Semicolon],
            vec![Node::new(NodeKind::Func("foo".to_string(), vec![]), None, None)],
        ),
        case(
            vec![Token::Identifier("foo".to_string()), Token::OpenParen, Token::Num(1), Token::Num(2), Token::CloseParen, Token::Semicolon],
            vec![Node::new(NodeKind::Func("foo".to_string(), vec![Node::new_num(1), Node::new_num(2)]), None, None)],
        ),
    )]
    fn test_parser_parse(input: Vec<Token>, expect: Vec<Node>) {
        let mut parser = Parser::new(input);
        parser.parse().unwrap();
        assert_eq!(expect, parser.nodes);
    }
}
