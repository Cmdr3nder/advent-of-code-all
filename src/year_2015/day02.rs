use std::error::Error;
use std::fs;
use std::str::FromStr;

use anyhow::{Context, Result};

use crate::day::Day;

pub struct Day02;

#[derive(Clone, Copy, Debug)]
struct Present(u32, u32, u32);

impl FromStr for Present {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut parts = s.split('x');
        let mut present = Present(0, 0, 0);
        if let Some(part) = parts.next() {
            present.0 = part
                .parse()
                .with_context(|| format!("Failed to parse part 1 '{part}' in '{s}'"))?;
        }
        if let Some(part) = parts.next() {
            present.1 = part
                .parse()
                .with_context(|| format!("Failed to parse part 2 '{part}' in '{s}'"))?;
        }
        if let Some(part) = parts.next() {
            present.2 = part
                .parse()
                .with_context(|| format!("Failed to parse part 3 '{part}' in '{s}'"))?;
        }
        // TODO: chuck an error if additional parts exist
        Ok(present)
    }
}

fn min(a: u32, b: u32, c: u32) -> u32 {
    if a <= b && a <= c {
        a
    } else if b <= a && b <= c {
        b
    } else {
        c
    }
}

impl Day for Day02 {
    fn main() -> Result<(), Box<dyn Error>> {
        let input = fs::read_to_string("input/2015/day02.txt")?;
        let mut wrapping_paper: u32 = 0;
        let mut ribbon: u32 = 0;
        for line in input.split('\n') {
            if line.len() > 0 {
                let present: Present = line.parse()?;
                let smallest_side_area = min(
                    present.0 * present.1,
                    present.1 * present.2,
                    present.2 * present.0,
                );
                let smallest_side_perim = 2 * min(
                    present.0 + present.1,
                    present.1 + present.2,
                    present.2 + present.0,
                );
                wrapping_paper += (2
                    * ((present.0 * present.1)
                        + (present.1 * present.2)
                        + (present.2 * present.0)))
                    + smallest_side_area;
                ribbon += (present.0 * present.1 * present.2) + smallest_side_perim;
            }
        }
        println!("Total Wrapping Paper SqFt {wrapping_paper}");
        println!("Feet of Ribbon {ribbon}");
        Ok(())
    }
}
