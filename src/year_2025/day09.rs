use anyhow::{Context, Result};
use lazy_regex::regex_captures;
use std::io::{BufRead, BufReader};

use crate::day::Day;
use crate::util::input::get_input;

#[derive(Clone, Copy)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    pub fn rect_area(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

pub struct Day09;

impl Day for Day09 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2025, 9)?);

        // Parse Points & Measure Basic Areas
        let mut red_tiles: Vec<Point> = Vec::new();
        let mut largest_area: u64 = 0;
        for (line_num, line) in input.lines().map(|l| l.unwrap()).enumerate() {
            let (_, x_str, y_str) = regex_captures!(r"([0-9]+),([0-9]+)", &line)
                .with_context(|| format!("No point on line {line_num}"))?;
            let new_tile = Point {
                x: x_str.parse()?,
                y: y_str.parse()?,
            };
            for tile in &red_tiles {
                let area = new_tile.rect_area(tile);
                if area > largest_area {
                    largest_area = area;
                }
            }
            red_tiles.push(new_tile);
        }

        println!("Largest area based on red tiles, {largest_area}");

        Ok(())
    }
}
