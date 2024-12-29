use crate::client::AocClientError;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AocError {
    #[error("client error: {0}")]
    ClientError(#[from] AocClientError),
    #[error("parse error: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("error: {0}")]
    Error(String),
}
