use aoc2024::client::AocClient;
use aoc2024::errors::AocError;
use futures::TryStreamExt;
use std::collections::BinaryHeap;

#[tokio::main]
async fn main() -> Result<(), AocError> {
    let aoc_client = AocClient::new(reqwest::Client::new());
    let mut stream = Box::pin(aoc_client.get_input_stream(2024, 1).await);
    let mut heap_list_1 = BinaryHeap::new();
    let mut heap_list_2 = BinaryHeap::new();
    while let Some(line) = stream.try_next().await? {
        let parts = line
            .split_ascii_whitespace()
            .map(|part| part.parse::<i32>())
            .collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(AocError::Error(format!(
                "invalid number of parts: {} in line: {}",
                parts.len(),
                line
            )));
        }
        let parse_err = Err(AocError::Error(format!("failed to parse line: {}", line)));
        if let Ok(l1) = parts[0] {
            heap_list_1.push(l1);
        } else {
            return parse_err;
        }
        if let Ok(l2) = parts[1] {
            heap_list_2.push(l2);
        } else {
            return parse_err;
        }
    }
    let mut total = 0;
    while let Some(l1) = heap_list_1.pop() {
        let l2 = heap_list_2.pop().unwrap();
        total += (l1 - l2).abs();
    }
    println!("Part 1: {}", total);

    Ok(())
}
