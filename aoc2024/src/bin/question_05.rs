use aoc2024::client::AocClient;
use aoc2024::errors::AocError;
use futures::TryStreamExt;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;

#[tokio::main]
async fn main() -> Result<(), AocError> {
    let aoc_client = AocClient::new(reqwest::Client::new());
    let mut stream = Box::pin(aoc_client.get_input_stream(2024, 5).await);
    let mut must_befores: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut total_part_1 = 0;
    let mut total_part_2 = 0;
    while let Some(line) = stream.try_next().await? {
        if line == "" {
            break;
        }
        let parts_result: Result<Vec<u32>, ParseIntError> =
            line.split("|").map(|v| v.parse::<u32>()).collect();
        let parts = parts_result?;
        if parts.len() != 2 {
            return Err(AocError::Error(format!("invalid priority line: {}", line)));
        }
        let current_higher_priorities = must_befores.entry(parts[1]).or_insert(HashSet::new());
        current_higher_priorities.insert(parts[0]);
    }
    while let Some(line) = stream.try_next().await? {
        let parts_result: Result<Vec<u32>, ParseIntError> =
            line.split(",").map(|v| v.parse::<u32>()).collect();
        let mut parts = parts_result?;
        let mut is_valid = true;
        for i in 0..parts.len() {
            if !is_valid {
                break;
            }
            if let Some(higher_prios) = must_befores.get(&(parts[i])) {
                for j in (i + 1)..parts.len() {
                    if higher_prios.contains(&(parts[j])) {
                        is_valid = false;
                        break;
                    }
                }
            }
        }
        if !is_valid {
            parts.sort_by(|a, b| {
                if let Some(higher_prio) = must_befores.get(a) {
                    if higher_prio.contains(b) {
                        return Ordering::Less;
                    }
                };
                if let Some(higher_prio) = must_befores.get(b) {
                    if higher_prio.contains(a) {
                        return Ordering::Greater;
                    };
                }
                return Ordering::Equal;
            });
        }
        let mid = parts[parts.len() / 2];
        if is_valid {
            total_part_1 += mid;
        } else {
            total_part_2 += mid;
        }
    }
    println!("Part 1: {}", total_part_1);
    println!("Part 2: {}", total_part_2);
    Ok(())
}
