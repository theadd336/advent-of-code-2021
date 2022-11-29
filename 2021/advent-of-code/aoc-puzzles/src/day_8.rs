use std::io::Error;

use crate::{create_data_iter, Day, Puzzle, PuzzleError};

const DIGITS_FILE: &str = "day_8/digits.txt";
const DAY: Day = Day::Eight;

struct DisplayOutput {
    signal_patterns: Vec<String>,
    output: Vec<String>,
}

impl DisplayOutput {
    pub fn new() -> Self {
        Self {
            signal_patterns: Vec::with_capacity(10),
            output: Vec::with_capacity(4),
        }
    }

    pub fn add_signal_pattern(&mut self, pattern: String) {
        self.signal_patterns.push(pattern);
    }

    pub fn add_output(&mut self, output: String) {
        self.output.push(output);
    }

    pub fn output(&self) -> &Vec<String> {
        &self.output
    }
}

fn parse_digits_input(
    digits_input: impl Iterator<Item = Result<String, Error>>,
) -> Result<Vec<DisplayOutput>, PuzzleError> {
    let mut digits = vec![];
    for line in digits_input {
        let line = line?;
        let mut in_output_section = false;
        let mut display_output = DisplayOutput::new();
        for piece in line.split_ascii_whitespace() {
            if in_output_section {
                display_output.add_output(piece.to_string());
            } else if piece == "|" {
                in_output_section = true;
            } else {
                display_output.add_signal_pattern(piece.to_string());
            }
        }
        digits.push(display_output);
    }
    Ok(digits)
}

fn puzzle_one_impl(
    digits_input: impl Iterator<Item = Result<String, Error>>,
) -> Result<usize, PuzzleError> {
    let mut seen_digits = 0;
    for display_output in &parse_digits_input(digits_input)? {
        for digits in display_output.output() {
            match digits.len() {
                2 | 3 | 4 | 7 => seen_digits += 1,
                _ => continue,
            }
        }
    }
    Ok(seen_digits)
}

pub fn puzzle_one() -> Result<(), PuzzleError> {
    println!("Starting day {}, puzzle {}", DAY, Puzzle::One);
    let digits_input = create_data_iter(DIGITS_FILE)?;
    let count_of_easy_digits = puzzle_one_impl(digits_input)?;
    println!("Easy digits count: {}", count_of_easy_digits);
    println!("Finished day {}, puzzle {}", DAY, Puzzle::One);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_input() -> [Result<String, Error>; 10] {
        [Ok(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"
                .to_string(),
        ),Ok(
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"
                .to_string(),
        ),Ok(
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"
                .to_string(),
        ),Ok(
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"
                .to_string(),
        ),Ok(
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"
                .to_string(),
        ),Ok(
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"
                .to_string(),
        ),Ok(
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"
                .to_string(),
        ),Ok(
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"
                .to_string(),
        ),Ok(
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
        ),Ok(
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string(),
        )]
    }

    #[test]
    fn test_puzzle_one_impl() {
        let input = create_input();
        let count_of_easy_digits = puzzle_one_impl(input.into_iter()).unwrap();
        assert_eq!(count_of_easy_digits, 26)
    }
}
