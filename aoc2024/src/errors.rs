use thiserror::Error;
use crate::client::AocClientError;

#[derive(Debug, Error)]
pub enum AocError {
    #[error("client error: {0}")]
    ClientError(#[from] AocClientError),
}
