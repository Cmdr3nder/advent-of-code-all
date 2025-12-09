use anyhow::{bail, Context, Result};
use lazy_regex::regex_captures;
use std::cmp::Ordering;
use std::io::{BufRead, BufReader};

use crate::day::Day;
use crate::util::input::get_input;

pub struct Day05;

#[derive(Clone, Copy)]
enum ParseMode {
    FreshRanges,
    AvailableIngredients,
}

type Ingredient = u64;
type FreshRange = (Ingredient, Ingredient);
type FreshRanges = Vec<FreshRange>;

fn is_fresh(fresh_ranges: &FreshRanges, ingredient: &Ingredient) -> bool {
    for (low, high) in fresh_ranges {
        if ingredient < low {
            // We've already gone past any ranges that might have included that number
            break;
        } else if ingredient <= high {
            // In range, so fresh!
            return true;
        }
    }
    false
}

fn merge_adjacent(fresh_ranges: &mut FreshRanges, i: usize) {
    let (_, high) = fresh_ranges[i];
    let j = i + 1;
    while j < fresh_ranges.len() {
        match (high.cmp(&fresh_ranges[j].0), high.cmp(&fresh_ranges[j].1)) {
            // Non overlapping
            (Ordering::Less, _) => {
                break;
            }
            // Overlapping inside
            (Ordering::Equal, _)
            | (Ordering::Greater, Ordering::Less)
            | (Ordering::Greater, Ordering::Equal) => {
                fresh_ranges[i].1 = fresh_ranges[j].1;
                fresh_ranges.remove(j);
                break;
            }
            // Consuming
            (Ordering::Greater, Ordering::Greater) => {
                fresh_ranges.remove(j);
                // Continue
            }
        }
    }
}

impl Day for Day05 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2025, 5)?);

        let mut mode = ParseMode::FreshRanges;
        let mut fresh_ranges = FreshRanges::new();
        let mut available_fresh = 0;
        for line in input.lines().map(|l| l.unwrap()) {
            match (mode, line.len()) {
                (ParseMode::FreshRanges, 0) => {
                    mode = ParseMode::AvailableIngredients;
                }
                (ParseMode::FreshRanges, _) => {
                    let (_, low_str, high_str) = regex_captures!(r"([0-9]+)-([0-9]+)", &line)
                        .with_context(|| "Fresh range not in correct format")?;
                    let (low, high): FreshRange = (low_str.parse()?, high_str.parse()?);
                    let mut i = 0;
                    while i < fresh_ranges.len() {
                        match (
                            low.cmp(&fresh_ranges[i].0),
                            high.cmp(&fresh_ranges[i].0),
                            low.cmp(&fresh_ranges[i].1),
                            high.cmp(&fresh_ranges[i].1),
                        ) {
                            // Less than
                            (Ordering::Less, Ordering::Less, _, _) => {
                                fresh_ranges.insert(i, (low, high));
                                break;
                            }
                            // Greater than
                            (_, _, Ordering::Greater, Ordering::Greater) => {
                                i += 1;
                            }
                            // Overlaping low
                            (
                                Ordering::Less,
                                Ordering::Equal | Ordering::Greater,
                                _,
                                Ordering::Less | Ordering::Equal,
                            ) => {
                                // New low for range
                                fresh_ranges[i].0 = low;
                                break;
                            }
                            // Containing
                            (Ordering::Less, _, _, Ordering::Greater) => {
                                // Subsume existing range
                                fresh_ranges[i].0 = low;
                                fresh_ranges[i].1 = high;
                                merge_adjacent(&mut fresh_ranges, i);
                                break;
                            }
                            // Contained
                            (
                                Ordering::Equal | Ordering::Greater,
                                _,
                                _,
                                Ordering::Equal | Ordering::Less,
                            ) => {
                                // Ignore new range
                                break;
                            }
                            // Overlaping high
                            (
                                Ordering::Equal | Ordering::Greater,
                                _,
                                Ordering::Equal | Ordering::Less,
                                Ordering::Greater,
                            ) => {
                                // New high for range
                                fresh_ranges[i].1 = high;
                                merge_adjacent(&mut fresh_ranges, i);
                            }
                        }
                    }
                    // Detect greater than all case
                    if i >= fresh_ranges.len() {
                        fresh_ranges.push((low, high));
                    }
                }
                (ParseMode::AvailableIngredients, 0) => bail!("Unexpected blank line"),
                (ParseMode::AvailableIngredients, _) => {
                    let ingredient = line.parse::<Ingredient>()?;
                    if is_fresh(&fresh_ranges, &ingredient) {
                        available_fresh += 1;
                    }
                }
            }
        }

        let mut fresh_id_count = 0;
        for (low, high) in fresh_ranges {
            fresh_id_count += high - low + 1;
        }

        println!("Available fresh ingredients: {available_fresh}");
        println!("Fresh IDs count: {fresh_id_count}");

        Ok(())
    }
}
