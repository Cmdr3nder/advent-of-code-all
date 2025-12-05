use std::io::{BufRead, BufReader};

use anyhow::{bail, Context, Result};
use lazy_regex::regex_captures;

use crate::day::Day;
use crate::util::input::get_input;

pub struct Day01;

impl Day for Day01 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2025, 1)?);
        let mut count: u32 = 0;
        let mut count_new: i32 = 0;
        let mut dial: i32 = 50;

        for line in input.lines().map(|l| l.unwrap()) {
            let (_, direction_str, magnitude_str) = regex_captures!("(L|R)([0-9]+)", &line)
                .with_context(|| format!("Failed to match line regex {line}"))?;
            let mut magnitude = magnitude_str.parse::<i32>()?;
            let delta = match direction_str {
                "L" => -1,
                "R" => 1,
                _ => bail!("Unexpected direction_str"),
            };
            while magnitude > 0 {
                dial += delta;
                if dial == 0 {
                    count_new += 1;
                } else if dial == 100 {
                    count_new += 1;
                    dial = 0;
                } else if dial == -1 {
                    dial = 99;
                }
                magnitude -= 1;
            }
            if dial == 0 {
                count += 1;
            }
        }

        println!("Dial at 0 count: {count}");
        println!("Dial passes 0 count: {count_new}");

        Ok(())
    }
}
