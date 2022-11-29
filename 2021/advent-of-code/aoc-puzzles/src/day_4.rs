use std::collections::{HashMap, HashSet};
use std::io::Error;

use crate::{create_data_iter, Day, Puzzle, PuzzleError};

const BINGO_FILE: &str = "day_4/bingo.txt";
const BINGO_BOARD_SIZE: usize = 5;
const DAY: Day = Day::Four;

pub fn puzzle_one() -> Result<(), PuzzleError> {
    println!("Starting day {}, puzzle {}", DAY, Puzzle::One);
    let bingo_input = create_data_iter(BINGO_FILE)?;
    let (winning_number, sum_of_uncalled_numbers, product) = puzzle_one_impl(bingo_input)?;
    println!(
        "winning number: {}, sum of uncalled numbers: {}, product: {}",
        winning_number, sum_of_uncalled_numbers, product
    );
    println!("Finished day {}, puzzle {}", DAY, Puzzle::One);
    Ok(())
}

pub fn puzzle_two() -> Result<(), PuzzleError> {
    println!("Starting day {}, puzzle {}", DAY, Puzzle::Two);
    let bingo_input = create_data_iter(BINGO_FILE)?;
    let (winning_number, sum_of_uncalled_numbers, product) = puzzle_two_impl(bingo_input)?;
    println!(
        "winning number: {}, sum of uncalled numbers: {}, product: {}",
        winning_number, sum_of_uncalled_numbers, product
    );
    println!("Finished day {}, puzzle {}", DAY, Puzzle::One);
    Ok(())
}

fn puzzle_one_impl(
    mut bingo_input: impl Iterator<Item = Result<String, Error>>,
) -> Result<(u16, u16, u16), PuzzleError> {
    let bingo_numbers_called = bingo_input.next().unwrap()?;
    bingo_input.next().unwrap()?;
    let mut numbers_to_boards = HashMap::new();
    let mut boards = vec![];
    let mut bingo_board = Some(BingoBoard::new(BINGO_BOARD_SIZE));
    let mut row = 0;

    for line in bingo_input {
        let line = line?.trim().to_string();
        if line.is_empty() {
            println!(
                "Finished building board {}. Board: {:?}",
                boards.len(),
                bingo_board.as_ref().unwrap()
            );
            row = 0;
            boards.push(bingo_board.take().unwrap());
            bingo_board = Some(BingoBoard::new(BINGO_BOARD_SIZE));
            continue;
        }

        let mut col = 0;
        for number in line.split_ascii_whitespace() {
            let number = number
                .parse::<u8>()
                .map_err(|_| PuzzleError::DataConsistencyError {
                    day: DAY,
                    puzzle: Puzzle::One,
                    expected: "bingo numbers to be parseable as u8s".to_string(),
                    found: number.to_string(),
                })?;
            bingo_board
                .as_mut()
                .unwrap()
                .add_number(number as u16, row, col);
            numbers_to_boards
                .entry(number)
                .or_insert_with(Vec::new)
                .push(boards.len());
            col += 1;
        }
        row += 1;
    }

    if bingo_board.is_some() {
        boards.push(bingo_board.unwrap());
    }

    for bingo_number in bingo_numbers_called.split(",") {
        let bingo_number =
            bingo_number
                .parse::<u16>()
                .map_err(|_| PuzzleError::DataConsistencyError {
                    day: DAY,
                    puzzle: Puzzle::One,
                    expected: "bingo numbers to be parseable as u16s".to_string(),
                    found: bingo_number.to_string(),
                })?;
        for board_list in numbers_to_boards.get(&(bingo_number as u8)) {
            for &board_index in board_list {
                let board = &mut boards[board_index];
                println!(
                    "Evaluating board {} for number {}",
                    board_index, bingo_number
                );
                if board.mark_number_called(bingo_number) {
                    println!("Board {} wins on number {}!", board_index, bingo_number);
                    let sum_of_uncalled_numbers = board.calculate_unmarked_number_sum();
                    return Ok((
                        bingo_number,
                        sum_of_uncalled_numbers,
                        bingo_number * sum_of_uncalled_numbers,
                    ));
                }
                println!(
                    "Board {} does not win on number {}",
                    board_index, bingo_number
                );
            }
        }
    }

    Err(PuzzleError::NoSolutionFound)
}

fn puzzle_two_impl(
    mut bingo_input: impl Iterator<Item = Result<String, Error>>,
) -> Result<(u16, u16, u16), PuzzleError> {
    let bingo_numbers_called = bingo_input.next().unwrap()?;
    bingo_input.next().unwrap()?;
    let mut numbers_to_boards = HashMap::new();
    let mut boards = vec![];
    let mut bingo_board = Some(BingoBoard::new(BINGO_BOARD_SIZE));
    let mut row = 0;

    for line in bingo_input {
        let line = line?.trim().to_string();
        if line.is_empty() {
            println!(
                "Finished building board {}. Board: {:?}",
                boards.len(),
                bingo_board.as_ref().unwrap()
            );
            row = 0;
            boards.push(bingo_board.take().unwrap());
            bingo_board = Some(BingoBoard::new(BINGO_BOARD_SIZE));
            continue;
        }

        let mut col = 0;
        for number in line.split_ascii_whitespace() {
            let number = number
                .parse::<u8>()
                .map_err(|_| PuzzleError::DataConsistencyError {
                    day: DAY,
                    puzzle: Puzzle::Two,
                    expected: "bingo numbers to be parseable as u8s".to_string(),
                    found: number.to_string(),
                })?;
            bingo_board
                .as_mut()
                .unwrap()
                .add_number(number as u16, row, col);
            numbers_to_boards
                .entry(number)
                .or_insert_with(Vec::new)
                .push(boards.len());
            col += 1;
        }
        row += 1;
    }

    if bingo_board.is_some() {
        boards.push(bingo_board.unwrap());
    }

    let mut unfinished_board_set = HashSet::with_capacity(boards.len());
    for i in 0..boards.len() {
        unfinished_board_set.insert(i);
    }

    let mut last_winning_board_index = 0;
    let mut number_called = 0;
    for bingo_number in bingo_numbers_called.split(",") {
        let bingo_number =
            bingo_number
                .parse::<u16>()
                .map_err(|_| PuzzleError::DataConsistencyError {
                    day: DAY,
                    puzzle: Puzzle::Two,
                    expected: "bingo numbers to be parseable as u16s".to_string(),
                    found: bingo_number.to_string(),
                })?;
        for board_list in numbers_to_boards.get(&(bingo_number as u8)) {
            for &board_index in board_list {
                let board = &mut boards[board_index];
                if unfinished_board_set.contains(&board_index)
                    && board.mark_number_called(bingo_number)
                {
                    unfinished_board_set.remove(&board_index);
                    println!(
                        "Board {} wins on number {} but there are more numbers to check.!",
                        board_index, bingo_number
                    );
                    last_winning_board_index = board_index;
                    number_called = bingo_number;
                }
            }
        }
    }

    let sum_of_uncalled_numbers = boards[last_winning_board_index].calculate_unmarked_number_sum();
    Ok((
        number_called,
        sum_of_uncalled_numbers,
        number_called * sum_of_uncalled_numbers,
    ))
}

#[derive(Debug)]
struct BingoBoard {
    numbers: HashSet<u16>,
    number_to_lines: HashMap<u16, Vec<usize>>,
    lines: Vec<u16>,
    board_size: usize,
}

impl BingoBoard {
    pub fn new(board_size: usize) -> Self {
        Self {
            numbers: HashSet::new(),
            number_to_lines: HashMap::new(),
            lines: vec![board_size as u16; board_size * 2],
            board_size,
        }
    }

    pub fn add_number(&mut self, number: u16, x_position: usize, y_position: usize) {
        self.numbers.insert(number);
        let number_to_line_vec = self.number_to_lines.entry(number).or_insert_with(Vec::new);
        // X coordinates
        number_to_line_vec.push(x_position);

        // Y coordinates. Add board size to y position
        number_to_line_vec.push(y_position + self.board_size);
    }

    pub fn mark_number_called(&mut self, number: u16) -> bool {
        if !self.numbers.remove(&number) {
            return false;
        }
        let impacted_lines = &self.number_to_lines[&number];
        for &line in impacted_lines {
            self.lines[line] -= 1;
            if self.lines[line] == 0 {
                return true;
            }
        }
        false
    }

    pub fn calculate_unmarked_number_sum(&self) -> u16 {
        self.numbers.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_input() -> [Result<String, Error>; 20] {
        [
            Ok(
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1"
                    .to_string(),
            ),
            Ok(" ".to_string()),
            Ok("22 13 17 11  0".to_string()),
            Ok("8  2 23  4 24".to_string()),
            Ok("21  9 14 16  7".to_string()),
            Ok("6 10  3 18  5".to_string()),
            Ok("1 12 20 15 19".to_string()),
            Ok("".to_string()),
            Ok("3 15  0  2 22".to_string()),
            Ok("9 18 13 17  5".to_string()),
            Ok("19  8  7 25 23".to_string()),
            Ok("20 11 10 24  4".to_string()),
            Ok("14 21 16 12  6".to_string()),
            Ok("  ".to_string()),
            Ok("14 21 17 24  4".to_string()),
            Ok("10 16 15  9 19".to_string()),
            Ok("18  8 23 26 20".to_string()),
            Ok("22 11 13  6  5".to_string()),
            Ok(" 2  0 12  3  7".to_string()),
            Ok("".to_string()),
        ]
    }

    #[test]
    fn test_puzzle_one_impl() {
        let input = create_input();
        let (number_called, sum_of_uncalled_numbers, product) =
            puzzle_one_impl(input.into_iter()).unwrap();
        assert_eq!(number_called, 24);
        assert_eq!(sum_of_uncalled_numbers, 188);
        assert_eq!(product, 4512);
    }

    #[test]
    fn test_puzzle_two_impl() {
        let input = create_input();
        let (number_called, sum_of_uncalled_numbers, product) =
            puzzle_two_impl(input.into_iter()).unwrap();
        assert_eq!(number_called, 13);
        assert_eq!(sum_of_uncalled_numbers, 148);
        assert_eq!(product, 1924);
    }
}
