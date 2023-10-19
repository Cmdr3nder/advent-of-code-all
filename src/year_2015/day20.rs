use std::fs;

use anyhow::{bail, Context, Result};
use integer_sqrt::IntegerSquareRoot;

use crate::day::Day;

pub struct Day20;

fn sum_of_divisors(num: usize) -> usize {
    let mut n = num.integer_sqrt();
    let mut sum = 0;
    while n >= 1 {
        if num % n == 0 {
            sum += n;
            let d = num / n;
            if d != n {
                sum += d;
            }
        }
        n -= 1;
    }
    sum
}

fn present_count(house_num: usize) -> usize {
    sum_of_divisors(house_num) * 10
}

impl Day for Day20 {
    fn main() -> Result<()> {
        let presents: usize = fs::read_to_string("input/2015/day20.txt")?.trim().parse()?;
        for house_num in 1..=presents {
            if present_count(house_num) >= presents {
                println!("lowest house number: {house_num}");
                break;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_divisors() {
        let data: Vec<(usize, usize)> = vec![
            (1, 1),
            (2, 3),
            (3, 4),
            (4, 7),
            (5, 6),
            (6, 12),
            (7, 8),
            (8, 15),
            (9, 13),
            (10, 18),
            (11, 12),
            (12, 28),
            (13, 14),
            (14, 24),
            (15, 24),
            (16, 31),
            (17, 18),
            (18, 39),
            (19, 20),
            (20, 42),
        ];
        for (num, expected) in data {
            let actual = sum_of_divisors(num);
            assert_eq!(
                actual, expected,
                "Got {}, but expected {} when calling sum_of_divisors({})",
                actual, expected, num
            );
        }
    }
}
