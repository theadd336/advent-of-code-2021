use std::io::Error;

use crate::{create_data_iter, Day, Puzzle, PuzzleError};

const LANTERNFISH_AGES_FILE: &str = "day_6/lanternfish_ages.txt";
const DAY: Day = Day::Six;
const DEFAULT_SPAWN_DAYS: u8 = 6;
const NEW_FISH_SPAWN_DAYS: u8 = 8;

struct LanternFish {
    days_to_spawn: u8,
}

impl LanternFish {
    pub fn new() -> Self {
        Self {
            days_to_spawn: NEW_FISH_SPAWN_DAYS,
        }
    }

    pub fn new_with_days_to_spawn(days_to_spawn: u8) -> Self {
        Self { days_to_spawn }
    }

    pub fn next_day_with_spawn_check(&mut self) -> bool {
        if self.days_to_spawn == 0 {
            self.days_to_spawn = DEFAULT_SPAWN_DAYS;
            return true;
        }
        self.days_to_spawn -= 1;
        return false;
    }
}

fn create_initial_lanternfish_list(
    lanternfish_ages: impl Iterator<Item = Result<String, Error>>,
    puzzle: Puzzle,
) -> Result<Vec<u8>, PuzzleError> {
    let mut lanternfish_vec = vec![];
    for age_line in lanternfish_ages {
        let ages = age_line?;
        for age in ages.trim().split(",") {
            let age = age
                .parse::<u8>()
                .map_err(|_| PuzzleError::DataConsistencyError {
                    day: DAY,
                    puzzle: puzzle,
                    expected: "lanternfish ages to be parseable as u8".to_string(),
                    found: age.to_string(),
                })?;
            lanternfish_vec.push(age);
        }
    }
    Ok(lanternfish_vec)
}

fn puzzle_one_impl(
    lanternfish_ages: impl Iterator<Item = Result<String, Error>>,
    simulation_days: u32,
) -> Result<usize, PuzzleError> {
    let mut lanternfish_vec: Vec<LanternFish> =
        create_initial_lanternfish_list(lanternfish_ages, Puzzle::One)?
            .into_iter()
            .map(|age| LanternFish::new_with_days_to_spawn(age))
            .collect();
    let mut new_spawns = vec![];
    for _ in 0..simulation_days {
        for lanternfish in lanternfish_vec.iter_mut() {
            if lanternfish.next_day_with_spawn_check() {
                new_spawns.push(LanternFish::new());
            }
        }
        lanternfish_vec.extend(new_spawns.drain(..))
    }
    Ok(lanternfish_vec.len())
}

fn puzzle_two_impl(
    lanternfish_ages: impl Iterator<Item = Result<String, Error>>,
    simulation_days: u32,
) -> Result<u64, PuzzleError> {
    let mut lanternfish_buckets: Vec<u64> = vec![0; 9];

    for age in create_initial_lanternfish_list(lanternfish_ages, Puzzle::Two)? {
        lanternfish_buckets[age as usize] += 1;
    }

    for _ in 0..simulation_days {
        let day_0_count = lanternfish_buckets[0];

        // Move everything down one
        for bucket in 0..=7 {
            lanternfish_buckets[bucket] = lanternfish_buckets[bucket + 1];
        }
        lanternfish_buckets[8] = day_0_count;
        lanternfish_buckets[6] += day_0_count;
    }

    Ok(lanternfish_buckets.into_iter().sum::<u64>())
}

pub fn puzzle_one() -> Result<(), PuzzleError> {
    println!("Starting day {}, puzzle {}", DAY, Puzzle::One);
    let lanternfish_ages = create_data_iter(LANTERNFISH_AGES_FILE)?;
    let lanternfish_count = puzzle_one_impl(lanternfish_ages, 80)?;
    println!("Lanternfish after 80 days: {}", lanternfish_count);
    println!("Finished day {}, puzzle {}", DAY, Puzzle::One);
    Ok(())
}

pub fn puzzle_two() -> Result<(), PuzzleError> {
    println!("Starting day {}, puzzle {}", DAY, Puzzle::Two);
    let lanternfish_ages = create_data_iter(LANTERNFISH_AGES_FILE)?;
    let lanternfish_count = puzzle_two_impl(lanternfish_ages, 256)?;
    println!("Lanternfish after 80 days: {}", lanternfish_count);
    println!("Finished day {}, puzzle {}", DAY, Puzzle::Two);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_input() -> [Result<String, Error>; 1] {
        [Ok("3,4,3,1,2".to_string())]
    }

    #[test]
    fn test_puzzle_one_impl_18_days() {
        let input = create_input();
        let lanternfish_count = puzzle_one_impl(input.into_iter(), 18).unwrap();
        assert_eq!(lanternfish_count, 26);
    }

    #[test]
    fn test_puzzle_one_impl_80_days() {
        let input = create_input();
        let lanternfish_count = puzzle_one_impl(input.into_iter(), 80).unwrap();
        assert_eq!(lanternfish_count, 5934);
    }

    #[test]
    fn test_puzzle_two_impl_256_days() {
        let input = create_input();
        let lanternfish_count = puzzle_two_impl(input.into_iter(), 256).unwrap();
        assert_eq!(lanternfish_count, 26984457539);
    }
}
