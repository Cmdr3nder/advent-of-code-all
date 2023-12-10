use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

use crate::day::Day;

fn score_for_count(count: usize) -> u32 {
    match count {
        0 => 0,
        1 => 1,
        _ => {
            let mut score = 1;
            for _ in 1..count {
                score *= 2;
            }
            score
        }
    }
}

pub struct Day04;

impl Day for Day04 {
    fn main() -> Result<()> {
        let input = BufReader::new(File::open("input/2023/day04.txt")?);
        let cards: Vec<(HashSet<u32>, HashSet<u32>)> = input
            .lines()
            .map(|l| {
                let line = l.unwrap();
                let mut lr = line.split(": ").last().unwrap_or("").split(" | ").map(|g| {
                    g.split(" ")
                        .map(|n| n.trim().parse::<u32>())
                        .filter(|n| n.is_ok())
                        .map(|n| n.unwrap())
                        .collect::<HashSet<u32>>()
                });
                match (lr.next(), lr.next()) {
                    (Some(a), Some(b)) => Some((a, b)),
                    _ => None,
                }
            })
            .filter(|p| p.is_some())
            .map(|p| p.unwrap())
            .collect();
        let points: u32 = cards
            .iter()
            .map(|(winning, have)| score_for_count(winning.intersection(have).count()))
            .sum();
        println!("Scratchits worth: {points}");
        Ok(())
    }
}
