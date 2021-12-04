use aoc_puzzles::{day_1, day_2, day_3, Day, Puzzle};

fn main() {
    let result = day_1::puzzle_one();
    println!("{}", result);
    let result = day_1::puzzle_two();
    println!("{}", result);
    day_2::puzzle_one().expect(&format!("Day {}, puzzle {} failed!", Day::Two, Puzzle::One));
    day_2::puzzle_two().expect(&format!("Day {}, puzzle {} failed!", Day::Two, Puzzle::Two));
    day_3::puzzle_one().expect(&format!(
        "Day {}, puzzle {} failed!",
        Day::Three,
        Puzzle::One
    ));
}
