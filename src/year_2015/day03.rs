use crate::util::input::get_input_string;
use std::collections::HashMap;

use anyhow::Result;

use crate::data::Point2D;
use crate::day::Day;

pub struct Day03;

impl Day for Day03 {
    fn main() -> Result<()> {
        let input = get_input_string(2015, 03)?;
        let mut y1_santa: Point2D<i32> = Point2D::default();
        let mut y2_santas: [Point2D<i32>; 2] = [Point2D::default(), Point2D::default()];
        let mut y1_visited: HashMap<Point2D<i32>, usize> = HashMap::new();
        let mut y2_visited: HashMap<Point2D<i32>, usize> = HashMap::new();
        y1_visited
            .entry(y1_santa)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        y2_visited
            .entry(y2_santas[0])
            .and_modify(|count| *count += 1)
            .or_insert(1);
        for (idx, ch) in input.chars().enumerate() {
            let vector: (i32, i32) = match ch {
                '^' => (0, -1),
                'v' => (0, 1),
                '>' => (1, 0),
                '<' => (-1, 0),
                _ => (0, 0),
            };
            y1_santa += vector;
            y2_santas[idx % 2] += vector;
            y1_visited
                .entry(y1_santa)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            y2_visited
                .entry(y2_santas[idx % 2])
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        println!("Year 1 Has Present Count: {}", y1_visited.len());
        println!("Year 2 Has Present Count: {}", y2_visited.len());
        Ok(())
    }
}
