use std::env;
use std::env::VarError;
use async_stream::try_stream;
use bytes::Bytes;
use futures::{Stream, TryStreamExt};
use reqwest;
use thiserror::Error;

const SESSION_COOKIE_ENV: &str = "AOC_SESSION_COOKIE";

#[derive(Debug, Error)]
pub enum AocClientError {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("missing environment variable error: {0}")]
    VarError(#[from] VarError),
}

pub struct AocClient {
    client: reqwest::Client,
}

impl AocClient {
    pub fn new(client: reqwest::Client) -> AocClient {
        AocClient { client }
    }

    async fn get_input_stream_bytes(&self, year: i16, question: i8) -> Result<impl Stream<Item=Result<Bytes, reqwest::Error>>, AocClientError> {
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, question);
        let session = env::var(SESSION_COOKIE_ENV)?;
        Ok(self.client.get(url).header("Cookie", format!("session={}", session)).send().await?.bytes_stream())
    }
    
    pub async fn get_input_stream(&self, year: i16, question: i8) -> impl Stream<Item=Result<String, AocClientError>> + use<'_> {
        try_stream! {
            let mut s = self.get_input_stream_bytes(year, question).await?;
            let mut line = String::new();
            while let Some(item) = s.try_next().await? {
                for b in item {
                    if b == b'\n' {
                        yield line;
                        line = String::new();
                    } else {
                        line.push(char::from(b));
                    }
                }
            }
            if !line.is_empty() {
                yield line
            }
        }
    }
}