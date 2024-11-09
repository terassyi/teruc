use std::{iter::Peekable, str::Chars};

use error::Error;
use token::{is_reserved, reserved, Token};

mod error;

pub struct Tokenizer {}

impl Tokenizer {
    fn new() -> Self {
        Self {}
    }

    pub fn process(&self, src: String) -> Result<Vec<Token>, Error> {
        let mut tokens = vec![];
        let mut chars = src.chars().peekable();

        while let Some(p) = chars.next() {
            match p {
                reserved::WHITE_SPACE => {}
                reserved::PLUS => tokens.push(Token::Add),
                reserved::MINUS => tokens.push(Token::Sub),
                reserved::ASTERISK => tokens.push(Token::Mul),
                reserved::SLASH => tokens.push(Token::Div),
                reserved::OPEN_PAREN => tokens.push(Token::OpenParen),
                reserved::CLOSE_PAREN => tokens.push(Token::CloseParen),
                reserved::EQUAL => {
                    let t = self.process_equal(&mut chars)?;
                    tokens.push(t);
                }
                reserved::EXCLAMATION => {
                    let t = self.process_exclamation(&mut chars)?;
                    tokens.push(t);
                }
                reserved::LESS_THAN => {
                    let t = self.process_less_than(&mut chars)?;
                    tokens.push(t);
                }
                reserved::GREATER_THAN => {
                    let t = self.process_greater_than(&mut chars)?;
                    tokens.push(t);
                }
                reserved::SEMICOLON => tokens.push(Token::Semicolon),
                _ => {
                    if p.is_ascii_digit() {
                        let n = get_num(&mut chars, p)?;
                        tokens.push(Token::Num(n));
                    } else if p.is_ascii_alphabetic() {
                        let ident = self.process_identifier(&mut chars, p)?;
                        tokens.push(ident);
                    } else {
                        return Err(Error::UnknownToken(p));
                    }
                }
            }
        }

        Ok(tokens)
    }

    fn process_equal(&self, chars: &mut Peekable<Chars>) -> Result<Token, Error> {
        if let Some(p) = chars.peek() {
            match *p {
                reserved::WHITE_SPACE => Ok(Token::Assignment),
                reserved::EQUAL => {
                    let _ = chars.next();
                    Ok(Token::Equal)
                }
                _ => {
                    // err
                    Err(Error::FailedToTokenize)
                }
            }
        } else {
            Err(Error::FailedToTokenize)
        }
    }

    fn process_exclamation(&self, chars: &mut Peekable<Chars>) -> Result<Token, Error> {
        if let Some(p) = chars.peek() {
            match *p {
                reserved::WHITE_SPACE => Ok(Token::Not),
                reserved::EQUAL => {
                    let _ = chars.next();
                    Ok(Token::NotEqual)
                }
                _ => Err(Error::FailedToTokenize),
            }
        } else {
            Err(Error::FailedToTokenize)
        }
    }

    fn process_less_than(&self, chars: &mut Peekable<Chars>) -> Result<Token, Error> {
        if let Some(p) = chars.peek() {
            match *p {
                reserved::WHITE_SPACE => Ok(Token::LessThan),
                reserved::EQUAL => {
                    let _ = chars.next();
                    Ok(Token::LessThanOrEqual)
                }
                _ => Err(Error::FailedToTokenize),
            }
        } else {
            Err(Error::FailedToTokenize)
        }
    }

    fn process_greater_than(&self, chars: &mut Peekable<Chars>) -> Result<Token, Error> {
        if let Some(p) = chars.peek() {
            match *p {
                reserved::WHITE_SPACE => Ok(Token::GreaterThan),
                reserved::EQUAL => {
                    let _ = chars.next();
                    Ok(Token::GreaterThanOrEqual)
                }
                _ => Err(Error::FailedToTokenize),
            }
        } else {
            Err(Error::FailedToTokenize)
        }
    }

    fn process_identifier(&self, chars: &mut Peekable<Chars>, head: char) -> Result<Token, Error> {
        let mut ident = head.to_string();
        while let Some(c) = chars.peek() {
            if is_reserved(*c) {
                return match Self::reserved_identifier(&ident) {
                    Some(t) => Ok(t),
                    None => Ok(Token::Identifier(ident)),
                };
            }
            if let Some(p) = chars.next() {
                ident += &p.to_string();
            } else {
                // got EOF
                return match Self::reserved_identifier(&ident) {
                    Some(t) => Ok(t),
                    None => Ok(Token::Identifier(ident)),
                };
            }
        }
        // got EOF while generating an identifier
        match Self::reserved_identifier(&ident) {
            Some(t) => Ok(t),
            None => Ok(Token::Identifier(ident)),
        }
    }

    fn reserved_identifier(ident: &str) -> Option<Token> {
        match ident {
            reserved::RETURN => Some(Token::Return),
            reserved::IF => Some(Token::If),
            reserved::ELSE => Some(Token::Else),
            reserved::WHILE => Some(Token::While),
            _ => None,
        }
    }
}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::new()
    }
}

fn get_num(chars: &mut Peekable<Chars>, head: char) -> Result<u64, Error> {
    let mut num_c = vec![head];

    while let Some(c) = chars.peek() {
        if c.is_ascii_digit() {
            // consume iter
            if let Some(p) = chars.next() {
                num_c.push(p);
            }
        } else {
            break;
        }
    }
    let num_s: String = num_c.iter().collect();
    num_s.parse::<u64>().map_err(|_| Error::FailedToParseToNum)
}

#[cfg(test)]
mod tests {

    use rstest::rstest;

    use crate::get_num;
    use crate::Tokenizer;

    use token::Token;

    #[rstest(
        input,
        head,
        expect,
        remain,
        case("", '1', 1, ""),
        case("0", '1', 10, ""),
        case("55", '2', 255, ""),
        case("55 ", '2', 255, " "),
        case("55+10", '2', 255, "+10")
    )]
    fn test_get_num(input: &str, head: char, expect: u64, remain: &str) {
        let mut chars = input.chars().peekable();
        let res = get_num(&mut chars, head).unwrap();
        assert_eq!(expect, res);

        // check unconsumed chars
        let unconsumed: String = chars.collect();
        assert_eq!(remain.to_string(), unconsumed);
    }

    #[rstest(
        input,
        expect,
        case("", vec![]),
        case(" ", vec![]),
        case("0", vec![Token::Num(0)]),
        case("10", vec![Token::Num(10)]),
        case("+", vec![Token::Add]),
        case(" 0", vec![Token::Num(0)]),
        case("0   ", vec![Token::Num(0)]),
        case("0+1", vec![Token::Num(0), Token::Add, Token::Num(1)]),
        case("0 + 1", vec![Token::Num(0), Token::Add, Token::Num(1)]),
        case("0 +  1", vec![Token::Num(0), Token::Add, Token::Num(1)]),
        case("10 +  1", vec![Token::Num(10), Token::Add, Token::Num(1)]),
        case("100 + 1100 - 200", vec![Token::Num(100), Token::Add, Token::Num(1100), Token::Sub, Token::Num(200)]),
        case("100 * 2", vec![Token::Num(100), Token::Mul, Token::Num(2)]),
        case("100/2", vec![Token::Num(100), Token::Div, Token::Num(2)]),
        case("1+2*3", vec![Token::Num(1), Token::Add, Token::Num(2), Token::Mul, Token::Num(3)]),
        case("1*2+(3+4)", vec![Token::Num(1), Token::Mul, Token::Num(2), Token::Add, Token::OpenParen, Token::Num(3), Token::Add, Token::Num(4), Token::CloseParen]),
        case("0 == 0", vec![Token::Num(0), Token::Equal, Token::Num(0)]),
        case("0 != 0", vec![Token::Num(0), Token::NotEqual, Token::Num(0)]),
        case("0 < 0", vec![Token::Num(0), Token::LessThan, Token::Num(0)]),
        case("0 <= 0", vec![Token::Num(0), Token::LessThanOrEqual, Token::Num(0)]),
        case("0 > 0", vec![Token::Num(0), Token::GreaterThan, Token::Num(0)]),
        case("0 >= 0", vec![Token::Num(0), Token::GreaterThanOrEqual, Token::Num(0)]),
        case("a = 1", vec![Token::Identifier("a".to_string()), Token::Assignment, Token::Num(1)]),
        case("aa = 1 + (2 * 3 - 4); ", vec![Token::Identifier("aa".to_string()), Token::Assignment, Token::Num(1), Token::Add, Token::OpenParen, Token::Num(2), Token::Mul, Token::Num(3), Token::Sub, Token::Num(4), Token::CloseParen, Token::Semicolon]),
        case("a = 1; b = 2; c = a + b", vec![Token::Identifier("a".to_string()), Token::Assignment, Token::Num(1), Token::Semicolon, Token::Identifier("b".to_string()), Token::Assignment, Token::Num(2), Token::Semicolon, Token::Identifier("c".to_string()), Token::Assignment, Token::Identifier("a".to_string()), Token::Add, Token::Identifier("b".to_string())]),
        case("return 1;", vec![Token::Return, Token::Num(1), Token::Semicolon]),
        case("if (1) a = 1; else a = 2;", vec![Token::If, Token::OpenParen, Token::Num(1), Token::CloseParen, Token::Identifier("a".to_string()), Token::Assignment, Token::Num(1), Token::Semicolon, Token::Else, Token::Identifier("a".to_string()), Token::Assignment, Token::Num(2), Token::Semicolon]),
        case("while (a == 10) a = a + 1", vec![Token::While, Token::OpenParen, Token::Identifier("a".to_string()), Token::Equal, Token::Num(10), Token::CloseParen, Token::Identifier("a".to_string()), Token::Assignment, Token::Identifier("a".to_string()), Token::Add, Token::Num(1)])
    )]
    fn test_tokenizer_process(input: &str, expect: Vec<Token>) {
        let tokenizer = Tokenizer::default();
        let res = tokenizer.process(input.to_string()).unwrap();
        assert_eq!(expect, res);
    }
}
