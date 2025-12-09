use anyhow::Result;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};

use crate::data::Point2D;
use crate::day::Day;
use crate::util::input::get_input;

pub struct Day04;

type Point = Point2D<usize>;
type Stacks = HashSet<Point>;
type Removable = Vec<Point>;
type Extents = (Point, Point);

fn is_removable(stacks: &Stacks, location: &Point) -> bool {
    let low_y = if location.y > 0 { location.y - 1 } else { 0 };
    let high_y = location.y + 1;
    let low_x = if location.x > 0 { location.x - 1 } else { 0 };
    let high_x = location.x + 1;
    let mut neighbor_count = 0;
    'removable_check: for y in low_y..=high_y {
        for x in low_x..=high_x {
            let neighbor = &Point::new(x, y);
            if neighbor != location && stacks.contains(neighbor) {
                neighbor_count += 1;
                if neighbor_count >= 4 {
                    break 'removable_check;
                }
            }
        }
    }
    return neighbor_count < 4;
}

fn find_removable(stacks: &Stacks, extents: &Extents) -> Removable {
    let mut removable = Removable::new();
    for y in extents.0.y..=extents.1.y {
        for x in extents.0.x..=extents.1.x {
            let location = Point2D::new(x, y);
            if stacks.contains(&location) && is_removable(stacks, &location) {
                removable.push(location);
            }
        }
    }
    removable
}

impl Day for Day04 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2025, 4)?);

        let mut stacks: Stacks = Stacks::new();
        let mut extents: Extents = (Point::new(0, 0), Point::new(0, 0));
        for (y, row) in input.lines().map(|l| l.unwrap()).enumerate() {
            if y > extents.1.y {
                extents.1.y = y;
            }
            for (x, ch) in row.chars().enumerate() {
                if x > extents.1.x {
                    extents.1.x = x;
                }
                if ch == '@' {
                    stacks.insert(Point::new(x, y));
                }
            }
        }
        // Proccess first pass
        let mut removable = find_removable(&stacks, &extents);
        let available_count = removable.len();
        let mut total_removed = 0;
        while removable.len() > 0 {
            total_removed += removable.len();
            // Apply removal
            for r in removable {
                stacks.remove(&r);
            }
            removable = find_removable(&stacks, &extents);
        }

        println!("Available rolls to move: {available_count}");
        println!("Removed rolls: {total_removed}");

        Ok(())
    }
}
