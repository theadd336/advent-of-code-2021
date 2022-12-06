use std::collections::HashMap;

use crate::{Part, Puzzle, PuzzleError};

#[derive(Debug, Clone, Copy)]
enum GameRes {
    Win,
    Loss,
    Tie,
}

pub struct Solver;

impl Solver {
    pub fn puzzle_one(input: Box<dyn Iterator<Item = String>>) -> Result<String, PuzzleError> {
        let values = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
        let mut score = 0;
        for line in input {
            let letters: Vec<&str> = line.split(" ").collect();
            let enemy_letter = letters[0];
            let my_letter = letters[1];
            let game_res = match (enemy_letter, my_letter) {
                ("A", "X") => GameRes::Tie,
                ("B", "Y") => GameRes::Tie,
                ("C", "Z") => GameRes::Tie,
                ("A", "Y") => GameRes::Win,
                ("B", "Z") => GameRes::Win,
                ("C", "X") => GameRes::Win,
                ("A", "Z") => GameRes::Loss,
                ("B", "X") => GameRes::Loss,
                ("C", "Y") => GameRes::Loss,
                _ => panic!(),
            };
            let res_score = match game_res {
                GameRes::Win => 6,
                GameRes::Loss => 0,
                GameRes::Tie => 3,
            };
            score += values[my_letter] + res_score;
        }
        Ok(score.to_string())
    }

    fn puzzle_two(input: Box<dyn Iterator<Item = String>>) -> Result<String, PuzzleError> {
        let values = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
        let mut score = 0;
        for line in input {
            let letters: Vec<&str> = line.split(" ").collect();
            let enemy_letter = letters[0];
            let my_letter = letters[1];
            let game_res = match my_letter {
                "X" => GameRes::Loss,
                "Y" => GameRes::Tie,
                "Z" => GameRes::Win,
                _ => panic!(),
            };
            let play_score = match (game_res, enemy_letter) {
                (GameRes::Win, "A") => values["Y"],
                (GameRes::Win, "B") => values["Z"],
                (GameRes::Win, "C") => values["X"],
                (GameRes::Tie, "A") => values["X"],
                (GameRes::Tie, "B") => values["Y"],
                (GameRes::Tie, "C") => values["Z"],
                (GameRes::Loss, "A") => values["Z"],
                (GameRes::Loss, "B") => values["X"],
                (GameRes::Loss, "C") => values["Y"],
                _ => panic!(),
            };
            let res_score = match game_res {
                GameRes::Win => 6,
                GameRes::Loss => 0,
                GameRes::Tie => 3,
            };
            score += play_score + res_score;
        }
        Ok(score.to_string())
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
        part: crate::Part,
    ) -> Result<String, crate::PuzzleError> {
        match part {
            Part::One => Solver::puzzle_one(input),
            Part::Two => Solver::puzzle_two(input),
        }
    }
}
