use thiserror::Error;

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    One,
    Two,
}

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("Data consistency error found in day {day}, part {part:?}. Expected {expected:?}, found {found:?}")]
    DataConsistencyError {
        day: u8,
        part: Part,
        expected: String,
        found: String,
    },
    #[error("No solution was found")]
    NoSolutionFound,
}

pub trait Puzzle {
    fn new() -> Box<Self>
    where
        Self: Sized;

    fn solve(
        &self,
        input: Box<dyn Iterator<Item = String>>,
        part: Part,
    ) -> Result<String, PuzzleError>;
}
