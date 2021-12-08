use std::cmp;
use std::collections::HashSet;
use std::io::Error;

use crate::{create_data_iter, Day, Puzzle, PuzzleError};

const CRAB_POSITIONS: &str = "day_7/crab_positions.txt";
const DAY: Day = Day::Seven;

fn calculate_scaling_fuel_cost(
    convergence_point: u32,
    crab_positions: &Vec<u32>,
) -> (u32, u32, u32) {
    let convergence_point = convergence_point as i32;

    let mut left_fuel_cost = 0;
    let mut mid_fuel_cost = 0;
    let mut right_fuel_cost = 0;
    for &position in crab_positions {
        let position = position as i32;
        let distance_to_convergence_point = (convergence_point - position).abs();

        mid_fuel_cost += distance_to_convergence_point * (distance_to_convergence_point + 1) / 2;

        let left_distance = (convergence_point - position - 1).abs();
        left_fuel_cost += left_distance * (left_distance + 1) / 2;

        let right_distance = (convergence_point - position + 1).abs();
        right_fuel_cost += right_distance * (right_distance + 1) / 2;
    }

    (
        left_fuel_cost as u32,
        mid_fuel_cost as u32,
        right_fuel_cost as u32,
    )
}

fn puzzle_one_impl(
    crab_positions_input: impl Iterator<Item = Result<String, Error>>,
) -> Result<u32, PuzzleError> {
    let mut crab_positions = vec![];
    let mut possible_positions = HashSet::new();
    for position_line in crab_positions_input {
        let position_line = position_line?;
        for position in position_line.trim().split(",") {
            let position =
                position
                    .parse::<u32>()
                    .map_err(|_| PuzzleError::DataConsistencyError {
                        day: DAY,
                        puzzle: Puzzle::One,
                        expected: "crab positions to be parseable as u32s".to_string(),
                        found: position.to_string(),
                    })?;
            crab_positions.push(position);
            possible_positions.insert(position);
        }
    }

    let mut min_fuel = u32::MAX;
    for position in possible_positions {
        let mut fuel_for_position = 0;
        for fuel_cost in crab_positions.iter().cloned() {
            fuel_for_position += if fuel_cost < position {
                position - fuel_cost
            } else {
                fuel_cost - position
            };
            if fuel_for_position >= min_fuel {
                break;
            }
        }
        min_fuel = cmp::min(fuel_for_position, min_fuel);
    }
    Ok(min_fuel)
}

fn puzzle_two_impl(
    crab_positions_input: impl Iterator<Item = Result<String, Error>>,
) -> Result<u32, PuzzleError> {
    let mut crab_positions = vec![];
    let mut min_position = u32::MAX;
    let mut max_position = 0;
    for position_line in crab_positions_input {
        let position_line = position_line?;
        for position in position_line.trim().split(",") {
            let position =
                position
                    .parse::<u32>()
                    .map_err(|_| PuzzleError::DataConsistencyError {
                        day: DAY,
                        puzzle: Puzzle::Two,
                        expected: "crab positions to be parseable as u32s".to_string(),
                        found: position.to_string(),
                    })?;
            min_position = cmp::min(min_position, position);
            max_position = cmp::max(max_position, position);
            crab_positions.push(position);
        }
    }

    while min_position <= max_position {
        let mid_point = min_position + (max_position - min_position) / 2;
        let (left_fuel, mid_fuel, right_fuel) =
            calculate_scaling_fuel_cost(mid_point, &crab_positions);

        if mid_fuel <= left_fuel && mid_fuel <= right_fuel {
            return Ok(mid_fuel);
        }
        if mid_fuel > left_fuel {
            max_position = mid_point;
        } else {
            min_position = mid_point;
        }
    }
    Err(PuzzleError::NoSolutionFound)
}

pub fn puzzle_one() -> Result<(), PuzzleError> {
    println!("Starting day {}, puzzle {}", DAY, Puzzle::One);
    let crab_positions = create_data_iter(CRAB_POSITIONS)?;
    let minimum_fuel = puzzle_one_impl(crab_positions)?;
    println!("Minimum fuel for crabs: {}", minimum_fuel);
    println!("Finished day {}, puzzle {}", DAY, Puzzle::One);
    Ok(())
}

pub fn puzzle_two() -> Result<(), PuzzleError> {
    println!("Starting day {}, puzzle {}", DAY, Puzzle::Two);
    let crab_positions = create_data_iter(CRAB_POSITIONS)?;
    let minimum_fuel = puzzle_two_impl(crab_positions)?;
    println!("Minimum fuel for crabs: {}", minimum_fuel);
    println!("Finished day {}, puzzle {}", DAY, Puzzle::Two);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_input() -> [Result<String, Error>; 1] {
        [Ok("16,1,2,0,4,2,7,1,2,14".to_string())]
    }

    #[test]
    fn test_puzzle_one_impl() {
        let input = create_input();
        let min_fuel = puzzle_one_impl(input.into_iter()).unwrap();
        assert_eq!(min_fuel, 37);
    }

    #[test]
    fn test_puzzle_two_impl() {
        let input = create_input();
        let min_fuel = puzzle_two_impl(input.into_iter()).unwrap();
        assert_eq!(min_fuel, 168);
    }
}
