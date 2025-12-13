use anyhow::{bail, Context, Result};
use lazy_regex::regex;
use std::io::{BufRead, BufReader};

use crate::day::Day;
use crate::util::input::get_input;

pub struct Day06;

impl Day for Day06 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2025, 6)?);
        let re_nums = regex!(r"([0-9]+)");
        let re_symbols = regex!(r"([*+])");
        let mut columns: Vec<Vec<u64>> = Vec::new();
        let mut total: u64 = 0;
        let mut char_columns: Vec<Vec<char>> = Vec::new();
        for line in input.lines().map(|l| l.unwrap()) {
            // Part One
            for (col, match_group) in re_nums.captures_iter(&line).enumerate() {
                let num = match_group
                    .get(1)
                    .with_context(|| "No actual match")
                    .and_then(|m| {
                        m.as_str()
                            .parse::<u64>()
                            .with_context(|| "Could not parse from")
                    })?;
                while col >= columns.len() {
                    columns.push(Vec::new());
                }
                columns[col].push(num);
            }

            for (col, match_group) in re_symbols.captures_iter(&line).enumerate() {
                while col >= columns.len() {
                    columns.push(Vec::new());
                }
                let sym = match_group
                    .get(1)
                    .with_context(|| "No actual match")
                    .map(|m| m.as_str())?;
                match sym {
                    "*" => {
                        let mut sub: u64 = 1;
                        for n in &columns[col] {
                            sub *= n;
                        }
                        total += sub;
                    }
                    "+" => {
                        for n in &columns[col] {
                            total += n;
                        }
                    }
                    _ => bail!("Unexpected symbol '{sym}'"),
                }
            }

            // Part 2
            for (col, ch) in line.chars().enumerate() {
                while col >= char_columns.len() {
                    char_columns.push(Vec::new());
                }
                if ch == '+' || ch == '*' || ch >= '0' && ch <= '9' {
                    char_columns[col].push(ch);
                }
            }
        }

        // Part 1 done
        println!("Column answers sum: {total}");

        // Part 2 continued
        let mut total: u64 = 0;
        let mut sub: u64 = 0;
        let mut sym = '+';
        for char_col in char_columns {
            let mut char_slice = &char_col[..];
            if char_slice.len() > 0 {
                match char_slice[char_slice.len() - 1] {
                    '+' => {
                        sym = '+';
                        total += sub;
                        sub = 0;
                        char_slice = &char_slice[..char_slice.len() - 1];
                    }
                    '*' => {
                        sym = '*';
                        total += sub;
                        sub = 1;
                        char_slice = &char_slice[..char_slice.len() - 1];
                    }
                    _ => {}
                }
                let n = char_slice.iter().collect::<String>().parse::<u64>()?;
                match sym {
                    '+' => {
                        sub += n;
                    }
                    '*' => {
                        sub *= n;
                    }
                    _ => bail!("Unexpected control char '{sym}'"),
                }
            }
        }
        total += sub;

        println!("Group answers sum: {total}");

        Ok(())
    }
}
