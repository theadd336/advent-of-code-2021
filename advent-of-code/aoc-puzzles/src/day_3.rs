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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_puzzle_one_impl() {
        let input: [Result<String, Error>; 12] = [
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
        ];

        let (gamma, epsilon, product) = puzzle_one_impl(input.into_iter()).unwrap();
        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
        assert_eq!(product, 198);
    }
}
