use std::collections::HashSet;
use std::fs;

use anyhow::{bail, Result};

use crate::data::Point2D;
use crate::day::Day;

pub struct Day01;

#[derive(Copy, Clone)]
enum Turn {
    Right,
    Left,
}

#[derive(Copy, Clone)]
enum Cardinal {
    North,
    East,
    South,
    West,
}

impl Cardinal {
    fn turn(self, turn: Turn) -> Self {
        match (self, turn) {
            (Cardinal::North, Turn::Right) => Cardinal::East,
            (Cardinal::North, Turn::Left) => Cardinal::West,
            (Cardinal::East, Turn::Right) => Cardinal::South,
            (Cardinal::East, Turn::Left) => Cardinal::North,
            (Cardinal::South, Turn::Right) => Cardinal::West,
            (Cardinal::South, Turn::Left) => Cardinal::East,
            (Cardinal::West, Turn::Right) => Cardinal::North,
            (Cardinal::West, Turn::Left) => Cardinal::South,
        }
    }

    fn forward(self, raw_magnitude: u16) -> (i32, i32) {
        let magnitude: i32 = raw_magnitude.into();
        match self {
            Cardinal::North => (0, magnitude),
            Cardinal::East => (magnitude, 0),
            Cardinal::South => (0, -magnitude),
            Cardinal::West => (-magnitude, 0),
        }
    }
}

impl Day for Day01 {
    fn main() -> Result<()> {
        let input = fs::read_to_string("input/2016/day01.txt")?;
        let mut instructions: Vec<(Turn, u16)> = Vec::new();
        for raw in input.split(',') {
            let clean = raw.trim();
            if clean.is_empty() {
                continue;
            }
            let mut chars = clean.chars();
            let turn = match chars.next() {
                Some('R') => Turn::Right,
                Some('L') => Turn::Left,
                Some(x) => bail!("Unexpected char {x}"),
                None => bail!("Expected a char"),
            };
            let magnitude: u16 = chars.collect::<String>().parse()?;
            instructions.push((turn, magnitude));
        }
        // Find final position
        let mut position: Point2D<i32> = Point2D::new(0, 0);
        let mut facing = Cardinal::North;
        for (turn, magnitude) in &instructions {
            facing = facing.turn(*turn);
            position += facing.forward(*magnitude);
        }
        println!(
            "Final position is {position:?}, taxicab distance from (0, 0) is {}",
            position.x.abs() + position.y.abs()
        );

        // Find first duplicated position
        let mut position: Point2D<i32> = Point2D::new(0, 0);
        let mut facing = Cardinal::North;
        let mut positions: HashSet<Point2D<i32>> = HashSet::new();
        positions.insert(position);
        'duplicate_find: for (turn, magnitude) in &instructions {
            facing = facing.turn(*turn);
            for _ in 0..*magnitude {
                position += facing.forward(1);
                if positions.contains(&position) {
                    println!(
                        "Duplicate position is {position:?}, taxicab distance from (0, 0) is {}",
                        position.x.abs() + position.y.abs()
                    );
                    break 'duplicate_find;
                } else {
                    positions.insert(position);
                }
            }
        }
        Ok(())
    }
}
