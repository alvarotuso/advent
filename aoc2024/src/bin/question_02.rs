use aoc2024::client::AocClient;
use aoc2024::errors::AocError;
use futures::TryStreamExt;
use std::num::ParseIntError;

fn is_report_safe(levels: &Vec<i32>) -> bool {
    let mut safe = true;
    let mut ascending = Option::<bool>::None;
    for i in 1..levels.len() {
        let diff = (levels[i] - levels[i - 1]).abs();
        if diff == 0 || diff > 3 {
            safe = false;
            break;
        }
        if ascending.is_some() {
            let curr_ascending = levels[i] > levels[i - 1];
            if ascending.unwrap() != curr_ascending {
                safe = false;
                break;
            }
        } else {
            ascending = Some(levels[i] > levels[i - 1]);
        }
    }
    safe
}

#[tokio::main]
async fn main() -> Result<(), AocError> {
    let aoc_client = AocClient::new(reqwest::Client::new());
    let mut stream = Box::pin(aoc_client.get_input_stream(2024, 2).await);
    let mut total_part_1 = 0;
    let mut extra_part_2 = 0;
    while let Some(line) = stream.try_next().await? {
        let report_result: Result<Vec<i32>, ParseIntError> = line
            .split_ascii_whitespace()
            .map(|part| part.parse::<i32>())
            .collect();
        let levels = report_result?;
        let mut safe = is_report_safe(&levels);
        if safe {
            total_part_1 += 1;
        }
        if !safe {
            for i in 0..levels.len() {
                safe = is_report_safe(&[&levels[..i], &levels[i + 1..]].concat());
                if safe {
                    extra_part_2 += 1;
                    break;
                }
            }
        }
    }
    println!("Part 1: {}", total_part_1);
    println!("Part 2: {}", total_part_1 + extra_part_2);
    Ok(())
}
