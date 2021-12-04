use std::io::Error;

use crate::{create_data_iter, PuzzleError};

const DIRECTIONS_FILE: &str = "day_2/directions.txt";
const DAY: u8 = 2;

pub fn puzzle_one() -> Result<(), PuzzleError> {
    const PUZZLE: u8 = 1;
    println!("Starting day {}, puzzle {}", DAY, PUZZLE);
    let directions_input_iter = create_data_iter(DIRECTIONS_FILE)?;
    let result = puzzle_one_impl(directions_input_iter);
    println!("Finished day {}, puzzle {}", DAY, PUZZLE);
    result
}

pub fn puzzle_two() -> Result<(), PuzzleError> {
    const PUZZLE: u8 = 2;
    println!("Starting day {}, puzzle {}", DAY, PUZZLE);
    let directions_input_iter = create_data_iter(DIRECTIONS_FILE)?;
    let result = puzzle_two_impl(directions_input_iter);
    let (horizontal_position, vertical_position, position_product) = result?;
    println!(
        "horizontal position: {}, vertical position: {}, position product: {}",
        horizontal_position, vertical_position, position_product
    );
    println!("Finished day {}, puzzle {}", DAY, PUZZLE);
    Ok(())
}

fn puzzle_one_impl(
    directions_input_iter: impl Iterator<Item = Result<String, Error>>,
) -> Result<(), PuzzleError> {
    const PUZZLE: u8 = 1;
    let mut horizontal_position = 0;
    let mut vertical_position = 0;
    for direction_input in directions_input_iter {
        let direction_input = direction_input?;
        let direction_pair: Vec<&str> = direction_input.split_ascii_whitespace().collect();
        if direction_pair.len() != 2 {
            return Err(PuzzleError::DataConsistencyError {
                day: DAY,
                puzzle: PUZZLE,
                expected: "directions to have exactly direction and distance".to_string(),
                found: format!("{:?}", direction_pair),
            });
        }
        let direction = direction_pair[0];
        let distance = direction_pair[1];
        let distance = distance
            .parse::<i32>()
            .map_err(|_| PuzzleError::DataConsistencyError {
                day: DAY,
                puzzle: PUZZLE,
                expected: "distance must be a valid integer".to_string(),
                found: distance.to_string(),
            })?;
        match direction {
            "forward" => horizontal_position += distance,
            "backward" => horizontal_position -= distance,
            // Distance down and up measured from surface, so going down
            // represents getting deeper, or increasing distance.
            "up" => vertical_position -= distance,
            "down" => vertical_position += distance,
            _ => {
                return Err(PuzzleError::DataConsistencyError {
                    day: DAY,
                    puzzle: PUZZLE,
                    expected: "direction must be one of up, down, forward, backward".to_string(),
                    found: direction.to_string(),
                })
            }
        }
    }
    println!(
        "horizontal position: {}, vertical position: {}, position product: {}",
        horizontal_position,
        vertical_position,
        horizontal_position * vertical_position
    );
    Ok(())
}

fn puzzle_two_impl(
    directions_input_iter: impl Iterator<Item = Result<String, Error>>,
) -> Result<(i32, i32, i32), PuzzleError> {
    const PUZZLE: u8 = 2;
    let mut horizontal_position = 0;
    let mut vertical_position = 0;
    let mut aim = 0;
    for direction_input in directions_input_iter {
        let direction_input = direction_input?;
        let direction_pair: Vec<&str> = direction_input.split_ascii_whitespace().collect();
        if direction_pair.len() != 2 {
            return Err(PuzzleError::DataConsistencyError {
                day: DAY,
                puzzle: PUZZLE,
                expected: "directions to have exactly direction and distance".to_string(),
                found: format!("{:?}", direction_pair),
            });
        }
        let direction = direction_pair[0];
        let distance = direction_pair[1];
        let distance = distance
            .parse::<i32>()
            .map_err(|_| PuzzleError::DataConsistencyError {
                day: DAY,
                puzzle: PUZZLE,
                expected: "distance must be a valid integer".to_string(),
                found: distance.to_string(),
            })?;
        match direction {
            "forward" => {
                horizontal_position += distance;
                vertical_position += aim * distance;
            }
            // Distance down and up measured from surface, so going down
            // represents getting deeper, or increasing aim.
            "up" => aim -= distance,
            "down" => aim += distance,
            _ => {
                return Err(PuzzleError::DataConsistencyError {
                    day: DAY,
                    puzzle: PUZZLE,
                    expected: "direction must be one of up, down, forward, backward".to_string(),
                    found: direction.to_string(),
                })
            }
        }
    }
    Ok((
        horizontal_position,
        vertical_position,
        horizontal_position * vertical_position,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_two_impl() {
        let input: [Result<String, Error>; 6] = [
            Ok("forward 5".to_string()),
            Ok("down 5".to_string()),
            Ok("forward 8".to_string()),
            Ok("up 3".to_string()),
            Ok("down 8".to_string()),
            Ok("forward 2".to_string()),
        ];

        let (horizontal_position, vertical_position, position_product) =
            puzzle_two_impl(input.into_iter()).unwrap();
        assert_eq!(horizontal_position, 15);
        assert_eq!(vertical_position, 60);
        assert_eq!(position_product, 900);
    }
}
