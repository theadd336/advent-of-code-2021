use std::collections::BinaryHeap;

use crate::{Part, Puzzle, PuzzleError};

pub struct Solver;

impl Solver {
    fn puzzle_one(input: impl Iterator<Item = String>) -> Result<String, PuzzleError> {
        let mut max = 0;
        let mut sum = 0;
        for value in input {
            if value == "" {
                max = std::cmp::max(max, sum);
                sum = 0;
            } else {
                let value: i32 = value.parse().unwrap();
                sum += value;
            }
        }
        Ok(max.to_string())
    }

    fn puzzle_two(input: impl Iterator<Item = String>) -> Result<String, PuzzleError> {
        let mut heap = BinaryHeap::new();
        let mut sum = 0;
        for value in input {
            if value == "" {
                heap.push(sum);
                sum = 0;
            } else {
                let value: i32 = value.parse().unwrap();
                sum += value;
            }
        }
        sum = 0;
        for _ in 0..3 {
            let value = heap.pop().unwrap();
            println!("{value}");
            sum += value;
        }
        Ok(sum.to_string())
    }
}

impl Puzzle for Solver {
    fn new() -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Self)
    }

    fn solve(
        &self,
        input: Box<dyn Iterator<Item = String>>,
        part: Part,
    ) -> Result<String, PuzzleError> {
        match part {
            Part::One => Solver::puzzle_one(input),
            Part::Two => Solver::puzzle_two(input),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::puzzle_one;
//     use super::puzzle_two;
//     use std::fs::File;
//     use std::io::{BufRead, BufReader};

//     #[test]
//     fn test_puzzle_one() {
//         let input = File::open("./data/day_1/calories.txt").unwrap();
//         puzzle_one(BufReader::new(input).lines());
//     }

//     #[test]
//     fn test_puzzle_two() {
//         let input = File::open("./data/day_1/calories.txt").unwrap();
//         puzzle_two(BufReader::new(input).lines());
//     }
// }
