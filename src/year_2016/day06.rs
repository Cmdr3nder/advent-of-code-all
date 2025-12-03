use crate::util::input::get_input;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

use anyhow::Result;

use crate::day::Day;

pub struct Day06;

impl Day for Day06 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2016, 06)?);
        let mut char_counts: HashMap<(char, usize), u32> = HashMap::new();
        let mut max_pos = 0;
        for line in input.lines().map(|l| l.unwrap()) {
            for (i, ch) in line.chars().enumerate() {
                char_counts
                    .entry((ch, i))
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                if i > max_pos {
                    max_pos = i;
                }
            }
        }
        let mut most_common: Vec<(char, u32)> = vec![('a', 0); max_pos + 1];
        let mut least_common: Vec<(char, u32)> = vec![('a', u32::MAX); max_pos + 1];
        for ((ch, pos), count) in char_counts {
            let (_, prev_count) = most_common[pos];
            if count > prev_count {
                most_common[pos] = (ch, count);
            }
            let (_, prev_count) = least_common[pos];
            if count < prev_count {
                least_common[pos] = (ch, count);
            }
        }
        let corrected: String = most_common.iter().map(|(ch, _)| ch).collect();
        println!("Error corrected message (most): {corrected}");
        let corrected: String = least_common.iter().map(|(ch, _)| ch).collect();
        println!("Error corrected message (least): {corrected}");
        Ok(())
    }
}
