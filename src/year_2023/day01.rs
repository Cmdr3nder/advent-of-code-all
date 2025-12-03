use crate::util::input::get_input;
use std::io::{BufRead, BufReader};

use anyhow::{bail, Result};

use crate::day::Day;

fn digit_parse(ch: char) -> Result<u32> {
    Ok(match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => bail!("Unexpected char digit '{ch}'"),
    })
}

#[derive(Clone)]
struct Observer {
    seq: Vec<char>,
    index: usize,
    value: u32,
}

impl Observer {
    fn new(s: &str, value: u32) -> Self {
        Observer {
            seq: s.chars().collect(),
            index: 0,
            value,
        }
    }

    fn observe(&mut self, ch: char) -> Option<u32> {
        if ch == self.seq[self.index] {
            self.index += 1;
        } else if ch == self.seq[0] {
            // If letter is correct start, but found during a previous sequence like 'eeight'
            self.index = 1;
        } else {
            self.index = 0;
        }
        if self.index >= self.seq.len() {
            self.index = 0;
            Some(self.value)
        } else {
            None
        }
    }
}

pub struct Day01;

impl Day for Day01 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2023, 01)?);
        let mut observers = vec![
            Observer::new("0", 0),
            Observer::new("1", 1),
            Observer::new("2", 2),
            Observer::new("3", 3),
            Observer::new("4", 4),
            Observer::new("5", 5),
            Observer::new("6", 6),
            Observer::new("7", 7),
            Observer::new("8", 8),
            Observer::new("9", 9),
            Observer::new("zero", 0),
            Observer::new("one", 1),
            Observer::new("two", 2),
            Observer::new("three", 3),
            Observer::new("four", 4),
            Observer::new("five", 5),
            Observer::new("six", 6),
            Observer::new("seven", 7),
            Observer::new("eight", 8),
            Observer::new("nine", 9),
        ];
        let mut sum = 0;
        let mut sum_spelled = 0;
        for line in input.lines().map(|l| l.unwrap()) {
            let mut chars = line.chars().filter(|ch| ch.is_digit(10));
            if let Some(first_digit) = chars.next() {
                let first_digit: u32 = digit_parse(first_digit)?;
                let last_digit: u32 = match chars.last() {
                    Some(digit) => digit_parse(digit)?,
                    None => first_digit,
                };
                sum += (first_digit * 10) + last_digit;
            } else {
                bail!("No Digit: '{line}'");
            }
            let mut digits: Option<(u32, u32)> = None;
            for ch in line.chars() {
                for obs in &mut observers {
                    if let Some(digit) = obs.observe(ch) {
                        digits = Some(digits.map_or((digit, digit), |(first, _)| (first, digit)));
                    }
                }
            }
            if let Some((first_digit, last_digit)) = digits {
                sum_spelled += (first_digit * 10) + last_digit;
            } else {
                bail!("No Digit: '{line}'");
            }
        }
        println!("Sum of calibration values: {sum}");
        println!("Sum of calibration values with spelled digits: {sum_spelled}");
        Ok(())
    }
}
