use anyhow::{Context, Result};
use lazy_regex::regex;

use crate::day::Day;
use crate::util::input::get_input_string;

pub struct Day02;

fn is_invalid(n: u64) -> bool {
    if n < 10 {
        return false;
    }
    let mut left = n;
    let mut right = 0;
    let mut factor = 1;
    let mut moved = 0;
    while left > right {
        right += (left % 10) * factor;
        left /= 10;
        factor *= 10;
        moved += 1;
    }
    if left == right {
        while left > 0 {
            left /= 10;
            moved -= 1;
        }
        return moved == 0;
    }
    return false;
}

fn is_invalid_patterned(n: u64) -> bool {
    if n < 10 {
        return false;
    }
    let n = n.to_string();
    let len = n.len();
    for l in 1..len / 2 {
        if len % l == 0 {
            // Pattern can be evenly distributed
            let pattern = &n[0..l];
            let mut i = l;
            let mut matching = true;
            while i < len && matching {
                let chunk = &n[i..i + l];
                i += l;
                matching = pattern == chunk;
            }
            if matching {
                return true;
            }
        }
    }
    return false;
}

impl Day for Day02 {
    fn main() -> Result<()> {
        let input = get_input_string(2025, 2)?;
        let re = regex!(r"([0-9]+)-([0-9]+)");
        let mut sum: u64 = 0;
        let mut sum_patterned: u64 = 0;

        for match_group in re.captures_iter(&input) {
            let from = match_group
                .get(1)
                .with_context(|| "No match for from")
                .and_then(|m| {
                    m.as_str()
                        .parse::<u64>()
                        .with_context(|| "Could not parse from")
                })?;
            let to = match_group
                .get(2)
                .with_context(|| "No match for to")
                .and_then(|m| {
                    m.as_str()
                        .parse::<u64>()
                        .with_context(|| "Could not parse to")
                })?;
            for n in from..to + 1 {
                if is_invalid(n) {
                    sum += n;
                    sum_patterned += n;
                } else if is_invalid_patterned(n) {
                    sum_patterned += n;
                }
            }
        }

        println!("Sum of invalid IDs: {sum}");
        println!("Sum of patterned invalid Ids: {sum_patterned}");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid() {
        let valid: Vec<u64> = vec![0, 1, 3, 9, 10, 12, 15, 19, 20, 21, 100, 101, 1000];
        let invalid: Vec<u64> = vec![11, 22, 66, 1010];
        for n in valid {
            assert!(!is_invalid(n), "Expected {n} to be valid!");
        }
        for n in invalid {
            assert!(is_invalid(n), "Expected {n} to be invalid!");
        }
    }
}
