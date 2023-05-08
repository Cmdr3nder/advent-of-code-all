use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{bail, Result};

use crate::day::Day;
use crate::util::is_hex::IsHex;

pub struct Day08;

impl Day for Day08 {
    fn main() -> Result<()> {
        let input = BufReader::new(File::open("input/2015/day08.txt")?);
        let mut diff_part_a: usize = 0;
        let mut diff_part_b: usize = 0;
        for (ln, line) in input.lines().map(|l| l.unwrap()).enumerate() {
            let last = line.len() - 1;
            let mut chars = line.chars().enumerate();
            while let Some((i, ch)) = chars.next() {
                match (i == 0 || i == last, ch) {
                    (true, '"') => {
                        diff_part_a += 1;
                        diff_part_b += 2;
                    }
                    (false, '"') => bail!("Unexpected lonesome '\"' @ {ln}:{i}"),
                    (_, '\\') => match chars.next() {
                        Some((_, ch)) => match ch {
                            '"' | '\\' => {
                                diff_part_a += 1;
                                diff_part_b += 2;
                            }
                            'x' => match (chars.next(), chars.next()) {
                                (Some((ai, a)), Some((bi, b))) => {
                                    if !a.is_hex() {
                                        bail!("Char '{a}' is not a hex digit @ {ln}:{ai}");
                                    } else if !b.is_hex() {
                                        bail!("Char '{b}' is not a hex digit @ {ln}:{bi}");
                                    } else {
                                        diff_part_a += 3;
                                        diff_part_b += 1;
                                    }
                                }
                                _ => bail!("Missing char(s) following '\\x' @ {ln}:{i}"),
                            },
                            _ => bail!("Unexpected char '{ch}' after slash @ {ln}:{i}"),
                        },
                        None => bail!("Should have char after slash @ {ln}:{i}"),
                    },
                    _ => {}
                }
            }
        }
        println!("Difference between file and in-memory: {}", diff_part_a);
        println!("Difference between file and up-encoded: {}", diff_part_b);
        Ok(())
    }
}
