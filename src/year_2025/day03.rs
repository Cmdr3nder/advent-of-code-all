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

// fn max_joltage(battery_bank: &str) -> Result<u64> {
//     let mut batteries = battery_bank.chars().enumerate();
//     let (_, mut tens_joltage) = batteries
//         .next()
//         .with_context(|| "Too few batteries in battery bank")
//         .and_then(|(i, ch)| Ok((i, char_u64(ch)?)))?;
//     let (_, mut ones_joltage) = batteries
//         .next()
//         .with_context(|| "Too few batteries in battery bank")
//         .and_then(|(i, ch)| Ok((i, char_u64(ch)?)))?;
//     for (_, ch) in batteries {
//         let battery_joltage = char_u64(ch)?;
//         let current_joltage = (10 * tens_joltage) + ones_joltage;
//         let with_tens_joltage = (10 * tens_joltage) + battery_joltage;
//         let with_ones_joltage = (10 * ones_joltage) + battery_joltage;
//         if current_joltage < with_ones_joltage {
//             tens_joltage = ones_joltage;
//             ones_joltage = battery_joltage;
//         } else if current_joltage < with_tens_joltage {
//             ones_joltage = battery_joltage;
//         }
//     }
//     let joltage = (10 * tens_joltage) + ones_joltage;
//     Ok(joltage)
// }

fn max_joltage(battery_bank: &[u8], battery_count: usize) -> u64 {
    let mut battery_state: Vec<bool> = battery_bank.iter().map(|_| true).collect();
    let mut turn_off_count = battery_bank.len() - battery_count;
    let mut least_wanted: u8 = 0;
    while turn_off_count > 0 && least_wanted <= 9 {
        for i in 0..battery_state.len() {
            if turn_off_count > 0 && battery_state[i] && battery_bank[i] == least_wanted {
                battery_state[i] = false;
                turn_off_count -= 1;
            }
        }
        least_wanted += 1;
    }
    let mut joltage: u64 = 0;
    for i in 0..battery_state.len() {
        if battery_state[i] {
            joltage *= 10;
            joltage += battery_bank[i] as u64;
        }
    }
    joltage
}

fn parse_battery_bank(battery_bank: &str) -> Result<Vec<u8>> {
    battery_bank.chars().map(|ch| char_u8(ch)).collect()
}

impl Day for Day03 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2025, 3)?);

        let mut total_joltage = 0;

        for battery_bank_raw in input.lines().map(|l| l.unwrap()) {
            let battery_bank = parse_battery_bank(&battery_bank_raw)?;
            let joltage = max_joltage(&battery_bank, 2);
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
