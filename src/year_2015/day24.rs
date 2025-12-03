use crate::util::input::get_input;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use itertools::Itertools;

use crate::day::Day;

pub struct Day24;

fn calculate_best_qe(weights: &[u64], compartments: u64) -> u64 {
    let target_weight: u64 = weights.iter().sum::<u64>() / compartments;
    let mut best: Option<(usize, u64)> = None;
    for group in weights.iter().powerset() {
        if let Some((best_len, _)) = best {
            if group.len() > best_len {
                break;
            }
        }
        if group.iter().map(|v| *v).sum::<u64>() == target_weight {
            let qe = group.iter().map(|v| *v).product();
            best = Some(match best {
                Some((best_len, best_qe)) => (best_len, if qe < best_qe { qe } else { best_qe }),
                None => (group.len(), qe),
            });
        }
    }
    best.map(|(_, qe)| qe).unwrap_or(u64::MAX)
}

impl Day for Day24 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2015, 24)?);
        let mut weights: Vec<u64> = Vec::new();
        for line in input.lines().map(|l| l.unwrap()) {
            weights.push(line.parse()?);
        }
        let best_qe = calculate_best_qe(&weights, 3);
        println!("Best qe of 3 compartments: {best_qe}");
        let best_qe = calculate_best_qe(&weights, 4);
        println!("Best qe of 4 compartments: {best_qe}");
        Ok(())
    }
}
