use aoc_puzzles::{day_1, day_2, day_3, day_4, day_5, day_6, day_7, day_8, Day, Puzzle};

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
    day_4::puzzle_one().expect(&format!(
        "Day {}, puzzle {} failed!",
        Day::Four,
        Puzzle::One
    ));
    day_4::puzzle_two().expect(&format!(
        "Day {}, puzzle {} failed!",
        Day::Four,
        Puzzle::Two
    ));
    day_5::puzzle_one().expect(&format!(
        "Day {}, puzzle {} failed!",
        Day::Five,
        Puzzle::One
    ));
    day_5::puzzle_two().expect(&format!(
        "Day {}, puzzle {} failed!",
        Day::Five,
        Puzzle::Two
    ));
    day_6::puzzle_one().expect(&format!("Day {}, puzzle {} failed!", Day::Six, Puzzle::One));
    day_6::puzzle_two().expect(&format!("Day {}, puzzle {} failed!", Day::Six, Puzzle::Two));
    day_7::puzzle_one().expect(&format!(
        "Day {}, puzzle {} failed!",
        Day::Seven,
        Puzzle::One
    ));
    day_7::puzzle_two().expect(&format!(
        "Day {}, puzzle {} failed!",
        Day::Seven,
        Puzzle::Two
    ));
    day_8::puzzle_one().expect(&format!(
        "Day {}, puzzle {} failed!",
        Day::Eight,
        Puzzle::One
    ));
}
