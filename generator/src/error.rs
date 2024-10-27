use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid node")]
    InvalidNode,
    #[error("left value of assignment must be identifier")]
    LeftValueMustBeIdentifier,
}
