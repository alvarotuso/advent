use bytes::Bytes;
use reqwest;
use thiserror::Error;
use futures::stream::IntoAsyncRead;
use futures::{AsyncRead, Stream, TryStreamExt};
use tokio::io::AsyncBufRead;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use tokio_util::io::StreamReader;

#[derive(Debug, Error)]
pub enum AocClientError {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}


type InputStream = dyn Stream<Item=reqwest::Result<Bytes>>;

struct  InputIterator {
    stream: Box<InputStream>,
}

impl InputIterator {
    fn new(stream: impl Stream<Item=reqwest::Result<Bytes>>) -> Self {
        Self { stream: Box::new(stream) }
    }
}

struct AocClient {
    client: reqwest::Client,
}

impl AocClient {
    pub fn new(client: reqwest::Client) -> AocClient {
        AocClient { client }
    }

    pub async fn get_input_reader(&self, year: i8, question: i8) -> Result<impl AsyncBufRead, AocClientError> {
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, question);
        let stream = self.client.get(url).send().await?.bytes_stream();
        Ok(StreamReader::new(stream))
    }
}