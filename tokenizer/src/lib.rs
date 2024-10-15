use std::{iter::Peekable, str::Chars};

use error::Error;
use token::{reserved, Token};

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
                _ => {
                    if p.is_ascii_digit() {
                        let n = get_num(&mut chars, p)?;
                        tokens.push(Token::Num(n));
                    } else {
                        return Err(Error::UnknownToken(p));
                    }
                }
            }
        }

        Ok(tokens)
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
    )]
    fn test_tokenizer_process(input: &str, expect: Vec<Token>) {
        let tokenizer = Tokenizer::default();
        let res = tokenizer.process(input.to_string()).unwrap();
        assert_eq!(expect, res);
    }
}
