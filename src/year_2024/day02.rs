use crate::util::input::get_input;
use std::cmp::Ordering;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};

use crate::day::Day;

pub struct Day02;

fn is_safe_levels<'a, T: Iterator<Item = &'a u32>>(levels: T) -> bool {
    let mut level_iter = levels.peekable();
    let mut existing_ordering = Ordering::Equal;
    let mut safe = true;
    loop {
        match level_iter.next().zip(level_iter.peek()) {
            None => break,
            Some((left_num, right_num)) => {
                let diff = left_num.abs_diff(**right_num);
                if diff < 1 || diff > 3 {
                    safe = false;
                    break;
                }
                let ordering = left_num.cmp(right_num);
                if existing_ordering == Ordering::Equal {
                    existing_ordering = ordering;
                } else if existing_ordering != ordering {
                    safe = false;
                    break;
                }
            }
        }
    }
    safe
}

impl Day for Day02 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2024, 02)?);
        let mut safe_count = 0;
        let mut skip_safe_count = 0;
        for line in input.lines().map(|l| l.unwrap()) {
            let levels = line
                .split(" ")
                .map(|level_str| {
                    level_str
                        .parse::<u32>()
                        .with_context(|| "Could not parse integer")
                })
                .collect::<Result<Vec<u32>>>()?;
            // Check Basic Safety
            if is_safe_levels(levels.iter()) {
                safe_count += 1;
                skip_safe_count += 1;
                continue;
            }
            // Check Skip Level Safety
            for i in 0..levels.len() {
                if is_safe_levels(levels.iter().enumerate().filter_map(|(idx, level)| {
                    if idx == i {
                        None
                    } else {
                        Some(level)
                    }
                })) {
                    skip_safe_count += 1;
                    break;
                }
            }
        }
        println!("Safe report count: {safe_count}");
        println!("Skip safe report count: {skip_safe_count}");
        Ok(())
    }
}
