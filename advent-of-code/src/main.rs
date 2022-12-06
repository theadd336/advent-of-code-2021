use std::io::BufReader;
use std::{fs::File, io::BufRead};

use chrono::{Datelike, Local};
use clap::{Parser, ValueEnum};

use aoc_puzzles::*;

const DATA_FILE_PATH: &str = "./advent-of-code/aoc-puzzles/data/";
const INPUT_FILE_NAME: &str = "input.txt";

fn day_num_from_today() -> u8 {
    let today = Local::now();
    today.day() as u8
}

#[derive(Copy, Clone, ValueEnum, Debug)]
enum CliPart {
    One,
    Two,
    Both,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The day to codegen
    #[arg(short, long, default_value_t=day_num_from_today())]
    day: u8,
    #[arg(short, long, value_enum, default_value_t=CliPart::Both)]
    part: CliPart,
}

fn solve_with_printout(
    day: u8,
    solver: &dyn Puzzle,
    input: impl Iterator<Item = String> + 'static,
    part: Part,
) {
    let ans = solver.solve(Box::new(input), part).unwrap();
    println!("Day {day}, part {part:?} answer: {ans}");
}

fn main() {
    let args = Args::parse();
    let day = args.day;
    let solver: Box<dyn Puzzle> = match args.day {
        1 => day_1::Solver::new(),
        2 => day_2::Solver::new(),
        3 => day_3::Solver::new(),
        4 => day_4::Solver::new(),
        5 => day_5::Solver::new(),
        _ => panic!("Unsupported test day"),
    };

    let input = File::open(format!("{DATA_FILE_PATH}/day_{day}/{INPUT_FILE_NAME}"))
        .expect(&format!("Failed to open input file for day {day}"));
    let input = BufReader::new(input)
        .lines()
        .map(|line| line.expect("Failed to read IO for the given line"));
    let input = Box::new(input);

    match args.part {
        CliPart::One => solve_with_printout(day, solver.as_ref(), input, Part::One),
        CliPart::Two => solve_with_printout(day, solver.as_ref(), input, Part::Two),
        CliPart::Both => {
            solve_with_printout(day, solver.as_ref(), input, Part::One);
            let input = File::open(format!("{DATA_FILE_PATH}/day_{day}/{INPUT_FILE_NAME}"))
                .expect(&format!("Failed to open input file for day {day}"));
            let input = BufReader::new(input)
                .lines()
                .map(|line| line.expect("Failed to read IO for the given line"));
            let input = Box::new(input);
            solve_with_printout(day, solver.as_ref(), input, Part::Two);
        }
    }
}
