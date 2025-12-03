use crate::util::input::get_input;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use anyhow::{Context, Error, Result};
use lazy_regex::regex_captures;

use crate::day::Day;

pub struct Day02;

#[derive(Clone, Copy, Debug)]
struct Present(u32, u32, u32);

impl FromStr for Present {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (_, l, w, h) = regex_captures!("([0-9]+)x([0-9]+)x([0-9]+)", s)
            .with_context(|| format!("Unexpected '{s}', expected LxWxH"))?;
        Ok(Present(
            l.parse()
                .with_context(|| format!("Failed to parse '{l}' in '{s}'"))?,
            w.parse()
                .with_context(|| format!("Failed to parse '{l}' in '{s}'"))?,
            h.parse()
                .with_context(|| format!("Failed to parse '{l}' in '{s}'"))?,
        ))
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
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2015, 02)?);
        let mut wrapping_paper: u32 = 0;
        let mut ribbon: u32 = 0;
        for line in input.lines().map(|l| l.unwrap()) {
            if line.is_empty() {
                continue;
            }
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
                * ((present.0 * present.1) + (present.1 * present.2) + (present.2 * present.0)))
                + smallest_side_area;
            ribbon += (present.0 * present.1 * present.2) + smallest_side_perim;
        }
        println!("Total Wrapping Paper SqFt {wrapping_paper}");
        println!("Feet of Ribbon {ribbon}");
        Ok(())
    }
}
