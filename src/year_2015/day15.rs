use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};
use lazy_regex::regex_captures;

use crate::day::Day;

pub struct Day15;

impl Day for Day15 {
    fn main() -> Result<()> {
        let input = BufReader::new(File::open("input/2015/day15.txt")?);
        for line in input.lines().map(|l| l.unwrap()) {
            println!("{line}");
            let (_, _name, fly_speed, fly_time_seconds, rest_time_seconds) = regex_captures!(
                "([A-Za-z]+) can fly ([0-9]+) km/s for ([0-9]+) seconds, but then must rest for ([0-9]+) seconds.",
                &line,
            ).with_context(|| "Could not match Reindeer speeds")?;
        }
        Ok(())
    }
}
