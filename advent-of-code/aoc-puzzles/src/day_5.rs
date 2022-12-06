use std::collections::*;

pub use crate::{Part, Puzzle, PuzzleError};

struct Parser {
    stacks: Vec<VecDeque<char>>,
    preserve_crate_order_on_move: bool,
}

impl Parser {
    fn parse_grid_line(&mut self, line: &str, initialize_stacks: bool) {
        let mut chars = line.chars();
        let mut maybe_crate = ['\0'; 3];
        let mut stack_pos = 0;
        loop {
            for i in 0..3 {
                if let Some(letter) = chars.next() {
                    maybe_crate[i] = letter;
                } else {
                    return;
                }
            }
            // We have a crate or blank. If initialize_stacks, push a stack here
            if initialize_stacks {
                self.stacks.push(VecDeque::new());
            }
            // Actual crate. Push it to the bottom of the stack
            if maybe_crate[0] == '[' {
                let crate_id = maybe_crate[1];
                self.stacks[stack_pos].push_front(crate_id);
            }
            chars.next();
            stack_pos += 1;
        }
    }

    fn parse_move_line(&mut self, line: &str) {
        let line_pieces = line.split(' ');
        let mut move_amount = 0;
        let mut source_stack: usize = 0;
        let mut dest_stack: usize = 0;
        for (index, entry) in line_pieces.enumerate() {
            match index {
                0 | 2 | 4 => continue,
                1 => move_amount = entry.parse().unwrap(),
                3 => source_stack = entry.parse::<usize>().unwrap() - 1,
                5 => dest_stack = entry.parse::<usize>().unwrap() - 1,
                _ => panic!("Failed to parse move line: {}", line),
            }
        }
        let mut crates = Vec::with_capacity(move_amount);
        for _ in 0..move_amount {
            let crate_ = self.stacks[source_stack].pop_back().unwrap();
            crates.push(crate_);
        }
        if self.preserve_crate_order_on_move {
            crates.reverse();
        }
        self.stacks[dest_stack].extend(crates);
    }

    pub fn new(preserve_crate_order_on_move: bool) -> Self {
        Self {
            stacks: vec![],
            preserve_crate_order_on_move,
        }
    }

    pub fn parse(&mut self, input: Box<dyn Iterator<Item = String>>) {
        for (index, line) in input.enumerate() {
            if line.starts_with('[') {
                self.parse_grid_line(&line, index == 0);
            } else if line.starts_with('m') {
                self.parse_move_line(&line);
            }
        }
    }

    pub fn top(&self) -> String {
        let mut output = String::with_capacity(self.stacks.len());
        for stack in &self.stacks {
            if let Some(top) = stack.back() {
                output.push(*top);
            }
        }
        output
    }
}

fn puzzle_one(input: Box<dyn Iterator<Item = String>>) -> Result<String, PuzzleError> {
    let mut parser = Parser::new(false);
    parser.parse(input);
    println!("{}", parser.top());
    Ok(parser.top())
}

fn puzzle_two(input: Box<dyn Iterator<Item = String>>) -> Result<String, PuzzleError> {
    let mut parser = Parser::new(true);
    parser.parse(input);
    Ok(parser.top())
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
