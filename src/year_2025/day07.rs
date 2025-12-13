use anyhow::{bail, Result};
use std::io::{BufRead, BufReader};

use crate::day::Day;
use crate::util::input::get_input;

pub struct Day07;

impl Day for Day07 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2025, 7)?);
        let mut tachyon_scan: Vec<u64> = Vec::new();
        let mut split_count: u64 = 0;
        for line in input.lines().map(|l| l.unwrap()) {
            let mut tachyon_prop = tachyon_scan.clone();
            for (i, ch) in line.chars().enumerate() {
                if i >= tachyon_scan.len() {
                    tachyon_scan.resize(i + 1, 0);
                }
                if i >= tachyon_prop.len() {
                    tachyon_prop.resize(i + 1, 0);
                }
                match ch {
                    '.' => {
                        // Empty Space, Vertical Prop
                        // Handled by tachyon_scan.clone() above
                    }
                    'S' => {
                        // Tachyon Spawnpoint
                        tachyon_prop[i] = 1;
                    }
                    '^' => {
                        // Tachyon Splitpoint
                        tachyon_prop[i] = 0; // Shadow
                        if tachyon_scan[i] > 0 {
                            split_count += 1;
                            if i > 0 {
                                tachyon_prop[i - 1] += tachyon_scan[i];
                            }
                            if i < tachyon_prop.len() - 1 {
                                tachyon_prop[i + 1] += tachyon_scan[i];
                            }
                        }
                    }
                    _ => bail!("Unexpected input '{ch}'"),
                }
            }
            tachyon_scan = tachyon_prop;
        }

        println!("Tachyon Beam Split Count: {split_count}");

        let mut timeline_count: u64 = 0;
        for t in tachyon_scan {
            timeline_count += t;
        }

        println!("Timeline Count: {timeline_count}");

        Ok(())
    }
}
