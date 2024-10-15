use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid node")]
    InvalidNode,
}