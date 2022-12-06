use crate::{Part, Puzzle, PuzzleError};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        if self.start <= other.start && self.end >= other.end {
            return true;
        }
        false
    }

    fn overlaps_with(&self, other: &Self) -> bool {
        if other.contains(&self) {
            return true;
        }
        if self.start <= other.start && self.end >= other.start {
            return true;
        }
        if self.end >= other.end && self.start <= other.end {
            return true;
        }
        false
    }
}

struct ElfPair {
    range_1: Range,
    range_2: Range,
}

impl TryFrom<&str> for ElfPair {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut ranges = value.split(',').map(|str_pair| {
            let mut range = str_pair.split('-');
            let start_point: i32 = range.next().unwrap().parse().unwrap();
            let end_point: i32 = range.next().unwrap().parse().unwrap();
            Range {
                start: start_point,
                end: end_point,
            }
        });
        let range_1 = ranges.next().unwrap();
        let range_2 = ranges.next().unwrap();
        Ok(ElfPair { range_1, range_2 })
    }
}

fn puzzle_one(input: Box<dyn Iterator<Item = String>>) -> Result<String, PuzzleError> {
    let mut ans = 0;
    for line in input {
        let elf_pair = ElfPair::try_from(line.as_str()).unwrap();
        if elf_pair.range_1.contains(&elf_pair.range_2)
            || elf_pair.range_2.contains(&elf_pair.range_1)
        {
            ans += 1;
        }
    }
    Ok(ans.to_string())
}

fn puzzle_two(input: Box<dyn Iterator<Item = String>>) -> Result<String, PuzzleError> {
    let mut ans = 0;
    for line in input {
        let elf_pair = ElfPair::try_from(line.as_str()).unwrap();
        if elf_pair.range_1.overlaps_with(&elf_pair.range_2) {
            ans += 1;
        }
    }
    Ok(ans.to_string())
}

#[cfg(test)]
mod tests {
    use super::Range;

    #[test]
    fn test_range_overlaps_with() {
        // No overlap, should be false
        let range_1 = Range { start: 0, end: 5 };
        let range_2 = Range { start: 6, end: 10 };
        assert!(!range_1.overlaps_with(&range_2));
        assert!(!range_2.overlaps_with(&range_1));

        // One sided overlap, should be true
        let range_1 = Range { start: 0, end: 5 };
        let range_2 = Range { start: 5, end: 6 };
        assert!(range_1.overlaps_with(&range_2));
        assert!(range_2.overlaps_with(&range_1));

        // One sided overlap, should be true
        let range_1 = Range { start: 0, end: 5 };
        let range_2 = Range { start: -1, end: 0 };
        assert!(range_1.overlaps_with(&range_2));
        assert!(range_2.overlaps_with(&range_1));

        // One range contains another, should be true
        let range_1 = Range { start: 0, end: 5 };
        let range_2 = Range { start: 2, end: 3 };
        assert!(range_1.overlaps_with(&range_2));
        assert!(range_2.overlaps_with(&range_1));
    }
}
