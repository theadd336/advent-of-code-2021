use std::collections::*;

use crate::{Part, Puzzle, PuzzleError};

fn init_hashmaps() -> HashMap<char, i32> {
    HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),
    ])
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

fn puzzle_one(input: Box<dyn Iterator<Item = String>>) -> Result<String, PuzzleError> {
    let mut ans = 0;
    let map = init_hashmaps();
    for line in input {
        let letters: Vec<char> = line.chars().collect();
        let midpoint = letters.len() / 2;
        let first_half: HashSet<char> = letters.iter().cloned().take(midpoint).collect();
        let second_half: HashSet<char> = letters[midpoint..].iter().cloned().collect();
        let overlap: Vec<char> = first_half.intersection(&second_half).cloned().collect();
        let overlap_char = overlap[0];
        println!("{}", overlap_char);
        ans += map[&overlap_char];
    }
    Ok(ans.to_string())
}

pub fn puzzle_two(input: Box<dyn Iterator<Item = String>>) -> Result<String, PuzzleError> {
    let mut ans = 0;
    let map = init_hashmaps();
    let mut group: [HashSet<char>; 3] = [HashSet::new(), HashSet::new(), HashSet::new()];
    for (index, line) in input.enumerate() {
        let letters: HashSet<char> = line.chars().collect();
        group[index % 3] = letters;
        if (index + 1) % 3 == 0 {
            let intersect_1: HashSet<char> = group[0].intersection(&group[1]).cloned().collect();
            let intersect_2: Vec<char> = intersect_1.intersection(&group[2]).cloned().collect();
            ans += map[&intersect_2[0]];
        }
    }
    Ok(ans.to_string())
}
