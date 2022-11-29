#![allow(dead_code)]
use std::collections::{HashMap, HashSet};
use std::io::Error;

use lazy_static::lazy_static;

use crate::{create_data_iter, Day, Puzzle, PuzzleError};

const NAV_SYSTEM_LINES: &str = "day_10/nav_system_lines.txt";
const DAY: Day = Day::Ten;


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum CharType {
    Opener,
    Closer
}

#[derive(Debug, Clone, Copy)]
struct ChunkCharPair {
    char_type: CharType,
    char_pair: char
}

lazy_static! {
    static ref VALID_CHUNK_CHARS: HashMap<char, ChunkCharPair> = {
        let mut valid_chars = HashMap::new();
        // valid_chars.insert(')', ChunkCharPair {
        //     char_type: CharType::Closer,
        //     char_pair: '(')
        // });
        valid_chars
    }
    
    static ref CHUNK_CLOSERS: HashMap<char, u32> = {
        let mut closers = HashMap::new();
        closers.insert(')', 3);
        closers.insert(']', 57);
        closers.insert('}', 1197);
        closers.insert('>', 25137);
        closers
    }
}

fn parse_line_for_illegal_chunks(line: &str) -> Option<char> {
    let mut char_stack = vec![];
    for char in line.trim().chars() {
        if char_stack.len() == 0 && CHUNK_CLOSERS.contains_key(&char) {
            return Some(char);
        } else if char_stack.len() == 0 {
            char_stack.push(char);
            continue;
        }

        let most_recent_char = 
    }
    None
}

fn puzzle_one_impl(
    nav_system_lines: impl Iterator<Item = Result<String, Error>>,
) -> Result<u32, PuzzleError> {
    Ok(0)
}
