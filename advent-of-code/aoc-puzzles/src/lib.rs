use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};

use thiserror::Error;

pub mod day_1;
pub mod day_2;
pub mod day_3;

fn create_data_iter(relative_file_path: &str) -> Result<Lines<BufReader<File>>, Error> {
    let absolute_file_path =
        "/home/theadd336/advent-of-code/advent-of-code/data/".to_string() + relative_file_path;
    let file = File::open(absolute_file_path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("An IO error occurred while attempting to read the input data file.")]
    DataFileError(#[from] Error),
    #[error("Data consistency error found in day {day:?}, puzzle {puzzle:?}. Expected {expected:?}, found {found:?}")]
    DataConsistencyError {
        day: u8,
        puzzle: u8,
        expected: String,
        found: String,
    },
}
