use std::collections::HashSet;

use crate::{Part, Puzzle, PuzzleError};

pub struct Forest {
    tree_grid: Vec<Vec<u8>>,
}

impl Forest {
    pub fn from_input(input: Box<dyn Iterator<Item = String>>) -> Self {
        let mut tree_grid = vec![];

        for line in input {
            let mut row = Vec::with_capacity(line.len());
            for number in line.chars() {
                let height = number.to_digit(10).unwrap() as u8;
                row.push(height);
            }
            tree_grid.push(row);
        }

        Self { tree_grid }
    }
}

fn puzzle_one(input: Box<dyn Iterator<Item = String>>) -> Result<String, PuzzleError> {
    let forest = Forest::from_input(input);
    let mut seen_trees = HashSet::new();
    let mut visible_trees = forest.tree_grid.len() * 2 + forest.tree_grid[0].len() * 2 - 4;

    // Check visibility from columns
    for (row_idx, row) in forest.tree_grid[1..forest.tree_grid.len() - 1]
        .iter()
        .enumerate()
    {
        let row_idx = row_idx + 1;
        let mut tallest_tree = row[0];
        for (col_idx, &tree) in row[1..row.len() - 1].iter().enumerate() {
            let col_idx = col_idx + 1;
            if tree > tallest_tree {
                if seen_trees.insert((row_idx, col_idx)) {
                    visible_trees += 1;
                }
                tallest_tree = tree;
            }
        }

        tallest_tree = row[row.len() - 1];
        for (col_idx, &tree) in row[1..row.len() - 1].iter().rev().enumerate() {
            let col_idx = row.len() - col_idx - 2;
            if tree > tallest_tree {
                if seen_trees.insert((row_idx, col_idx)) {
                    visible_trees += 1;
                }
                tallest_tree = tree;
            }
        }
    }

    for col_idx in 1..forest.tree_grid[0].len() - 1 {
        let mut tallest_tree = forest.tree_grid[0][col_idx];
        for (row_idx, row) in forest.tree_grid[1..forest.tree_grid.len() - 1]
            .iter()
            .enumerate()
        {
            let row_idx = row_idx + 1;
            let tree_height = row[col_idx];
            if tree_height > tallest_tree {
                if seen_trees.insert((row_idx, col_idx)) {
                    visible_trees += 1;
                }
                tallest_tree = tree_height;
            }
        }

        // Backward pass
        tallest_tree = forest.tree_grid[forest.tree_grid.len() - 1][col_idx];
        for (row_idx, row) in forest.tree_grid[1..forest.tree_grid.len() - 1]
            .iter()
            .rev()
            .enumerate()
        {
            let row_idx = forest.tree_grid.len() - row_idx - 2;
            let tree_height = row[col_idx];
            if tree_height > tallest_tree {
                if seen_trees.insert((row_idx, col_idx)) {
                    visible_trees += 1;
                }
                tallest_tree = tree_height;
            }
        }
    }
    Ok(visible_trees.to_string())
}

fn puzzle_two(input: Box<dyn Iterator<Item = String>>) -> Result<String, PuzzleError> {
    let forest = Forest::from_input(input);
    let mut scenic_score = 0;
    for (row_idx, row) in forest.tree_grid.iter().enumerate() {
        for (col_idx, &tree) in row.iter().enumerate() {
            let mut down_row_vis = 0;
            let mut up_row_vis = 0;
            let mut left_col_vis = 0;
            let mut right_col_vis = 0;
            // First, move down
            for down_row in &forest.tree_grid[row_idx + 1..] {
                down_row_vis += 1;
                if down_row[col_idx] >= tree {
                    break;
                }
            }

            // Next, move up
            for up_row in forest.tree_grid[0..row_idx].iter().rev() {
                up_row_vis += 1;
                if up_row[col_idx] >= tree {
                    break;
                }
            }

            // Next, to the left
            for &left_col in row[0..col_idx].iter().rev() {
                left_col_vis += 1;
                if left_col >= tree {
                    break;
                }
            }

            // Finally, move right
            for &right_col in row[col_idx + 1..].iter() {
                right_col_vis += 1;
                if right_col >= tree {
                    break;
                }
            }

            let tree_scenic_score = down_row_vis * up_row_vis * left_col_vis * right_col_vis;
            scenic_score = std::cmp::max(tree_scenic_score, scenic_score);
        }
    }
    Ok(scenic_score.to_string())
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
    use std::io::{BufRead, BufReader};

    use super::*;

    fn get_test_input() -> Box<dyn Iterator<Item = String>> {
        let test_data = "30373
25512
65332
33549
35390";
        let reader = BufReader::new(test_data.as_bytes());
        Box::new(reader.lines().map(|line| line.unwrap()))
    }

    #[test]
    fn test_puzzle_one() {
        let input = get_test_input();
        let ans = puzzle_one(input).unwrap();
        assert_eq!(ans, "21");
    }

    #[test]
    fn test_puzzle_two() {
        let input = get_test_input();
        let ans = puzzle_two(input).unwrap();
        assert_eq!(ans, "8");
    }
}
