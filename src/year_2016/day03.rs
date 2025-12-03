use crate::util::input::get_input;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use lazy_regex::regex_captures;

use crate::day::Day;

pub struct Day03;

fn good_triangle(sides: [u32; 3]) -> bool {
    let mut sum = 0;
    let mut large = 0;
    for s in sides {
        sum += s;
        if s > large {
            large = s;
        }
    }
    (sum - large) > large
}

impl Day for Day03 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2016, 03)?);
        let mut horizontal_count = 0;
        let mut vertical_count = 0;
        let mut window = [[0u32; 3]; 3];
        for (num, line) in input.lines().map(|l| l.unwrap()).enumerate() {
            let cycle = num % 3;
            if let Some((_, a, b, c)) = regex_captures!("([0-9]+)\\s+([0-9]+)\\s+([0-9]+)", &line) {
                let a: u32 = a.parse()?;
                let b: u32 = b.parse()?;
                let c: u32 = c.parse()?;

                // Check horizontal line
                if good_triangle([a, b, c]) {
                    horizontal_count += 1;
                }

                window[0][cycle] = a;
                window[1][cycle] = b;
                window[2][cycle] = c;

                if cycle == 2 {
                    // Check vertical windows
                    for triangle in window {
                        if good_triangle(triangle) {
                            vertical_count += 1;
                        }
                    }
                }
            }
        }
        println!("Horizontal Possible Triangles: {horizontal_count}");
        println!("Vertical Possible Triangles: {vertical_count}");
        Ok(())
    }
}
