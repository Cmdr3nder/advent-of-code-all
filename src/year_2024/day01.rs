use std::cmp::Ordering;

use anyhow::{Context, Result};
use lazy_regex::regex_captures;

use crate::day::Day;
use crate::input::get_input;
use crate::util::ordered_vec::OrderedVec;

pub struct Day01;

impl Day for Day01 {
    fn main() -> Result<()> {
        let input_str = get_input(2024, 1)?;
        let mut left_nums: OrderedVec<u32> = OrderedVec::new();
        let mut right_nums: OrderedVec<u32> = OrderedVec::new();
        for line in input_str.lines() {
            let (_, left_str, right_str) = regex_captures!("([0-9]+)\\s+([0-9]+)", &line)
                .with_context(|| format!("Failed to match line regex {line}"))?;
            left_nums.push(left_str.parse()?);
            right_nums.push(right_str.parse()?);
        }

        let mut total_distance = 0;
        for (left_num, right_num) in left_nums.into_iter().zip(right_nums.into_iter()) {
            total_distance += left_num.abs_diff(*right_num);
        }
        println!("Total distance between lists: {total_distance}");

        let mut total_similarity = 0;
        let mut left_iter = left_nums.into_iter().peekable();
        let mut right_iter = right_nums.into_iter().peekable();
        loop {
            match left_iter.peek().zip(right_iter.peek()) {
                None => break,
                Some((left_num, right_num)) => match left_num.cmp(right_num) {
                    Ordering::Less => {
                        left_iter.next();
                    }
                    Ordering::Greater => {
                        right_iter.next();
                    }
                    Ordering::Equal => {
                        total_similarity += *right_num;
                        right_iter.next();
                    }
                },
            }
        }
        println!("Total similarity between lists: {total_similarity}");

        Ok(())
    }
}
