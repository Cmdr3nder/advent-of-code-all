use std::fs;

use anyhow::{Context, Result};
use lazy_regex::regex;

use crate::day::Day;

pub struct Day03;

impl Day for Day03 {
    fn main() -> Result<()> {
        let input = fs::read_to_string("input/2024/day03.txt")?;
        let re = regex!(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)");
        let mut total_mult_sum = 0;
        for match_group in re.captures_iter(&input) {
            let (match_a, match_b) = match_group.get(1).zip(match_group.get(2)).with_context(|| "A match group was not in expected form.")?;
            let num_a: u32 = match_a.as_str().parse()?;
            let num_b: u32 = match_b.as_str().parse()?;
            total_mult_sum += num_a * num_b;
        }
        println!("Total mult command sum: {total_mult_sum}");
        Ok(())
    }
}
