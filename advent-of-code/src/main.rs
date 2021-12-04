use aoc_puzzles::{day_1, day_2, day_3};

fn main() {
    let result = day_1::puzzle_one();
    println!("{}", result);
    let result = day_1::puzzle_two();
    println!("{}", result);
    day_2::puzzle_one().expect("Day 2, puzzle 1 failed!");
    day_2::puzzle_two().expect("Day 2, puzzle 2 failed!");
    day_3::puzzle_one().expect("Day 3, puzzle 1 failed!");
}
