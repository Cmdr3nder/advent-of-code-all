use anyhow::{bail, Context, Result};
use lazy_regex::regex_captures;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::u64;

use crate::day::Day;
use crate::util::input::get_input;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    pub fn rect_area(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

struct Polygon {
    first_point: Option<Point>,
    last_point: Option<Point>,
    outline: HashSet<Point>,
    finished: bool,
    y_min: u64,
    y_max: u64,
    x_min: u64,
    x_max: u64,
}

impl Polygon {
    fn new() -> Self {
        Polygon {
            first_point: None,
            last_point: None,
            outline: HashSet::new(),
            finished: false,
            y_min: u64::MAX,
            y_max: 0,
            x_min: u64::MAX,
            x_max: 0,
        }
    }

    fn insert(&mut self, new_tile: Point) -> Result<()> {
        if self.finished {
            bail!("Polygon already finished, cannot insert {new_tile:?}");
        }
        if self.first_point == None {
            self.first_point = Some(new_tile);
        }
        // Draw Outline
        if let Some(prev) = self.last_point {
            match (prev.x.cmp(&new_tile.x), prev.y.cmp(&new_tile.y)) {
                (Ordering::Less | Ordering::Greater, Ordering::Equal) => {
                    // Horizontal
                    for x in new_tile.x..prev.x {
                        self.outline.insert(Point { x, y: prev.y });
                    }
                }
                (Ordering::Equal, Ordering::Less | Ordering::Greater) => {
                    // Vertical
                    for y in new_tile.y..prev.y {
                        self.outline.insert(Point { x: prev.x, y });
                    }
                }
                _ => bail!("Unexpected point pair for outlining, ({prev:?}, {new_tile:?})"),
            }
        } else {
            self.outline.insert(new_tile);
        }
        // Find Extents
        if new_tile.x < self.x_min {
            self.x_min = new_tile.x;
        } else if new_tile.x > self.x_max {
            self.x_max = new_tile.x;
        }
        if new_tile.y < self.y_min {
            self.y_min = new_tile.y;
        } else if new_tile.y > self.y_max {
            self.y_max = new_tile.y;
        }
        // Prep for next insert
        self.last_point = Some(new_tile);
        Ok(())
    }

    fn finish(&mut self) -> Result<()> {
        if self.finished {
            bail!("Polygon already finished, cannot finish twice");
        } else {
            if let Some(first_point) = self.first_point {
                // Finish outline
                self.insert(first_point)?;
                // Do infill
                let mut flood_zone: Vec<Point> = Vec::new();
                flood_zone.push(Point {
                    x: (self.x_min + self.x_max) / 2,
                    y: (self.y_min + self.y_max) / 2,
                });
                println!(
                    "Extents ({}, {}) -> ({}, {})",
                    self.x_min, self.y_min, self.x_max, self.y_max
                );
                while let Some(p) = flood_zone.pop() {
                    if !self.outline.contains(&p) {
                        self.outline.insert(p);
                        flood_zone.push(Point { x: p.x, y: p.y + 1 });
                        flood_zone.push(Point { x: p.x, y: p.y - 1 });
                        flood_zone.push(Point { x: p.x + 1, y: p.y });
                        flood_zone.push(Point { x: p.x - 1, y: p.y });
                    }
                    if p.x < self.x_min || p.x > self.x_max || p.y < self.y_min || p.y > self.y_max
                    {
                        bail!("Oops, picked a bad flood start point and reached outside the expected polygon reaches");
                    }
                }
                // Finished
                self.finished = true;
                Ok(())
            } else {
                bail!("Polygon has no points, cannot finish yet");
            }
        }
    }

    fn safe_area(&self, a: &Point, b: &Point) -> Result<bool> {
        if !self.finished {
            bail!("Polygon must be finished before doing safe_area checks");
        }
        for x in a.x..=b.x {
            for y in a.y..=b.y {
                if !self.outline.contains(&Point { x, y }) {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }
}

pub struct Day09;

impl Day for Day09 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2025, 9)?);

        // Parse Points & Measure Basic Areas
        let mut red_tiles: Vec<Point> = Vec::new();
        let mut polygon: Polygon = Polygon::new();
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
            polygon.insert(new_tile)?;
            red_tiles.push(new_tile);
        }

        println!("Largest area based on red tiles, {largest_area}");

        polygon.finish()?;
        largest_area = 0;
        for i in 0..red_tiles.len() {
            for j in i..red_tiles.len() {
                let a = &red_tiles[i];
                let b = &red_tiles[j];
                let area = a.rect_area(b);
                if area > largest_area && polygon.safe_area(a, b)? {
                    largest_area = area;
                }
            }
        }

        println!("Largest christmas area, {largest_area}");

        Ok(())
    }
}
