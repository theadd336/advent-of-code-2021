use std::collections::HashSet;
use std::io::Error;

use crate::{create_data_iter, Day, Puzzle, PuzzleError};

const DIAGNOSTICS_FILE: &str = "day_3/diagnostic.txt";
const DAY: Day = Day::Three;

pub fn puzzle_one() -> Result<(), PuzzleError> {
    println!("Starting day {}, puzzle {}", DAY, Puzzle::One);
    let diagnostics_input = create_data_iter(DIAGNOSTICS_FILE)?;
    let (gamma, epsilon, product) = puzzle_one_impl(diagnostics_input)?;
    println!(
        "gamma: {}, epsilon: {}, product: {}",
        gamma, epsilon, product
    );
    println!("Finished day {}, puzzle {}", DAY, Puzzle::One);
    Ok(())
}

pub fn puzzle_two() -> Result<(), PuzzleError> {
    println!("Starting day {}, puzzle {}", DAY, Puzzle::Two);
    let diagnostics_input = create_data_iter(DIAGNOSTICS_FILE)?;
    let (oxygen_generator_rating, co2_scrubber_rating, product) =
        puzzle_two_impl(diagnostics_input)?;

    println!(
        "Oxygen Generator Rating: {}, CO2 Scrubber Rating: {}, product: {}",
        oxygen_generator_rating, co2_scrubber_rating, product
    );
    Ok(())
}

fn puzzle_one_impl(
    diagnostic_input: impl Iterator<Item = Result<String, Error>>,
) -> Result<(u32, u32, u32), PuzzleError> {
    let mut bit_counts: Vec<u32> = vec![];
    let mut num_bytes: u32 = 0;
    for byte in diagnostic_input {
        let byte = byte?;
        num_bytes += 1;

        for (index, bit) in byte.chars().enumerate() {
            if index == bit_counts.len() {
                bit_counts.push(0);
            }
            match bit {
                '1' => bit_counts[index] += 1,
                '0' => {}
                _ => {
                    return Err(PuzzleError::DataConsistencyError {
                        day: DAY,
                        puzzle: Puzzle::One,
                        expected: "Bit must be either 1 or 0".to_string(),
                        found: bit.to_string(),
                    });
                }
            }
        }
    }

    println!("Bit counts: {:?}", bit_counts);

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    let bit_pivot_point = num_bytes / 2;
    println!("Bit pivot point: {}", bit_pivot_point);
    // Reverse the counts as the vector's endianness is backwards from how the
    // final numbers are calculated.
    for (power, bit_count) in bit_counts.into_iter().rev().enumerate() {
        if bit_count >= bit_pivot_point {
            gamma += 2u32.pow(power as u32);
        } else {
            epsilon += 2u32.pow(power as u32);
        }
    }
    Ok((gamma, epsilon, gamma * epsilon))
}

fn puzzle_two_impl(
    diagnostic_input: impl Iterator<Item = Result<String, Error>>,
) -> Result<(u32, u32, u32), PuzzleError> {
    let mut bit_position_tracker = BitPositionTracker::new();
    for byte in diagnostic_input {
        let byte = byte?;
        bit_position_tracker.add_byte(&byte)?;
    }

    println!("Bit position tracker: {:?}", bit_position_tracker);

    let mut found_oxygen_rating = false;
    let mut found_co2_rating = false;
    let mut position = 0;
    let mut possible_oxygen_ratings = None;
    let mut possible_co2_ratings = None;
    while !found_co2_rating && !found_oxygen_rating {
        let bit_0_count = bit_position_tracker.get_bit_count_by_position(position, 0);
        let bit_1_count = bit_position_tracker.get_bit_count_by_position(position, 1);

        let most_common_bit;
        if bit_0_count > bit_1_count {
            most_common_bit = 0;
        } else {
            most_common_bit = 1;
        }
        println!(
            "Position {}, most common bit: {}",
            position, most_common_bit
        );
        if position == 0 {
            possible_oxygen_ratings = Some(
                bit_position_tracker
                    .get_bytes_with_bit_at_position(position, most_common_bit)
                    .clone(),
            );
            possible_co2_ratings = Some(
                bit_position_tracker
                    .get_bytes_with_bit_at_position(position, 1 - most_common_bit)
                    .clone(),
            );
        } else {
            if !found_oxygen_rating {
                println!("Evaluating oxygen at position {}", position);
                let bits_at_position =
                    bit_position_tracker.get_bytes_with_bit_at_position(position, most_common_bit);

                println!(
                    "Position {}, oxygen bytes: {:?}, oxygen bytes at position: {:?}",
                    position,
                    possible_oxygen_ratings.as_ref().unwrap(),
                    bits_at_position
                );
                possible_oxygen_ratings = Some(
                    possible_oxygen_ratings
                        .as_ref()
                        .unwrap()
                        .intersection(bits_at_position)
                        .cloned()
                        .collect(),
                )
            }
            if !found_co2_rating {
                possible_co2_ratings = Some(
                    possible_co2_ratings
                        .as_ref()
                        .unwrap()
                        .intersection(
                            bit_position_tracker
                                .get_bytes_with_bit_at_position(position, 1 - most_common_bit),
                        )
                        .cloned()
                        .collect(),
                )
            }
        }

        if possible_oxygen_ratings.as_ref().unwrap().len() == 1 {
            found_oxygen_rating = true;
        }
        if possible_co2_ratings.as_ref().unwrap().len() == 1 {
            found_co2_rating = true;
        }
        position += 1;
    }

    let mut possible_oxygen_ratings = possible_oxygen_ratings.unwrap();
    let mut possible_co2_ratings = possible_co2_ratings.unwrap();
    let oxygen_generator_rating = possible_oxygen_ratings.drain().next().unwrap();
    let oxygen_generator_rating =
        u32::from_str_radix(&oxygen_generator_rating, 2).map_err(|_| {
            PuzzleError::DataConsistencyError {
                day: DAY,
                puzzle: Puzzle::Two,
                expected: "Byte strings to contain only 1s or 0s.".to_string(),
                found: oxygen_generator_rating,
            }
        })?;

    let co2_scrubber_rating = possible_co2_ratings.drain().next().unwrap();
    let co2_scrubber_rating = u32::from_str_radix(&co2_scrubber_rating, 2).map_err(|_| {
        PuzzleError::DataConsistencyError {
            day: DAY,
            puzzle: Puzzle::Two,
            expected: "Byte strings to contain only 1s or 0s.".to_string(),
            found: co2_scrubber_rating,
        }
    })?;

    Ok((
        oxygen_generator_rating,
        co2_scrubber_rating,
        oxygen_generator_rating * co2_scrubber_rating,
    ))
}

#[derive(Debug)]
struct BitPositionTracker {
    bit_0: Vec<HashSet<String>>,
    bit_1: Vec<HashSet<String>>,
}
impl BitPositionTracker {
    pub fn new() -> Self {
        Self {
            bit_0: vec![HashSet::new()],
            bit_1: vec![HashSet::new()],
        }
    }

    pub fn add_byte(&mut self, byte_str: &String) -> Result<(), PuzzleError> {
        for (position, bit) in byte_str.chars().enumerate() {
            if position == self.bit_0.len() {
                self.bit_0.push(HashSet::new());
                self.bit_1.push(HashSet::new());
            }
            match bit {
                '1' => self.bit_1[position].insert(byte_str.clone()),
                '0' => self.bit_0[position].insert(byte_str.clone()),
                _ => {
                    return Err(PuzzleError::DataConsistencyError {
                        day: DAY,
                        puzzle: Puzzle::One,
                        expected: "Bit must be either 1 or 0".to_string(),
                        found: bit.to_string(),
                    });
                }
            };
        }
        Ok(())
    }

    pub fn get_bytes_with_bit_at_position(&self, position: usize, bit: u8) -> &HashSet<String> {
        if bit == 0 {
            &self.bit_0[position]
        } else {
            &self.bit_1[position]
        }
    }

    pub fn get_bit_count_by_position(&self, position: usize, bit: u8) -> usize {
        if bit == 0 {
            self.bit_0[position].len()
        } else {
            self.bit_1[position].len()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_input() -> [Result<String, Error>; 12] {
        [
            Ok("00100".to_string()),
            Ok("11110".to_string()),
            Ok("10110".to_string()),
            Ok("10111".to_string()),
            Ok("10101".to_string()),
            Ok("01111".to_string()),
            Ok("00111".to_string()),
            Ok("11100".to_string()),
            Ok("10000".to_string()),
            Ok("11001".to_string()),
            Ok("00010".to_string()),
            Ok("01010".to_string()),
        ]
    }
    #[test]
    fn test_puzzle_one_impl() {
        let input = create_input();
        let (gamma, epsilon, product) = puzzle_one_impl(input.into_iter()).unwrap();
        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
        assert_eq!(product, 198);
    }

    #[test]
    fn test_puzzle_two_impl() {
        let input = create_input();
        let (oxygen_generator_rating, co2_scrubber_rating, product) =
            puzzle_two_impl(input.into_iter()).unwrap();
        assert_eq!(oxygen_generator_rating, 23);
        assert_eq!(co2_scrubber_rating, 10);
        assert_eq!(product, 230);
    }
}
