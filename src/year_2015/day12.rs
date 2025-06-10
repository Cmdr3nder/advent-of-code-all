use std::iter::Sum;

use anyhow::Result;
use serde_json::{self, Value};

use crate::day::Day;
use crate::input::get_input;

pub struct Day12;

#[derive(Clone, Copy)]
struct SometimesRed(i64, i64);

impl Sum for SometimesRed {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = SometimesRed>,
    {
        let mut left = 0;
        let mut right = 0;
        for item in iter {
            left += item.0;
            right += item.1;
        }
        SometimesRed(left, right)
    }
}

fn sum_numbers(value: &Value) -> SometimesRed {
    match value {
        Value::Number(n) => match n.as_i64() {
            Some(i) => SometimesRed(i, i),
            None => SometimesRed(0, 0),
        },
        Value::Array(vec) => vec.iter().map(|v| sum_numbers(v)).sum(),
        Value::Object(map) => {
            let mut has_red = false;
            let mut left = 0;
            let mut right = 0;
            for (_, value) in map {
                match value {
                    Value::String(s) => {
                        if s == "red" {
                            has_red = true;
                        }
                    }
                    _ => {
                        let nums = sum_numbers(value);
                        left += nums.0;
                        right += nums.1;
                    }
                }
            }
            if has_red {
                right = 0;
            }
            SometimesRed(left, right)
        }
        _ => SometimesRed(0, 0),
    }
}

impl Day for Day12 {
    fn main() -> Result<()> {
        let input = get_input(2015, 12)?;
        let sums = sum_numbers(&serde_json::from_str(&input)?);
        println!("Sum of all numbers: {}", sums.0);
        println!("Sum of non-red numbers: {}", sums.1);
        Ok(())
    }
}
