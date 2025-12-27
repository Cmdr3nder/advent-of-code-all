use anyhow::{bail, Context, Result};
use lazy_regex::regex_captures;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::u64;

use crate::day::Day;
use crate::util::input::get_input;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Area {
    area: u64,
    low: Point,
    high: Point,
    a: Point,
    b: Point,
}

impl Point {
    pub fn rect_area(&self, other: &Self) -> Area {
        Area {
            area: (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1),
            low: Point {
                x: if self.x < other.x { self.x } else { other.x },
                y: if self.y < other.y { self.y } else { other.y },
            },
            high: Point {
                x: if self.x > other.x { self.x } else { other.x },
                y: if self.y > other.y { self.y } else { other.y },
            },
            a: *self,
            b: *other,
        }
    }
}

enum PenState {
    FindEdge,      // Pen up
    FoundEdge,     // Pen up, but TBD
    TraceEdge,     // Pen up, wait for next gap to FindEdge
    TraceContents, // Pen down, wait for next edge to FoundEdge
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
                // Finished
                self.finished = true;
                Ok(())
            } else {
                bail!("Polygon has no points, cannot finish yet");
            }
        }
    }

    fn safe(&self, area: &Area) -> Result<bool> {
        if !self.finished {
            bail!("Polygon must be finished before doing safe checks");
        }
        for y in area.low.y..=area.high.y {
            for x in self.x_min..=area.high.x {
                let p = Point { x, y };
                if !self.outline.contains(&p) {
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
        let mut areas: Vec<Area> = Vec::new();
        let mut polygon = Polygon::new();
        for (line_num, line) in input.lines().map(|l| l.unwrap()).enumerate() {
            let (_, x_str, y_str) = regex_captures!(r"([0-9]+),([0-9]+)", &line)
                .with_context(|| format!("No point on line {line_num}"))?;
            let new_tile = Point {
                x: x_str.parse()?,
                y: y_str.parse()?,
            };
            for tile in &red_tiles {
                areas.push(new_tile.rect_area(tile));
            }
            polygon.insert(new_tile)?;
            red_tiles.push(new_tile);
        }

        areas.sort_unstable();
        areas.reverse();

        println!("Largest area based on red tiles, {}", areas[0].area,);

        polygon.finish()?;

        for area in areas {
            if polygon.safe(&area)? {
                println!("Largest christmas area, {}", area.area);
                break;
            }
        }

        Ok(())
    }
}
