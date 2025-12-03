use crate::util::input::get_input;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};

use crate::data::Point2D;
use crate::day::Day;

pub struct Day18;

const GRID_SIZE: usize = 100 * 100;
const STEP_COUNT: usize = 100;
const DIMENSIONS: (Point2D<isize>, Point2D<isize>) = (Point2D::new(0, 0), Point2D::new(99, 99));
const PEERS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

impl Day for Day18 {
    fn main() -> Result<()> {
        let corners: HashSet<Point2D<isize>> = [
            Point2D::new(0, 0),
            Point2D::new(99, 99),
            Point2D::new(0, 99),
            Point2D::new(99, 0),
        ]
        .iter()
        .copied()
        .collect();
        let input = BufReader::new(get_input(2015, 18)?);
        let mut grid: [bool; GRID_SIZE] = [false; GRID_SIZE];
        let mut stuck_grid: [bool; GRID_SIZE] = [false; GRID_SIZE];
        {
            let mut idx = 0;
            for line in input.lines().map(|l| l.unwrap()) {
                for ch in line.chars() {
                    let on = ch == '#';
                    grid[idx] = on;
                    stuck_grid[idx] = on;
                    idx += 1;
                }
            }
            for corner in &corners {
                stuck_grid[corner
                    .to_index(&DIMENSIONS)
                    .with_context(|| "Can't leave dimensions")?] = true;
            }
        }
        for _ in 0..STEP_COUNT {
            let mut grid_next: [bool; GRID_SIZE] = [false; GRID_SIZE];
            let mut stuck_grid_next: [bool; GRID_SIZE] = [false; GRID_SIZE];
            for point in DIMENSIONS.0.iter_to(&DIMENSIONS.1) {
                let mut peer_count = 0;
                let mut stuck_peer_count = 0;
                for peer_diff in PEERS {
                    if let Some(idx) = (point + peer_diff).to_index(&DIMENSIONS) {
                        if grid[idx] {
                            peer_count += 1;
                        }
                        if stuck_grid[idx] {
                            stuck_peer_count += 1;
                        }
                    }
                }
                let idx = point
                    .to_index(&DIMENSIONS)
                    .with_context(|| "We should not be able to exceed the dimensions")?;
                grid_next[idx] = if grid[idx] {
                    peer_count == 2 || peer_count == 3
                } else {
                    peer_count == 3
                };
                stuck_grid_next[idx] = if corners.contains(&point) {
                    true
                } else if stuck_grid[idx] {
                    stuck_peer_count == 2 || stuck_peer_count == 3
                } else {
                    stuck_peer_count == 3
                };
            }
            grid = grid_next;
            stuck_grid = stuck_grid_next;
        }
        let mut count = 0;
        let mut stuck_count = 0;
        for idx in 0..GRID_SIZE {
            if grid[idx] {
                count += 1;
            }
            if stuck_grid[idx] {
                stuck_count += 1;
            }
        }
        println!("Lit lights: {count}");
        println!("Lit lights, stuck: {stuck_count}");
        Ok(())
    }
}
