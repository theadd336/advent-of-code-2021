use std::cmp;
use std::io::Error;

use crate::{create_data_iter, Day, Puzzle, PuzzleError};

const GEOTHERMAL_VENT_FILE: &str = "day_5/geothermal_vents.txt";
const DAY: Day = Day::Five;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct VentPoint {
    pub row: usize,
    pub col: usize,
}

impl VentPoint {
    pub fn try_from_str(input: &str, delim: &str) -> Result<Self, PuzzleError> {
        let input: Vec<&str> = input.split(delim).collect();
        if input.len() != 2 {
            return Err(PuzzleError::DataConsistencyError {
                day: DAY,
                puzzle: Puzzle::One,
                expected: format!("exactly two coordinates when separated by {}", delim),
                found: format!("{:?}", input),
            });
        }
        let point = Self {
            col: input[0]
                .parse()
                .map_err(|_| PuzzleError::DataConsistencyError {
                    day: DAY,
                    puzzle: Puzzle::Two,
                    expected: "vent points to be parseable as usize".to_string(),
                    found: input[0].to_string(),
                })?,
            row: input[1]
                .parse()
                .map_err(|_| PuzzleError::DataConsistencyError {
                    day: DAY,
                    puzzle: Puzzle::Two,
                    expected: "vent points to be parseable as usize".to_string(),
                    found: input[1].to_string(),
                })?,
        };
        Ok(point)
    }
}

#[derive(Debug)]
struct VentMap {
    vents: Vec<Vec<u32>>,
    vent_overlap_count: u32,
    include_diagonals: bool,
}

impl VentMap {
    pub fn new(num_cols: usize, num_rows: usize, include_diagonals: bool) -> Self {
        Self {
            vents: vec![vec![0; num_rows]; num_cols],
            vent_overlap_count: 0,
            include_diagonals,
        }
    }

    pub fn add_vent_line(&mut self, start_point: VentPoint, end_point: VentPoint) {
        if !self.include_diagonals
            && (start_point.row != end_point.row && start_point.col != end_point.col)
        {
            return;
        }

        let starting_col = cmp::min(start_point.col, end_point.col);
        let ending_col = cmp::max(start_point.col, end_point.col);
        let starting_row = cmp::min(start_point.row, end_point.row);
        let ending_row = cmp::max(start_point.row, end_point.row);
        for col in starting_col..=ending_col {
            for row in starting_row..=ending_row {
                let vent_count = self.vents[col][row] + 1;
                self.vents[col][row] = vent_count;
                if vent_count == 2 {
                    self.vent_overlap_count += 1;
                }
            }
        }
    }

    pub fn vent_overlap_count(&self) -> u32 {
        self.vent_overlap_count
    }
}

fn create_point_pairs()

fn puzzle_one_impl(
    geothermal_vent_input: impl Iterator<Item = Result<String, Error>>,
) -> Result<u32, PuzzleError> {
    let mut max_col = 0;
    let mut max_row = 0;
    let mut points = vec![];
    for line in geothermal_vent_input {
        let line = line?;
        let coordinate_pairs = line.trim().split(" -> ");
        let mut point_pair = vec![];
        for coordinates in coordinate_pairs {
            let point = VentPoint::try_from_str(coordinates, ",")?;
            if point.col > max_col {
                max_col = point.col;
            }
            if point.row > max_row {
                max_row = point.row;
            }
            point_pair.push(point);
        }
        points.push(point_pair);
    }

    let mut vent_map = VentMap::new(max_col + 1, max_row + 1, false);
    for point_pair in points {
        vent_map.add_vent_line(point_pair[0], point_pair[1]);
    }
    Ok(vent_map.vent_overlap_count())
}

pub fn puzzle_one() -> Result<(), PuzzleError> {
    println!("Starting day {}, puzzle {}", DAY, Puzzle::One);
    let geothermal_vent_input = create_data_iter(GEOTHERMAL_VENT_FILE)?;
    let dangerous_points = puzzle_one_impl(geothermal_vent_input)?;
    println!("Dangerous points: {}", dangerous_points);
    println!("Finished day {}, puzzle {}", DAY, Puzzle::One);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_input() -> [Result<String, Error>; 10] {
        [
            Ok("0,9 -> 5,9".to_string()),
            Ok("8,0 -> 0,8".to_string()),
            Ok("9,4 -> 3,4".to_string()),
            Ok("2,2 -> 2,1".to_string()),
            Ok("7,0 -> 7,4".to_string()),
            Ok("6,4 -> 2,0".to_string()),
            Ok("0,9 -> 2,9".to_string()),
            Ok("3,4 -> 1,4".to_string()),
            Ok("0,0 -> 8,8".to_string()),
            Ok("5,5 -> 8,2".to_string()),
        ]
    }

    #[test]
    fn test_puzzle_one_impl() {
        let input = create_input();
        let dangerous_point_count = puzzle_one_impl(input.into_iter()).unwrap();
        assert_eq!(dangerous_point_count, 5);
    }
}
