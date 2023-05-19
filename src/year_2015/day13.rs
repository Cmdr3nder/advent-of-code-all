use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};
use lazy_regex::regex_captures;
use permute::permute;

use crate::data::StringIdMap;
use crate::day::Day;

pub struct Day13;

fn find_best_happiness(
    people: &StringIdMap,
    happiness_chart: &HashMap<(usize, usize), i64>,
) -> i64 {
    let mut best_happiness = i64::MIN;
    for permutation in permute((0..people.count()).collect::<Vec<usize>>()) {
        let mut happiness: i64 = 0;
        for p_idx in 0..permutation.len() {
            let person = permutation[p_idx];
            let left_neighbor = if p_idx == 0 {
                permutation.len() - 1
            } else {
                p_idx - 1
            };
            let left_neighbor = permutation[left_neighbor];
            let right_neighbor = permutation[(p_idx + 1) % permutation.len()];
            let left_happiness: i64 = happiness_chart
                .get(&(person, left_neighbor))
                .map(|h| *h)
                .unwrap_or(0);
            let right_happiness: i64 = happiness_chart
                .get(&(person, right_neighbor))
                .map(|h| *h)
                .unwrap_or(0);
            happiness += left_happiness + right_happiness;
        }
        if happiness > best_happiness {
            best_happiness = happiness;
        }
    }
    best_happiness
}

impl Day for Day13 {
    fn main() -> Result<()> {
        let input = BufReader::new(File::open("input/2015/day13.txt")?);
        let mut people = StringIdMap::default();
        let mut happiness_chart: HashMap<(usize, usize), i64> = HashMap::new();
        for line in input.lines().map(|l| l.unwrap()) {
            let (_, person, sign, happiness, neighbor) = regex_captures!(
                "([A-Za-z]+) would (gain|lose) ([0-9]+) happiness units by sitting next to ([A-Za-z]+)",
                &line,
            ).with_context(|| "Could not match happiness meters")?;
            let person = people.to_id(person);
            let neighbor = people.to_id(neighbor);
            let mut happiness: i64 = happiness.parse()?;
            if sign == "lose" {
                happiness *= -1;
            }
            happiness_chart.insert((person, neighbor), happiness);
        }
        println!(
            "Best happiness without myself: {}",
            find_best_happiness(&people, &happiness_chart)
        );
        people.to_id("---MYSELF---");
        println!(
            "Best happiness with myself: {}",
            find_best_happiness(&people, &happiness_chart)
        );
        Ok(())
    }
}
