use thiserror::Error;
use token::Token;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid token: {0}")]
    InvalidToken(Token),
    #[error("unexpected token: expected -> {0}, actual -> {1}")]
    UnexpectedToken(Token, Token),
    #[error("invalid termination")]
    InvalidTermination,
}
