use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{bail, Context, Result};
use lazy_regex::regex_captures;

use crate::day::Day;

fn find_drop_time(discs: &[(usize, usize)]) -> Option<usize> {
    for time in 0..=usize::MAX {
        let mut pass = true;
        for (c, (pos, init)) in discs.iter().enumerate() {
            if (c + 1 + init + time) % pos != 0 {
                pass = false;
                break;
            }
        }
        if pass {
            return Some(time);
        }
    }
    None
}

pub struct Day15;

impl Day for Day15 {
    fn main() -> Result<()> {
        let input = BufReader::new(File::open("input/2016/day15.txt")?);
        let mut discs: Vec<(usize, usize)> = Vec::new();
        for line in input.lines().map(|l| l.unwrap()) {
            if let Some((_, positions, initial_position)) = regex_captures!(
                "Disc #[0-9]+ has ([0-9]+) positions; at time=0, it is at position ([0-9]+)",
                &line
            ) {
                let positions: usize = positions.parse()?;
                let initial_position: usize = initial_position.parse()?;
                discs.push((positions, initial_position));
            } else {
                bail!("Malformed input");
            }
        }
        let first_opportunity =
            find_drop_time(&discs).with_context(|| "Should be able to find a good drop time!")?;
        println!("First time you can press the button to get a capsule is {first_opportunity}");
        discs.push((11, 0));
        let first_opportunity =
            find_drop_time(&discs).with_context(|| "Should be able to find a good drop time!")?;
        println!("First time you can press the button to get a capsule is {first_opportunity}, with additional disc");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_day_2016_15() {
        let op = find_drop_time(&vec![(5, 4), (2, 1)]).unwrap();
        assert_eq!(5, op);
    }
}
