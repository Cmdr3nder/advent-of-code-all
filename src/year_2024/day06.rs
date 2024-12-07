use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{bail, Result};

use crate::data::Point2D;
use crate::day::Day;
use crate::util::cardinal::{Cardinal, Turn};

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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

pub struct Day06;

impl Day for Day06 {
    fn main() -> Result<()> {
        let input = BufReader::new(File::open("input/2024/day06.txt")?);
        let mut guard: Guard = Guard::new();
        let mut map: GuardMap = GuardMap::new();
        for (y, line) in input.lines().map(|l| l.unwrap()).enumerate() {
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
        let mut visited: HashSet<Point2D<usize>> = HashSet::new();
        visited.insert(guard.pos);
        while guard.step(&map) {
            visited.insert(guard.pos);
        }
        println!("Unique guard visited positions: {}", visited.len());
        Ok(())
    }
}
