use std::collections::HashMap;

use anyhow::{Context, Result};
use lazy_regex::regex_captures;

use crate::data::StringIdMap;
use crate::day::Day;
use crate::input::get_input;

pub struct Day13;

fn pair_happiness(happiness_chart: &HashMap<(usize, usize), i64>, a: usize, b: usize) -> i64 {
    happiness_chart.get(&(a, b)).unwrap_or(&0) + happiness_chart.get(&(b, a)).unwrap_or(&0)
}

fn find_best_happiness(
    people: &StringIdMap,
    happiness_chart: &HashMap<(usize, usize), i64>,
) -> i64 {
    let mut sequences: Vec<Vec<usize>> = vec![Vec::new()];
    for i in 0..people.count() {
        let mut new_sequences = Vec::new();
        for sequence in sequences {
            for j in 0..=sequence.len() {
                let mut s = sequence.clone();
                s.insert(j, i);
                new_sequences.push(s);
            }
        }
        sequences = new_sequences
    }
    let mut best_happiness = i64::MIN;
    for sequence in sequences {
        let mut happiness = 0;
        for i in 0..sequence.len() {
            happiness += pair_happiness(
                happiness_chart,
                sequence[i],
                sequence[(i + 1) % sequence.len()],
            );
        }
        if happiness > best_happiness {
            best_happiness = happiness;
        }
    }
    best_happiness
}

impl Day for Day13 {
    fn main() -> Result<()> {
        let input = get_input(2015, 13)?;
        let mut people = StringIdMap::default();
        let mut happiness_chart: HashMap<(usize, usize), i64> = HashMap::new();
        for line in input.lines() {
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
