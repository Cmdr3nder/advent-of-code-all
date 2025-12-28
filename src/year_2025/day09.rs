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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Range {
    low: u64,
    high: u64,
}

impl Range {
    fn new(a: u64, b: u64) -> Self {
        if a < b {
            Range { low: a, high: b }
        } else {
            Range { low: b, high: a }
        }
    }

    fn size(&self) -> u64 {
        (self.high - self.low) + 1
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.low.max(other.low) <= self.high.min(other.high)
    }

    fn inner(&self) -> Option<Self> {
        if self.size() < 3 {
            // Squished out of existence
            None
        } else {
            Some(Range {
                low: self.low + 1,
                high: self.high - 1,
            })
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, Hash)]
struct Rectangle {
    x: Range,
    y: Range,
}

impl Rectangle {
    fn new(a: &Point, b: &Point) -> Self {
        Rectangle {
            x: Range::new(a.x, b.x),
            y: Range::new(a.y, b.y),
        }
    }

    fn area(&self) -> u64 {
        self.x.size() * self.y.size()
    }

    fn overlaps(&self, other: &Rectangle) -> bool {
        self.x.overlaps(&other.x) && self.y.overlaps(&other.y)
    }

    fn inner(&self) -> Option<Self> {
        self.x
            .inner()
            .zip(self.y.inner())
            .map(|(x, y)| Rectangle { x, y })
    }
}

impl PartialOrd for Rectangle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.area()
                .cmp(&other.area())
                .then(self.x.cmp(&other.x))
                .then(self.y.cmp(&other.y)),
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
                red_zones.push(Rectangle::new(&new_tile, tile));
            }
            // Draw outline
            if let Some(prev) = red_tiles.last() {
                outline.push(Rectangle::new(prev, &new_tile));
            }
            red_tiles.push(new_tile);
        }

        // Complete outline
        if let Some((first, last)) = red_tiles.first().zip(red_tiles.last()) {
            outline.push(Rectangle::new(last, first));
        }

        // Sort red cornered rectangles
        red_zones.sort_unstable();
        red_zones.reverse();

        println!(
            "Largest area based on red tiles, {}",
            red_zones.first().with_context(|| "No Red Zones?!")?.area()
        );

        for red_zone in &red_zones {
            if let Some(inner) = red_zone.inner() {
                if !outline.iter().any(|line| inner.overlaps(line)) {
                    println!("Christmas area, {}", red_zone.area());
                    break;
                }
            } else {
                println!("Christmas area, {}", red_zone.area());
                break;
            }
        }

        Ok(())
    }
}
