use error::Error;
use reserved::{MINUS, PLUS, WHITE_SPACE};

mod error;
mod reserved;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Reserved(char),
    Num(u32),
    Eof,
}

pub struct Tokenizer {
    token: Option<Token>,
}

impl Tokenizer {
    fn new() -> Self {
        Self { token: None }
    }

    pub fn process(&self, src: String) -> Result<Vec<Token>, Error> {
        let mut tokens = vec![];
        let mut chars = src.chars();

        while let Some(p) = chars.next() {
            if let Some(token) = Tokenizer::process_one(p)? {
                tokens.push(token);
            }
        }

        Ok(tokens)
    }

    fn process_one(p: char) -> Result<Option<Token>, Error> {
        match p {
            WHITE_SPACE => {
                // ignore white space
                Ok(None)
            }
            PLUS | MINUS => {
                // process plus and minus
                Ok(Some(Token::Reserved(p)))
            }
            _ => match p.to_digit(10) {
                Some(n) => Ok(Some(Token::Num(n))),
                None => Err(Error::UnknownToken(p)),
            },
        }
    }
}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{Token, Tokenizer};

    #[rstest(
        input,
        expect,
        case('+', Token::Reserved('+')),
        case('-', Token::Reserved('-')),
        case('0', Token::Num(0)),
        case('9', Token::Num(9))
    )]
    fn tokenizer_process_one_ok_with_some(input: char, expect: Token) {
        let res = Tokenizer::process_one(input).unwrap().unwrap();
        assert_eq!(expect, res);
    }
    #[rstest(input, case(' '))]
    fn tokenizer_process_one_ok_with_none(input: char) {
        let res = Tokenizer::process_one(input).unwrap();
        assert!(res.is_none())
    }
    #[rstest(input, case('p'))]
    fn tokenizer_process_one_err(input: char) {
        let res = Tokenizer::process_one(input);
        assert!(res.is_err())
    }

    #[rstest(
        input,
        expect,
        case("", vec![]),
        case(" ", vec![]),
        case("0", vec![Token::Num(0)]),
        case("+", vec![Token::Reserved('+')]),
        case(" 0", vec![Token::Num(0)]),
        case("0   ", vec![Token::Num(0)]),
        case("0+1", vec![Token::Num(0), Token::Reserved('+'), Token::Num(1)]),
        case("0 + 1", vec![Token::Num(0), Token::Reserved('+'), Token::Num(1)]),
        case("0 +  1", vec![Token::Num(0), Token::Reserved('+'), Token::Num(1)]),
    )]
    fn tokenizer_process(input: &str, expect: Vec<Token>) {
        let tokenizer = Tokenizer::default();
        let res = tokenizer.process(input.to_string()).unwrap();
        assert_eq!(expect, res);
    }
}
