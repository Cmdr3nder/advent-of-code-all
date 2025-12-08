use anyhow::{bail, Result};
use std::io::{BufRead, BufReader};

use crate::day::Day;
use crate::util::input::get_input;

pub struct Day04;

fn count_available(current_row: &Vec<char>, prev_row: &Vec<char>, next_row: &Vec<char>) -> u64 {
    let mut available_count = 0;
    for (x, ch) in current_row.iter().enumerate() {
        let ch = *ch;
        if ch == '@' {
            let mut neighbor_count = 0;

            // Check (x - 1 column)
            if x > 0 {
                let check_x = x - 1;
                if check_x < prev_row.len() && prev_row[check_x] == '@' {
                    neighbor_count += 1;
                }
                if check_x < current_row.len() && current_row[check_x] == '@' {
                    neighbor_count += 1;
                }
                if check_x < next_row.len() && next_row[check_x] == '@' {
                    neighbor_count += 1;
                }
            }

            // Check (x column)
            {
                if x < prev_row.len() && prev_row[x] == '@' {
                    neighbor_count += 1;
                }
                if x < next_row.len() && next_row[x] == '@' {
                    neighbor_count += 1;
                }
            }

            // Check (x + 1 column)
            {
                let check_x = x + 1;
                if check_x < prev_row.len() && prev_row[check_x] == '@' {
                    neighbor_count += 1;
                }
                if check_x < current_row.len() && current_row[check_x] == '@' {
                    neighbor_count += 1;
                }
                if check_x < next_row.len() && next_row[check_x] == '@' {
                    neighbor_count += 1;
                }
            }

            if neighbor_count < 4 {
                available_count += 1;
            }
        }
    }
    available_count
}

impl Day for Day04 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2025, 4)?);

        let empty_row: Vec<char> = Vec::new();
        let mut recent_rows: Vec<Vec<char>> = Vec::with_capacity(3);

        let mut available_count = 0;
        for row in input.lines().map(|l| l.unwrap()) {
            if recent_rows.len() == 3 {
                // Drop oldest row
                recent_rows.remove(0);
            }
            // Add newest row
            recent_rows.push(row.chars().collect());
            match recent_rows.len() {
                1 => {
                    // Do nothing, we can't look ahead yet.
                },
                2 => {
                    // Special case, look at 'first' row while checking the second row.
                    available_count += count_available(&recent_rows[0], &empty_row, &recent_rows[1]);
                },
                3 => {
                    // Common case, look at 'second' row while checking the 'first' and 'third' rows.
                    available_count += count_available(&recent_rows[1], &recent_rows[0], &recent_rows[2]);
                },
                _ => bail!("Unexpected count for recent_rows, expected there to be at least 1 row and no more than 3"),
            }
        }
        // Process last row
        available_count += count_available(&recent_rows[2], &recent_rows[1], &empty_row);

        println!("Available rolls to move: {available_count}");

        Ok(())
    }
}
