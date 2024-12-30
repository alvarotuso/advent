use aoc2024::client::AocClient;
use aoc2024::errors::AocError;
use futures::TryStreamExt;
use std::cmp::PartialEq;
use std::collections::HashSet;

const OBSTACLE: char = '#';
const GUARD: char = '^';

enum GridItem {
    Guard,
    Obstacle,
    None,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum WalkDirection {
    N,
    E,
    S,
    W,
}

impl WalkDirection {
    fn value(&self) -> [i8; 2] {
        match *self {
            Self::N => [-1, 0],
            Self::E => [0, 1],
            Self::S => [1, 0],
            Self::W => [0, -1],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct PositionWithDirection {
    i: i32,
    j: i32,
    direction: WalkDirection,
}

impl PositionWithDirection {
    fn new(i: i32, j: i32, direction: WalkDirection) -> Self {
        Self { i, j, direction }
    }
}

fn rotate_direction(direction: WalkDirection) -> WalkDirection {
    match direction {
        WalkDirection::N => WalkDirection::E,
        WalkDirection::E => WalkDirection::S,
        WalkDirection::S => WalkDirection::W,
        WalkDirection::W => WalkDirection::N,
    }
}

#[tokio::main]
async fn main() -> Result<(), AocError> {
    let aoc_client = AocClient::new(reqwest::Client::new());
    let mut stream = Box::pin(aoc_client.get_input_stream(2024, 6).await);
    let mut i = 0;
    let mut guard_pos_i: i32 = 0;
    let mut guard_pos_j: i32 = 0;
    let mut grid: Vec<Vec<GridItem>> = Vec::new();
    while let Some(line) = stream.try_next().await? {
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            row.push(match c {
                GUARD => GridItem::Guard,
                OBSTACLE => GridItem::Obstacle,
                _ => GridItem::None,
            });
            if c == GUARD {
                guard_pos_i = i;
                guard_pos_j = j as i32;
            }
        }
        i += 1;
        grid.push(row);
    }
    let mut positions = HashSet::new();
    positions.insert([guard_pos_i, guard_pos_j]);
    let mut direction = WalkDirection::N;
    let mut positions_and_directions = HashSet::new();
    positions_and_directions.insert(PositionWithDirection {
        i: guard_pos_i.clone(),
        j: guard_pos_j.clone(),
        direction: direction.clone(),
    });
    let mut target_i = guard_pos_i + direction.value()[0] as i32;
    let mut target_j = guard_pos_j + direction.value()[1] as i32;
    while target_i >= 0
        && target_i < grid.len() as i32
        && target_j >= 0
        && target_j < grid[0].len() as i32
    {
        match grid[target_i as usize][target_j as usize] {
            GridItem::Obstacle => {
                direction = rotate_direction(direction);
                println!("Switched direction to {:?} with targets ({:?}, {:?}) and current position ({:?}, {:?})", direction, target_i, target_j, guard_pos_i, guard_pos_j);
            }
            _ => {
                guard_pos_i = target_i;
                guard_pos_j = target_j;
                positions.insert([guard_pos_i, guard_pos_j]);
                positions_and_directions.insert(PositionWithDirection::new(
                    guard_pos_i,
                    guard_pos_j,
                    direction,
                ));
            }
        }
        target_i = guard_pos_i + direction.value()[0] as i32;
        target_j = guard_pos_j + direction.value()[1] as i32;
    }

    let mut positions_for_obstacles = HashSet::new();

    for position in positions_and_directions.iter() {
        let emulated_direction = rotate_direction(position.direction);
        let emulated_i = position.i + emulated_direction.value()[0] as i32;
        let emulated_j = position.j + emulated_direction.value()[1] as i32;
        let emulated_position_and_direction =
            PositionWithDirection::new(emulated_i, emulated_j, emulated_direction);
        // putting an obstacle in front and forcing a rotation would make a loop
        if positions_and_directions.contains(&emulated_position_and_direction) {
            // in front of actual facing direction
            positions_for_obstacles.insert([
                position.i + direction.value()[0] as i32,
                position.j + direction.value()[1] as i32,
            ]);
        }
    }
    println!("Part 1: {}", positions.len());
    println!("Part 2: {}", positions_for_obstacles.len());
    Ok(())
}
