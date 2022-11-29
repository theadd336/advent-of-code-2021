use std::collections::VecDeque;

use crate::create_data_iter;

const DEPTH_DATA: &str = "day_1/depths.txt";

pub fn puzzle_one() -> u32 {
    let mut depth_increases = 0;
    let mut depth_data_iter = create_data_iter(DEPTH_DATA).unwrap();
    let mut prior_depth = str::parse::<u32>(&depth_data_iter.next().unwrap().unwrap()).unwrap();
    for depth in depth_data_iter {
        let depth = str::parse::<u32>(&depth.unwrap()).unwrap();
        if depth > prior_depth {
            depth_increases += 1;
        }
        prior_depth = depth;
    }
    depth_increases
}

pub fn puzzle_two() -> i32 {
    let mut depth_data_iter = create_data_iter(DEPTH_DATA).unwrap();
    let mut sliding_window = SlidingWindow::new(3);
    for _ in 0..3 {
        let depth = str::parse::<i32>(&depth_data_iter.next().unwrap().unwrap()).unwrap();
        sliding_window.add_element(depth);
    }

    let mut prior_depth = sliding_window.current_sum();
    let mut depth_increases = 0;
    for depth in depth_data_iter {
        let depth = str::parse::<i32>(&depth.unwrap()).unwrap();
        sliding_window.add_element(depth);
        let current_depth = sliding_window.current_sum();
        if current_depth > prior_depth {
            depth_increases += 1;
        }
        prior_depth = current_depth;
    }
    depth_increases
}

struct SlidingWindow {
    current_sum: i32,
    data: VecDeque<i32>,
}

impl SlidingWindow {
    pub fn new(window_size: usize) -> Self {
        Self {
            current_sum: 0,
            data: VecDeque::with_capacity(window_size),
        }
    }

    pub fn current_sum(&self) -> i32 {
        self.current_sum
    }

    pub fn add_element(&mut self, element: i32) -> Option<i32> {
        if self.data.len() < self.data.capacity() {
            self.data.push_back(element);
            self.current_sum += element;
            return None;
        } else {
            let evicted_element = self.data.pop_front().unwrap();
            self.data.push_back(element);
            self.current_sum += element - evicted_element;
            return Some(evicted_element);
        }
    }
}
