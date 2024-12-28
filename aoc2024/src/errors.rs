use crate::client::AocClientError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AocError {
    #[error("client error: {0}")]
    ClientError(#[from] AocClientError),
    #[error("error: {0}")]
    Error(String),
}
