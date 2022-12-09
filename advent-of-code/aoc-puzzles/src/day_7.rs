use std::mem;

use crate::{Part, Puzzle, PuzzleError};

pub struct TerminalParser {
    sizes: Vec<usize>,
    directory_stack: Vec<(String, usize)>,
    working_dir: Option<String>,
    working_size: usize,
}

impl TerminalParser {
    fn change_directory(&mut self, new_dir: &str) {
        if new_dir == ".." {
            if self.directory_stack.len() > 0 {
                let (parent_dir, parent_size) = self.directory_stack.pop().unwrap();
                self.working_dir = Some(parent_dir);
                let old_size = mem::replace(&mut self.working_size, parent_size);
                self.working_size += old_size;
                self.sizes.push(old_size);
            }
        } else {
            let child_dir = new_dir.to_string();
            // let parent_size;
            // if let Some(child_size) = self.sizes.get(&child_dir) {
            //     parent_size = mem::replace(&mut self.working_size, 0);
            // } else {
            //     parent_size = mem::replace(&mut self.working_size, 0);
            // }
            let parent_size = mem::replace(&mut self.working_size, 0);
            let parent_dir = self.working_dir.replace(child_dir);
            if let Some(parent_dir) = parent_dir {
                self.directory_stack.push((parent_dir, parent_size));
            }
        }
    }

    fn parse_command_line(&mut self, line_pieces: &[&str]) {
        if line_pieces[1] == "cd" {
            self.change_directory(line_pieces[2]);
        }
    }

    fn parse_file_line(&mut self, file_line_pieces: &[&str]) {
        let file_size: usize = file_line_pieces[0].parse().unwrap();
        self.working_size += file_size;
    }

    pub fn new() -> Self {
        Self {
            sizes: vec![],
            directory_stack: vec![],
            working_dir: None,
            working_size: 0,
        }
    }

    pub fn parse(&mut self, line: &str) {
        let line_pieces: Vec<&str> = line.split(' ').collect();
        match line_pieces[0] {
            "$" => self.parse_command_line(&line_pieces),
            "dir" => return,
            _ => self.parse_file_line(&line_pieces),
        }
    }
}

fn puzzle_one(input: Box<dyn Iterator<Item = String>>) -> Result<String, PuzzleError> {
    let mut parser = TerminalParser::new();
    for line in input {
        parser.parse(&line);
    }
    let mut sum = 0;
    for size in parser.sizes {
        if size <= 100000 {
            sum += size;
        }
    }
    Ok(sum.to_string())
}

fn puzzle_two(input: Box<dyn Iterator<Item = String>>) -> Result<String, PuzzleError> {
    const TOTAL_SPACE: usize = 70000000;
    const REQUIRED_SPACE: usize = 30000000;
    let mut parser = TerminalParser::new();
    for line in input {
        parser.parse(&line);
    }
    while parser.directory_stack.len() > 0 {
        parser.change_directory("..");
    }
    let current_free_space = TOTAL_SPACE - parser.working_size;
    if current_free_space >= REQUIRED_SPACE {
        return Err(PuzzleError::NoSolutionFound);
    }
    let space_to_free = REQUIRED_SPACE - current_free_space;
    let mut current_min_size = parser.working_size;
    for size in parser.sizes {
        if size < space_to_free {
            continue;
        }
        current_min_size = std::cmp::min(current_min_size, size);
    }

    Ok(current_min_size.to_string())
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
    use super::*;
    use std::io::{BufRead, BufReader};

    fn get_test_input() -> impl BufRead {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        BufReader::new(input.as_bytes())
    }

    #[test]
    fn test_puzzle_one() {
        let input = get_test_input();
        let res = puzzle_one(Box::new(input.lines().map(|line| line.unwrap()))).unwrap();
        println!("{}", res);
    }

    #[test]
    fn test_puzzle_two() {
        let input = get_test_input();
        let res = puzzle_two(Box::new(input.lines().map(|line| line.unwrap()))).unwrap();
        println!("{}", res);
    }
}
