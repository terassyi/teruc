use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unknown token: {0}")]
    UnknownToken(char),
    #[error("failed to tokenize")]
    FailedToTokenize,
    #[error("failed to parse to number")]
    FailedToParseToNum,
}
