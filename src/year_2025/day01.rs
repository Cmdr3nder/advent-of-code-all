use std::io::{BufRead, BufReader};

use anyhow::{bail, Context, Result};
use lazy_regex::regex_captures;

use crate::day::Day;
use crate::util::input::get_input;

pub struct Day01;

impl Day for Day01 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2025, 01)?);
        let mut count: u32 = 0;
        let mut count_new: i32 = 0;
        let mut dial: i32 = 50;

        for line in input.lines().map(|l| l.unwrap()) {
            let (_, direction_str, magnitude_str) = regex_captures!("(L|R)([0-9]+)", &line)
                .with_context(|| format!("Failed to match line regex {line}"))?;
            let magnitude = magnitude_str.parse::<i32>()?;
            match direction_str {
                "L" => {
                    dial -= magnitude;
                }
                "R" => {
                    dial += magnitude;
                }
                _ => bail!("Unexpected direction_str"),
            }
            count_new += (dial / 100).abs();
            dial = dial % 100;
            if dial < 0 {
                dial += 100;
                count_new += 1;
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
