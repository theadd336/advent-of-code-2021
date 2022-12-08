use std::collections::{HashMap, VecDeque};

use crate::{Part, Puzzle, PuzzleError};

struct SlidingWindow {
    size: usize,
    window: VecDeque<char>,
    uniques: HashMap<char, i32>,
}

impl SlidingWindow {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            window: VecDeque::with_capacity(size),
            uniques: HashMap::with_capacity(size),
        }
    }

    pub fn push_char(&mut self, letter: char) {
        if self.window.len() == self.size {
            let removed = self.window.pop_front().unwrap();
            if self.uniques[&removed] == 1 {
                self.uniques.remove(&removed);
            } else {
                *self.uniques.get_mut(&removed).unwrap() -= 1;
            }
        }
        self.window.push_back(letter);
        *self.uniques.entry(letter).or_insert(0) += 1;
    }

    pub fn all_unique(&self) -> bool {
        self.uniques.len() == self.size
    }
}

fn puzzle_one(input: Box<dyn Iterator<Item = String>>) -> Result<String, PuzzleError> {
    for line in input {
        let mut window = SlidingWindow::new(4);
        for (index, letter) in line.chars().enumerate() {
            window.push_char(letter);
            if window.all_unique() {
                return Ok((index + 1).to_string());
            }
        }
    }
    Err(PuzzleError::NoSolutionFound)
}

fn puzzle_two(input: Box<dyn Iterator<Item = String>>) -> Result<String, PuzzleError> {
    for line in input {
        let mut window = SlidingWindow::new(14);
        for (index, letter) in line.chars().enumerate() {
            window.push_char(letter);
            if window.all_unique() {
                return Ok((index + 1).to_string());
            }
        }
    }
    Err(PuzzleError::NoSolutionFound)
}

pub struct Solver;

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
            Part::One => puzzle_one(input),
            Part::Two => puzzle_two(input),
        }
    }
}
