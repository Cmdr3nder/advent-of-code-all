use anyhow::{Context, Result};
use lazy_regex::regex_captures;

use crate::data::Point2D;
use crate::day::Day;
use crate::input::get_input;

pub struct Day06;

const DIMENSIONS: (Point2D<usize>, Point2D<usize>) =
    (Point2D::new(0, 0), Point2D::new(1000 - 1, 1000 - 1));

impl Day for Day06 {
    fn main() -> Result<()> {
        let input_str = get_input(2015, 6)?;
        let mut lights_bool: [bool; 1_000_000] = [false; 1_000_000];
        let mut lights_num: [u32; 1_000_000] = [0; 1_000_000];
        for line in input_str.lines() {
            let (_, action, x1, y1, x2, y2) = regex_captures!(
                "(turn on|toggle|turn off) ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)",
                &line
            )
            .with_context(|| format!("Failed to match line regex {line}"))?;
            let from = Point2D::<usize>::new(x1.parse()?, y1.parse()?);
            let to = Point2D::<usize>::new(x2.parse()?, y2.parse()?);
            for p in from.iter_to(&to) {
                let idx = p.to_index(&DIMENSIONS).unwrap();
                match action {
                    "turn on" => {
                        lights_bool[idx] = true;
                        lights_num[idx] += 1;
                    }
                    "toggle" => {
                        lights_bool[idx] = !lights_bool[idx];
                        lights_num[idx] += 2;
                    }
                    "turn off" => {
                        lights_bool[idx] = false;
                        lights_num[idx] = match lights_num[idx] {
                            0 => 0,
                            x => x - 1,
                        };
                    }
                    _ => panic!("Unexpected instruction"),
                }
            }
        }

        println!("Lit lights: {}", lights_bool.iter().filter(|l| **l).count());
        println!("Total brightness: {}", lights_num.iter().sum::<u32>());
        Ok(())
    }
}
