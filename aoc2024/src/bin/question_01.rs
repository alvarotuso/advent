use aoc2024::client::AocClient;
use aoc2024::errors::AocError;
use futures::TryStreamExt;
use std::collections::{BinaryHeap, HashMap};

#[tokio::main]
async fn main() -> Result<(), AocError> {
    let aoc_client = AocClient::new(reqwest::Client::new());
    let mut stream = Box::pin(aoc_client.get_input_stream(2024, 1).await);
    let mut heap_list_1 = BinaryHeap::new();
    let mut heap_list_2 = BinaryHeap::new();
    let mut l2_occurrences = HashMap::new();
    let mut l1_numbers = Vec::new();
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
            l1_numbers.push(l1);
            heap_list_1.push(l1);
        } else {
            return parse_err;
        }
        if let Ok(l2) = parts[1] {
            l2_occurrences
                .entry(l2)
                .and_modify(|c| *c += 1)
                .or_insert(1);
            heap_list_2.push(l2);
        } else {
            return parse_err;
        }
    }
    let mut total_part_1 = 0;
    while let Some(l1) = heap_list_1.pop() {
        let l2 = heap_list_2.pop().unwrap();
        total_part_1 += (l1 - l2).abs();
    }
    println!("Part 1: {}", total_part_1);

    let mut total_part_2 = 0;
    for l1 in l1_numbers {
        total_part_2 += l1 * l2_occurrences.get(&l1).unwrap_or(&0);
    }
    println!("Part 2: {}", total_part_2);
    Ok(())
}
