use std::collections::{HashMap, HashSet};

use anyhow::{bail, Result};

use crate::data::Point2D;
use crate::day::Day;
use crate::input::get_input;
use crate::util::cardinal::{Cardinal, Turn};

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    Obstruction,
    Empty,
}

#[derive(Clone)]
struct GuardMap {
    tiles: HashMap<Point2D<usize>, Tile>,
    max: Point2D<usize>,
}

impl GuardMap {
    fn new() -> Self {
        GuardMap {
            tiles: HashMap::new(),
            max: Point2D::new(0, 0),
        }
    }

    fn insert(&mut self, pos: Point2D<usize>, tile: Tile) {
        self.tiles.insert(pos, tile);
        match (pos.x > self.max.x, pos.y > self.max.y) {
            (true, true) => self.max = pos,
            (true, false) => self.max = Point2D::new(pos.x, self.max.y),
            (false, true) => self.max = Point2D::new(self.max.x, pos.y),
            (false, false) => {}
        }
    }

    fn get(&self, pos: &Point2D<usize>) -> Tile {
        self.tiles.get(pos).map(|t| *t).unwrap_or(Tile::Empty)
    }
}

impl Point2D<usize> {
    fn step(&self, dir: Cardinal, mag: usize) -> Option<Self> {
        match dir {
            Cardinal::North => self.y.checked_sub(mag).map(|y| Point2D::new(self.x, y)),
            Cardinal::East => self.x.checked_add(mag).map(|x| Point2D::new(x, self.y)),
            Cardinal::South => self.y.checked_add(mag).map(|y| Point2D::new(self.x, y)),
            Cardinal::West => self.x.checked_sub(mag).map(|x| Point2D::new(x, self.y)),
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Guard {
    pos: Point2D<usize>,
    dir: Cardinal,
}

impl Guard {
    fn new() -> Self {
        Guard {
            pos: Point2D::new(0, 0),
            dir: Cardinal::North,
        }
    }

    fn step(&mut self, map: &GuardMap) -> bool {
        if let Some(next) = self
            .pos
            .step(self.dir, 1)
            .filter(|next| next.y <= map.max.y && next.x <= map.max.x)
        {
            match map.get(&next) {
                Tile::Empty => {
                    self.pos = next;
                }
                Tile::Obstruction => {
                    self.dir = self.dir.turn(Turn::Right);
                }
            }
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
enum PathTermination {
    Exited(HashSet<Guard>),
    Looped,
}

fn execute_pathing(guard_start: &Guard, map: &GuardMap) -> PathTermination {
    let mut guard = guard_start.clone();
    let mut visited: HashSet<Guard> = HashSet::new();
    visited.insert(guard);
    while guard.step(&map) {
        if visited.contains(&guard) {
            return PathTermination::Looped;
        } else {
            visited.insert(guard);
        }
    }
    PathTermination::Exited(visited)
}

fn unique_locations(visited: &HashSet<Guard>) -> usize {
    let mut unique: HashSet<Point2D<usize>> = HashSet::new();
    for v in visited {
        unique.insert(v.pos);
    }
    unique.len()
}

pub struct Day06;

impl Day for Day06 {
    fn main() -> Result<()> {
        let input = get_input(2024, 6)?;
        let mut guard: Guard = Guard::new();
        let mut map: GuardMap = GuardMap::new();
        // Prepare guard & map
        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let point = Point2D::new(x, y);
                match ch {
                    '#' => {
                        map.insert(point, Tile::Obstruction);
                    }
                    '.' => {
                        map.insert(point, Tile::Empty);
                    }
                    '^' => {
                        guard.pos = point;
                    }
                    _ => bail!("Unexpected char {ch}"),
                }
            }
        }
        if let PathTermination::Exited(visited) = execute_pathing(&guard, &map) {
            println!(
                "Unique guard visited positions: {}",
                unique_locations(&visited)
            );
            let mut block_attempted: HashSet<Point2D<usize>> = HashSet::new();
            let mut loop_count = 0;
            for v in visited {
                // If the next step is a valid empty tile then we should try filling it with an obstruction
                if let Some(next) = v.pos.step(v.dir, 1).filter(|n| {
                    n.y <= map.max.y
                        && n.x <= map.max.x
                        && map.get(n) == Tile::Empty
                        && !block_attempted.contains(n)
                }) {
                    block_attempted.insert(next);
                    let mut alt_map = map.clone();
                    alt_map.insert(next, Tile::Obstruction);
                    if execute_pathing(&guard, &alt_map) == PathTermination::Looped {
                        loop_count += 1;
                    }
                }
            }
            println!("Loop causing obstruction locations: {loop_count}");
        } else {
            bail!("Unexpected loop for first pathing.");
        }
        Ok(())
    }
}
