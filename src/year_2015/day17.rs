use anyhow::{Context, Result};

use crate::day::Day;
use crate::input::get_input;

pub struct Day17;

const EXPECTED_NOG: u32 = 150;

impl Day for Day17 {
    fn main() -> Result<()> {
        let input_str = get_input(2015, 17)?;
        let mut fills: Vec<(u32, u32)> = Vec::new(); // (container_count, fill_amount)
        fills.push((0, 0));
        for line in input_str.lines() {
            let container: u32 = line
                .parse()
                .with_context(|| format!("Expected integer, got '{line}'"))?;
            let mut new_fills: Vec<(u32, u32)> = Vec::with_capacity(fills.len());
            for (count, fill) in fills {
                let sum = fill + container;
                if sum <= EXPECTED_NOG {
                    new_fills.push((count + 1, sum));
                }
                new_fills.push((count, fill));
            }
            fills = new_fills
        }
        fills.retain(|(_, fill)| *fill == EXPECTED_NOG);
        println!("150 liter fill count: {:?}", fills.len());
        let mut min_containers = u32::MAX;
        let mut instances = 0;
        for (count, _) in fills {
            if count < min_containers {
                min_containers = count;
                instances = 1;
            } else if count == min_containers {
                instances += 1;
            }
        }
        println!("Minimum number of containers needed is {min_containers}, which occurs {instances} times");
        Ok(())
    }
}
