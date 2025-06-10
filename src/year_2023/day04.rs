use std::collections::HashSet;

use anyhow::{Context, Result};

use crate::day::Day;
use crate::input::get_input;
use crate::util::expand::expand_with;

fn score_for_count(count: usize) -> u32 {
    match count {
        0 => 0,
        1 => 1,
        _ => {
            let mut score = 1;
            for _ in 1..count {
                score *= 2;
            }
            score
        }
    }
}

pub struct Day04;

impl Day for Day04 {
    fn main() -> Result<()> {
        let input_str = get_input(2023, 4)?;
        let mut card_copies = vec![1];
        let mut points = 0;
        let mut copies = 0;
        for (line_number, line) in input_str.lines().enumerate() {
            let mut split = line
                .split(": ")
                .last()
                .with_context(|| "Expected header and body split on ': '")?
                .split(" | ");
            let mut winning: HashSet<u32> = HashSet::new();
            for n in split
                .next()
                .with_context(|| "Expected winning numbers")?
                .split(" ")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
            {
                winning.insert(n.parse()?);
            }
            let mut actuals: HashSet<u32> = HashSet::new();
            for n in split
                .next()
                .with_context(|| "Expected actual numbers")?
                .split(" ")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
            {
                actuals.insert(n.parse()?);
            }
            let won = winning.intersection(&actuals).count();
            points += score_for_count(won);
            expand_with(&mut card_copies, line_number + won + 1, 1);
            let copy_count = card_copies[line_number];
            copies += copy_count;
            for x in 1..=won {
                card_copies[line_number + x] += copy_count;
            }
        }
        println!("Scratchits worth: {points}");
        println!("Total copies: {copies}");
        Ok(())
    }
}
