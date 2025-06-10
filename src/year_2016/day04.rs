use std::collections::HashMap;

use anyhow::Result;
use lazy_regex::{regex_captures, regex_is_match};

use crate::day::Day;
use crate::input::get_input;

pub struct Day04;

fn calculate_checksum(str: &str) -> String {
    let mut char_counts: HashMap<char, u32> = HashMap::new();

    for ch in str.chars() {
        if ch != '-' {
            char_counts
                .entry(ch)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    let mut char_counts: Vec<(char, u32)> = char_counts
        .iter()
        .map(|(ch, count)| (*ch, *count))
        .collect();
    // Sort count descending, char ascending
    char_counts.sort_by(|(a_ch, a_count), (b_ch, b_count)| (b_count, a_ch).cmp(&(a_count, b_ch)));
    char_counts.iter().take(5).map(|(ch, _)| ch).collect()
}

fn decode(str: &str, sector: u32) -> String {
    str.chars()
        .map(|ch| {
            let mut ch = ch;
            for _ in 0..sector {
                ch = match ch {
                    '-' => ' ',
                    'a' => 'b',
                    'b' => 'c',
                    'c' => 'd',
                    'd' => 'e',
                    'e' => 'f',
                    'f' => 'g',
                    'g' => 'h',
                    'h' => 'i',
                    'i' => 'j',
                    'j' => 'k',
                    'k' => 'l',
                    'l' => 'm',
                    'm' => 'n',
                    'n' => 'o',
                    'o' => 'p',
                    'p' => 'q',
                    'q' => 'r',
                    'r' => 's',
                    's' => 't',
                    't' => 'u',
                    'u' => 'v',
                    'v' => 'w',
                    'w' => 'x',
                    'x' => 'y',
                    'y' => 'z',
                    'z' => 'a',
                    x => x,
                };
            }
            ch
        })
        .collect()
}

impl Day for Day04 {
    fn main() -> Result<()> {
        let input = get_input(2016, 4)?;
        let mut sector_sum: u32 = 0;
        for line in input.lines() {
            if let Some((_, name, sector, checksum)) =
                regex_captures!("([-a-z]+)-([0-9]+)\\[([a-z]+)\\]", &line)
            {
                if calculate_checksum(name) == checksum {
                    let sector: u32 = sector.parse()?;
                    sector_sum += sector;
                    let decoded = decode(name, sector);
                    if regex_is_match!("north\\s*pole", &decoded) {
                        println!("Storage room: {sector} -> {decoded}");
                    }
                }
            }
        }
        println!("Sum of sectors for real rooms: {sector_sum}");
        Ok(())
    }
}
