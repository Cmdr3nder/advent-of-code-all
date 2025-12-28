use anyhow::{Context, Result};
use lazy_regex::regex_captures;
use std::cmp::{Ordering, PartialOrd};
use std::io::{BufRead, BufReader};
use std::u64;

use crate::day::Day;
use crate::util::input::get_input;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, Hash)]
struct Rectangle {
    a: Point,
    b: Point,
}

impl Rectangle {
    fn area(&self) -> u64 {
        (self.a.x.abs_diff(self.b.x) + 1) * (self.a.y.abs_diff(self.b.y) + 1)
    }
}

impl PartialOrd for Rectangle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.area()
                .cmp(&other.area())
                .then(self.a.cmp(&other.a))
                .then(self.b.cmp(&other.b)),
        )
    }
}

pub struct Day09;

impl Day for Day09 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2025, 9)?);

        // Parse Points & Measure Basic Areas
        let mut red_tiles: Vec<Point> = Vec::new();
        let mut outline: Vec<Rectangle> = Vec::new();
        let mut red_zones: Vec<Rectangle> = Vec::new();
        for (line_num, line) in input.lines().map(|l| l.unwrap()).enumerate() {
            let (_, x_str, y_str) = regex_captures!(r"([0-9]+),([0-9]+)", &line)
                .with_context(|| format!("No point on line {line_num}"))?;
            let new_tile = Point {
                x: x_str.parse()?,
                y: y_str.parse()?,
            };
            // Spawn red cornered rectangles
            for tile in &red_tiles {
                red_zones.push(Rectangle {
                    a: new_tile,
                    b: *tile,
                });
            }
            // Draw outline
            if let Some(prev) = red_tiles.last() {
                outline.push(Rectangle {
                    a: *prev,
                    b: new_tile,
                });
            }
            red_tiles.push(new_tile);
        }

        // Complete outline
        if let Some((first, last)) = red_tiles.first().zip(red_tiles.last()) {
            outline.push(Rectangle {
                a: *last,
                b: *first,
            });
        }

        // Sort red cornered rectangles
        red_zones.sort_unstable();
        red_zones.reverse();

        println!(
            "Largest area based on red tiles, {}",
            red_zones.first().with_context(|| "No Red Zones?!")?.area()
        );

        Ok(())
    }
}
