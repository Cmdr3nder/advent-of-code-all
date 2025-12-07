use anyhow::{bail, Result};
use std::io::{BufRead, BufReader};

use crate::day::Day;
use crate::util::input::get_input;

pub struct Day03;

fn char_u8(ch: char) -> Result<u8> {
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

fn max_joltage(battery_bank: &[u8], battery_count: usize) -> u64 {
    let mut battery_bank: Vec<u8> = battery_bank.iter().map(|n| *n).collect();
    while battery_bank.len() > battery_count {
        for i in 0..battery_bank.len() {
            if i == battery_bank.len() - 1 || battery_bank[i] < battery_bank[i + 1] {
                battery_bank.remove(i);
                break;
            }
        }
    }
    let mut joltage: u64 = 0;
    for i in 0..battery_bank.len() {
        joltage *= 10;
        joltage += battery_bank[i] as u64;
    }
    joltage
}

fn parse_battery_bank(battery_bank: &str) -> Result<Vec<u8>> {
    battery_bank.chars().map(|ch| char_u8(ch)).collect()
}

impl Day for Day03 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2025, 3)?);

        let mut total_joltage_2 = 0;
        let mut total_joltage_12 = 0;

        for battery_bank_raw in input.lines().map(|l| l.unwrap()) {
            let battery_bank = parse_battery_bank(&battery_bank_raw)?;
            total_joltage_2 += max_joltage(&battery_bank, 2);
            total_joltage_12 += max_joltage(&battery_bank, 12);
        }

        println!("Total output joltage of the battery banks (2): {total_joltage_2}");
        println!("Total output joltage of the battery banks (12): {total_joltage_12}");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_joltage() {
        let data: Vec<(&'static str, Vec<(usize, u64)>)> = vec![
            ("987654321111111", vec![(2, 98), (12, 987654321111)]),
            ("811111111111119", vec![(2, 89), (12, 811111111119)]),
            ("234234234234278", vec![(2, 78), (12, 434234234278)]),
            ("818181911112111", vec![(2, 92), (12, 888911112111)]),
        ];
        for (battery_bank_raw, tests) in data {
            let battery_bank = parse_battery_bank(battery_bank_raw).unwrap();
            for (battery_count, expected_joltage) in tests {
                let joltage = max_joltage(&battery_bank, battery_count);
                assert_eq!(
                    joltage, expected_joltage,
                    "Given battery bank '{battery_bank_raw}' ({battery_count})"
                );
            }
        }
    }
}
