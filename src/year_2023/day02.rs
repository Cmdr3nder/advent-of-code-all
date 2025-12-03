use crate::util::input::get_input;
use std::io::{BufRead, BufReader};

use anyhow::{bail, Context, Result};

use crate::day::Day;

pub struct Day02;

impl Day for Day02 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2023, 02)?);
        let mut id_sum = 0;
        let mut power_sum = 0;
        for line in input.lines().map(|l| l.unwrap()) {
            let parts: Vec<&str> = line.split(": ").collect();
            if parts.len() != 2 {
                bail!("No header/body split for '{line}'");
            }
            let game_num: u32 = parts[0]
                .split(" ")
                .last()
                .with_context(|| "Can't extract game number")?
                .parse()?;
            let mut possible = true;
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;
            for pull in parts[1].split("; ") {
                for color_count in pull.split(", ") {
                    let cc: Vec<&str> = color_count.split(" ").collect();
                    if cc.len() != 2 {
                        bail!("No color/count split for '{color_count}'");
                    }
                    let count: u32 = cc[0].parse()?;
                    match cc[1] {
                        "red" => {
                            possible = possible && count <= 12;
                            if min_red < count {
                                min_red = count;
                            }
                        }
                        "green" => {
                            possible = possible && count <= 13;
                            if min_green < count {
                                min_green = count;
                            }
                        }
                        "blue" => {
                            possible = possible && count <= 14;
                            if min_blue < count {
                                min_blue = count;
                            }
                        }
                        x => bail!("Unexpected cube color '{x}'"),
                    }
                }
            }
            if possible {
                id_sum += game_num;
            }
            power_sum += min_red * min_green * min_blue;
        }
        println!("Valid game id sum: {id_sum}");
        println!("Minimum power sum: {power_sum}");
        Ok(())
    }
}
