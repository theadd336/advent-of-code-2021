use std::collections::HashSet;

use crate::{Part, Puzzle, PuzzleError};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl TryFrom<&str> for Direction {
    type Error = PuzzleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let direction = match value {
            "R" => Self::Right,
            "L" => Self::Left,
            "U" => Self::Up,
            "D" => Self::Down,
            _ => return Err(PuzzleError::ParseError(value.to_string())),
        };
        Ok(direction)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn add_x(mut self, x: i32) -> Self {
        self.x += x;
        self
    }

    pub fn add_y(mut self, y: i32) -> Self {
        self.y += y;
        self
    }

    pub fn is_adjacent_to(&self, other: &Self) -> bool {
        if (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1 {
            return true;
        }
        false
    }
}

struct RopeTracker {
    seen_positions: HashSet<Coordinate>,
    head_pos: Coordinate,
    tail_pos: Coordinate,
}

impl RopeTracker {
    fn update_positions(&mut self, direction: Direction) {
        let new_head_pos = match direction {
            Direction::Up => self.head_pos.add_y(1),
            Direction::Down => self.head_pos.add_y(-1),
            Direction::Left => self.head_pos.add_x(-1),
            Direction::Right => self.head_pos.add_x(1),
        };
        self.head_pos = new_head_pos;
        if self.head_pos.is_adjacent_to(&self.tail_pos) {
            return;
        }
        if (self.head_pos.y - self.tail_pos.y).abs() >= 1 {
            let move_amount = if self.tail_pos.y < self.head_pos.y {
                1
            } else {
                -1
            };
            self.tail_pos = self.tail_pos.add_y(move_amount);
        }
        if (self.head_pos.x - self.tail_pos.x).abs() >= 1 {
            let move_amount = if self.tail_pos.x < self.head_pos.x {
                1
            } else {
                -1
            };
            self.tail_pos = self.tail_pos.add_x(move_amount);
        }
        self.seen_positions.insert(self.tail_pos);
    }

    pub fn new() -> Self {
        Self {
            seen_positions: HashSet::from([Coordinate::default()]),
            head_pos: Coordinate::default(),
            tail_pos: Coordinate::default(),
        }
    }

    pub fn parse_instruction(&mut self, instruction: &str) -> Result<(), PuzzleError> {
        let mut chars = instruction.split(' ');
        let direction = Direction::try_from(chars.next().unwrap())?;
        let steps: i32 = chars.next().unwrap().parse().unwrap();
        for _ in 0..steps {
            self.update_positions(direction);
        }
        Ok(())
    }

    pub fn unique_tail_positions(&self) -> usize {
        self.seen_positions.len()
    }
}

fn puzzle_one(input: Box<dyn Iterator<Item = String>>) -> Result<String, PuzzleError> {
    let mut rope_tracker = RopeTracker::new();
    for line in input {
        rope_tracker.parse_instruction(&line)?;
    }
    Ok(rope_tracker.unique_tail_positions().to_string())
}

fn puzzle_two(input: Box<dyn Iterator<Item = String>>) -> Result<String, PuzzleError> {
    Ok("".to_string())
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

#[cfg(test)]
mod tests {
    use std::io::{BufRead, BufReader};

    use super::*;

    fn get_test_input() -> Box<dyn Iterator<Item = String>> {
        let test_data = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let reader = BufReader::new(test_data.as_bytes());
        Box::new(reader.lines().map(|line| line.unwrap()))
    }

    #[test]
    fn test_coordinate_add_y() {
        let coord_1 = Coordinate::new(5, 10);
        let coord_2 = coord_1.add_y(1);
        assert_eq!(coord_1.x, coord_2.x);
        assert_eq!(coord_1.y, coord_2.y - 1);
    }

    #[test]
    fn test_coordinate_add_x() {
        let coord_1 = Coordinate::new(5, 10);
        let coord_2 = coord_1.add_x(1);
        assert_eq!(coord_1.x, coord_2.x - 1);
        assert_eq!(coord_1.y, coord_2.y);
    }

    #[test]
    fn test_coordinate_is_adjacent_to() {
        let coord_1 = Coordinate::new(5, 10);
        let coord_2 = Coordinate::new(5, 10);
        assert!(coord_1.is_adjacent_to(&coord_2));

        let coord_2 = Coordinate::new(5, 11);
        assert!(coord_1.is_adjacent_to(&coord_2));

        let coord_2 = Coordinate::new(6, 11);
        assert!(coord_1.is_adjacent_to(&coord_2));

        let coord_2 = Coordinate::new(6, 10);
        assert!(coord_1.is_adjacent_to(&coord_2));

        let coord_2 = Coordinate::new(6, 9);
        assert!(coord_1.is_adjacent_to(&coord_2));

        let coord_2 = Coordinate::new(5, 9);
        assert!(coord_1.is_adjacent_to(&coord_2));

        let coord_2 = Coordinate::new(4, 9);
        assert!(coord_1.is_adjacent_to(&coord_2));

        let coord_2 = Coordinate::new(4, 10);
        assert!(coord_1.is_adjacent_to(&coord_2));

        let coord_2 = Coordinate::new(4, 11);
        assert!(coord_1.is_adjacent_to(&coord_2));

        let coord_2 = Coordinate::new(4, 8);
        assert!(!coord_1.is_adjacent_to(&coord_2));
    }

    #[test]
    fn test_puzzle_one() {
        let input = get_test_input();
        let ans = puzzle_one(input).unwrap();
        assert_eq!(ans, "13");
    }

    #[test]
    fn test_puzzle_two() {
        let input = get_test_input();
        let ans = puzzle_two(input).unwrap();
        assert_eq!(ans, "8");
    }
}
