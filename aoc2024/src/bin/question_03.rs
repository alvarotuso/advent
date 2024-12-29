use aoc2024::client::AocClient;
use aoc2024::errors::AocError;
use futures::TryStreamExt;
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), AocError> {
    let aoc_client = AocClient::new(reqwest::Client::new());
    let mut stream = Box::pin(aoc_client.get_input_stream(2024, 3).await);
    let re = Regex::new(r"(mul|do|don't)\(([0-9,]*)\)").unwrap();
    let params_re = Regex::new(r"([0-9]{1,3}),([0-9]{1,3})").unwrap();
    let mut total_part_1 = 0;
    let mut total_part_2 = 0;
    let mut do_mul = true;
    while let Some(line) = stream.try_next().await? {
        for (_, [ins, params]) in re.captures_iter(&line).map(|c| c.extract()) {
            match ins {
                "mul" => {
                    if let Some((_, [n1, n2])) =
                        params_re.captures(params).and_then(|c| Some(c.extract()))
                    {
                        let mul = n1.parse::<i32>()? * n2.parse::<i32>()?;
                        total_part_1 += mul;
                        if do_mul {
                            total_part_2 += mul;
                        }
                    }
                }
                "do" => do_mul = true,
                "don't" => do_mul = false,
                _ => (),
            }
        }
    }
    println!("Part 1: {}", total_part_1);
    println!("Part 2: {}", total_part_2);
    Ok(())
}
