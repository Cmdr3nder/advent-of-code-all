use std::fs;

use anyhow::{bail, Context, Result};
use lazy_regex::regex;

use crate::day::Day;

pub struct Day03;

impl Day for Day03 {
    fn main() -> Result<()> {
        let input = fs::read_to_string("input/2024/day03.txt")?;
        let re = regex!(r"((mul)\(([0-9]{1,3}),([0-9]{1,3})\))|((do|don't)\(\))");
        let mut total_mult_sum = 0;
        let mut total_cond_mult_sum = 0;
        let mut cond = true;
        for match_group in re.captures_iter(&input) {
            match match_group
                .get(2)
                .or(match_group.get(6))
                .map(|m| m.as_str())
                .with_context(|| "A match group label was not in expected form.")?
            {
                "mul" => {
                    let (match_a, match_b) = match_group
                        .get(3)
                        .zip(match_group.get(4))
                        .with_context(|| "A mul match group was not in expected form.")?;
                    let num_a: u32 = match_a.as_str().parse()?;
                    let num_b: u32 = match_b.as_str().parse()?;
                    let mult = num_a * num_b;
                    total_mult_sum += mult;
                    if cond {
                        total_cond_mult_sum += mult;
                    }
                }
                "do" => {
                    cond = true;
                }
                "don't" => {
                    cond = false;
                }
                _ => bail!("Unexpected command label"),
            }
        }
        println!("Total mult command sum: {total_mult_sum}");
        println!("Total conditional mult command sum: {total_cond_mult_sum}");
        Ok(())
    }
}
