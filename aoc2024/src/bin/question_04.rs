use aoc2024::client::AocClient;
use aoc2024::errors::AocError;
use futures::TryStreamExt;

const SEARCH_WORD: [char; 4] = ['X', 'M', 'A', 'S'];

enum SearchDirection {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl SearchDirection {
    const VALUES: [Self; 8] = [
        Self::N,
        Self::NE,
        Self::E,
        Self::SE,
        Self::S,
        Self::SW,
        Self::W,
        Self::NW,
    ];

    fn value(&self) -> [i8; 2] {
        match *self {
            Self::N => [0, -1],
            Self::NE => [1, -1],
            Self::E => [1, 0],
            Self::SE => [1, 1],
            Self::S => [0, 1],
            Self::SW => [-1, 1],
            Self::W => [-1, 0],
            Self::NW => [-1, -1],
        }
    }
}

fn one_way_search_xmas(
    letter_grid: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    direction: &SearchDirection,
    search_letter: usize,
) -> bool {
    if search_letter > SEARCH_WORD.len() - 1 {
        return true;
    }
    let direction_value = direction.value();
    let next_i = i as i32 + direction_value[0] as i32;
    let next_ui = next_i as usize;
    let next_j = j as i32 + direction_value[1] as i32;
    let next_uj = next_j as usize;
    if next_i < 0 || next_j < 0 || next_ui >= letter_grid.len() || next_uj >= letter_grid[0].len() {
        return false;
    }
    if SEARCH_WORD[search_letter] != letter_grid[next_ui][next_uj] {
        return false;
    }
    one_way_search_xmas(letter_grid, next_ui, next_uj, direction, search_letter + 1)
}

fn eight_way_search_xmas(letter_grid: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let mut eight_way = 0;
    for direction in SearchDirection::VALUES.iter() {
        if one_way_search_xmas(letter_grid, i, j, direction, 1) {
            eight_way += 1;
        }
    }
    eight_way
}

fn mas_check(letter_grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i == 0 || j == 0 || i == letter_grid.len() - 1 || j == letter_grid[0].len() - 1 {
        return false;
    }
    let pairs = vec![
        [letter_grid[i - 1][j - 1], letter_grid[i + 1][j + 1]],
        [letter_grid[i - 1][j + 1], letter_grid[i + 1][j - 1]],
    ];

    pairs.iter().all(|p| match p {
        ['M', 'S'] => true,
        ['S', 'M'] => true,
        _ => false,
    })
}

#[tokio::main]
async fn main() -> Result<(), AocError> {
    let aoc_client = AocClient::new(reqwest::Client::new());
    let mut stream = Box::pin(aoc_client.get_input_stream(2024, 4).await);
    let mut letter_grid: Vec<Vec<char>> = Vec::new();
    let mut total_part_1 = 0;
    let mut total_part_2 = 0;
    while let Some(line) = stream.try_next().await? {
        letter_grid.push(line.chars().collect());
    }
    for i in 0..letter_grid.len() {
        for j in 0..letter_grid[0].len() {
            if letter_grid[i][j] == SEARCH_WORD[0] {
                total_part_1 += eight_way_search_xmas(&letter_grid, i, j);
            }
            if letter_grid[i][j] == 'A' && mas_check(&letter_grid, i, j) {
                total_part_2 += 1;
            }
        }
    }
    println!("Part 1: {}", total_part_1);
    println!("Part 2: {}", total_part_2);
    Ok(())
}
