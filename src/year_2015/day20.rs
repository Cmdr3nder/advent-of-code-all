use std::fs;

use anyhow::{bail, Context, Result};
use integer_sqrt::IntegerSquareRoot;

use crate::day::Day;

pub struct Day20;

fn sum_of_divisors<F>(num: usize, filter: F) -> usize where
    F: Fn(usize, usize) -> bool {
    let mut n = num.integer_sqrt();
    let mut sum = 0;
    while n >= 1 {
        if num % n == 0 {
            if filter(num, n) {
                sum += n;
            }
            let d = num / n;
            if d != n && filter(num, d) {
                sum += d;
            }
        }
        n -= 1;
    }
    sum
}

fn present_count<F>(house_num: usize, filter: F, mult: usize) -> usize where
    F: Fn(usize, usize) -> bool {
    sum_of_divisors(house_num, filter) * mult
}

impl Day for Day20 {
    fn main() -> Result<()> {
        let presents: usize = fs::read_to_string("input/2015/day20.txt")?.trim().parse()?;
        let mut d1 = false;
        let mut d2 = false;
        for house_num in 1..=presents {
            if !d1 && present_count(house_num, |_, _| true, 10) >= presents {
                println!("lowest house number: {house_num}");
                d1 = true;
            }
            if !d2 && present_count(house_num, |num, d| d * 50 >= num, 11) >= presents {
                println!("new lowest house number: {house_num}");
                d2 = true;
            }
            if d1 && d2 {
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
