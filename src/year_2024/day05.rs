use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};
use lazy_regex::regex_captures;

use crate::day::Day;
use crate::input::get_input;

#[derive(Clone, Copy)]
enum ParseMode {
    Rules,
    Pages,
}

pub struct Day05;

impl Day for Day05 {
    fn main() -> Result<()> {
        let input = get_input(2024, 5)?;
        let mut mode = ParseMode::Rules;
        let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
        let mut sum_middle_pages = 0;
        let mut sum_corrected = 0;
        for line in input.lines() {
            match mode {
                ParseMode::Rules => {
                    if line.is_empty() {
                        mode = ParseMode::Pages;
                    } else {
                        // Parse Rule
                        let (_, before_str, after_str) =
                            regex_captures!(r"([0-9]+)\|([0-9]+)", &line)
                                .with_context(|| "Rule not in correct format")?;
                        let before: u32 = before_str.parse().with_context(|| "rule before")?;
                        let after: u32 = after_str.parse().with_context(|| "rule after")?;
                        if rules.get(&after).is_none() {
                            rules.insert(after, HashSet::new());
                        }
                        rules
                            .get_mut(&after)
                            .with_context(|| {
                                "Unexpected missing HashSet, should have been inserted"
                            })?
                            .insert(before);
                    }
                }
                ParseMode::Pages => {
                    // Parse & Check Page Order
                    let mut pages = line
                        .split(",")
                        .map(|page_str| {
                            page_str
                                .parse::<u32>()
                                .with_context(|| "Could not parse page in list")
                        })
                        .collect::<Result<Vec<u32>>>()?;
                    let mut good_order = true;
                    for (after_index, after) in pages.iter().enumerate() {
                        if let Some(pages_before) = rules.get(&after) {
                            for before in pages_before {
                                if let Some(before_index) = pages
                                    .iter()
                                    .enumerate()
                                    .filter_map(
                                        |(index, page)| {
                                            if page == before {
                                                Some(index)
                                            } else {
                                                None
                                            }
                                        },
                                    )
                                    .last()
                                {
                                    if before_index > after_index {
                                        good_order = false;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    if good_order {
                        sum_middle_pages += pages[pages.len() / 2];
                    } else {
                        // Fix Page Order
                        pages.sort_by(|a, b| {
                            if let Some(pages_before) = rules.get(&a) {
                                if pages_before.contains(&b) {
                                    return Ordering::Greater;
                                }
                            }
                            if let Some(pages_before) = rules.get(&b) {
                                if pages_before.contains(&a) {
                                    return Ordering::Less;
                                }
                            }
                            a.cmp(b)
                        });
                        sum_corrected += pages[pages.len() / 2];
                    }
                }
            }
        }
        println!("Correct middle page sum: {sum_middle_pages}");
        println!("Corrected middle page sum: {sum_corrected}");
        Ok(())
    }
}
