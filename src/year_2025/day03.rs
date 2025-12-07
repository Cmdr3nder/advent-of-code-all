use anyhow::{bail, Context, Result};
use std::io::{BufRead, BufReader};

use crate::day::Day;
use crate::util::input::get_input;

pub struct Day03;

fn char_u32(ch: char) -> Result<u32> {
    Ok(match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => bail!("Unexpected char when parsing digit '{ch}'"),
    })
}

fn max_joltage(battery_bank: &str) -> Result<u32> {
    let mut batteries = battery_bank.chars().enumerate();
    let (_, mut tens_joltage) = batteries
        .next()
        .with_context(|| "Too few batteries in battery bank")
        .and_then(|(i, ch)| Ok((i, char_u32(ch)?)))?;
    let (_, mut ones_joltage) = batteries
        .next()
        .with_context(|| "Too few batteries in battery bank")
        .and_then(|(i, ch)| Ok((i, char_u32(ch)?)))?;
    for (_, ch) in batteries {
        let battery_joltage = char_u32(ch)?;
        let current_joltage = (10 * tens_joltage) + ones_joltage;
        let with_tens_joltage = (10 * tens_joltage) + battery_joltage;
        let with_ones_joltage = (10 * ones_joltage) + battery_joltage;
        if current_joltage < with_ones_joltage {
            tens_joltage = ones_joltage;
            ones_joltage = battery_joltage;
        } else if current_joltage < with_tens_joltage {
            ones_joltage = battery_joltage;
        }
    }
    let joltage = (10 * tens_joltage) + ones_joltage;
    Ok(joltage)
}

impl Day for Day03 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2025, 3)?);

        let mut total_joltage = 0;

        for battery_bank in input.lines().map(|l| l.unwrap()) {
            let joltage = max_joltage(&battery_bank)?;
            total_joltage += joltage;
        }

        println!("Total output joltage of the battery banks: {total_joltage}");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_joltage() {
        let data: Vec<(&'static str, u32)> = vec![
            ("987654321111111", 98),
            ("811111111111119", 89),
            ("234234234234278", 78),
            ("818181911112111", 92),
        ];
        for (battery_bank, expected_joltage) in data {
            match max_joltage(battery_bank) {
                Ok(joltage) => assert_eq!(joltage, expected_joltage, "Expected battery bank '{battery_bank}' to produce a max joltage of {expected_joltage}"),
                Err(_) => panic!("Did not expect max_joltage to throw when given '{battery_bank}"),
            }
        }
    }
}
