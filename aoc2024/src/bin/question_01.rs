use futures::TryStreamExt;
use aoc2024::client::AocClient;
use aoc2024::errors::AocError;

#[tokio::main]
async fn main() -> Result<(), AocError> {
    let aoc_client = AocClient::new(reqwest::Client::new());
    let mut s = Box::pin(aoc_client.get_input_stream(2024, 1).await);
    while let Some(line) =  s.try_next().await? {
        println!("{}", line);
    }
    Ok(())
}