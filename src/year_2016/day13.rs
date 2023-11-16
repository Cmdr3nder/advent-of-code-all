use std::fs;

use anyhow::Result;

use crate::data::Point2D;
use crate::day::Day;

fn is_wall(point: Point2D<usize>, favorite_number: usize) -> bool {
    let x = point.x;
    let y = point.y;
    let location_number = (x * x) + (3 * x) + (2 * x * y) + y + (y * y) + favorite_number;
    let bits = location_number.count_ones();
    bits % 2 != 0
}

pub struct Day13;

impl Day for Day13 {
    fn main() -> Result<()> {
        let input = fs::read_to_string("input/2016/day13.txt")?;
        let input = input.trim();
        let favorite_number: usize = input.parse()?;
        // Work up to A* using the red star games site and our imported priority queue?
        Ok(())
    }
}
