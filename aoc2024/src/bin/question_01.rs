use aoc2024::client::AocClient;
use aoc2024::errors::AocError;
use futures::TryStreamExt;
use std::collections::{BinaryHeap, HashMap};
use std::num::ParseIntError;

#[tokio::main]
async fn main() -> Result<(), AocError> {
    let aoc_client = AocClient::new(reqwest::Client::new());
    let mut stream = Box::pin(aoc_client.get_input_stream(2024, 1).await);
    let mut heap_list_1 = BinaryHeap::new();
    let mut heap_list_2 = BinaryHeap::new();
    let mut l2_occurrences = HashMap::new();
    let mut l1_numbers = Vec::new();
    while let Some(line) = stream.try_next().await? {
        let parts_result: Result<Vec<i32>, ParseIntError> = line
            .split_ascii_whitespace()
            .map(|part| part.parse::<i32>())
            .collect();
        let parts = parts_result?;
        if parts.len() != 2 {
            return Err(AocError::Error(format!(
                "invalid number of parts: {} in line: {}",
                parts.len(),
                line
            )));
        }
        l1_numbers.push(parts[0]);
        l2_occurrences
            .entry(parts[1])
            .and_modify(|c| *c += 1)
            .or_insert(1);
        heap_list_1.push(parts[0]);
        heap_list_2.push(parts[1]);
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
