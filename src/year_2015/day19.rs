use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{bail, Context, Result};
use lazy_regex::regex_captures;
use priority_queue::PriorityQueue;

use crate::day::Day;

pub struct Day19;

impl Day for Day19 {
    fn main() -> Result<()> {
        let input = BufReader::new(File::open("input/2015/day19.txt")?);
        let mut reverse_map: HashMap<String, String> = HashMap::new();
        let mut medicine: String = String::new();
        for line in input.lines().map(|l| l.unwrap()) {
            if let Some((_, from, to)) = regex_captures!("([A-Za-z]+) => ([A-Za-z]+)", &line) {
                if reverse_map.contains_key(to) {
                    // Check assumption about only one way to reverse a 'long' string
                    bail!("Unexpected existing reverse");
                }
                reverse_map.insert(to.to_string(), from.to_string());
            } else if line != "" {
                medicine = line;
            }
        }
        let mut possibilities: HashSet<String> = HashSet::new();
        for (to, from) in &reverse_map {
            for (index, _) in medicine.match_indices(from) {
                let mut med = medicine.clone();
                med.replace_range(index..index + from.len(), to);
                possibilities.insert(med);
            }
        }
        println!("Distinct molecules: {}", possibilities.len());
        let mut ancestors = PriorityQueue::new();
        let len = medicine.len();
        ancestors.push((0, medicine), Reverse(len));
        loop {
            if let Some(((steps, ancestor), _)) = ancestors.pop() {
                if ancestor == "e" {
                    println!("Fewest steps to 'e': {steps}");
                    break;
                }

                for (to, from) in &reverse_map {
                    for (index, _) in ancestor.match_indices(to) {
                        let mut anc = ancestor.clone();
                        anc.replace_range(index..index + to.len(), from);
                        let len = anc.len();
                        ancestors.push((steps + 1, anc), Reverse(len));
                    }
                }
            } else {
                println!("Could not find steps to 'e'");
                break;
            }
        }
        Ok(())
    }
}
