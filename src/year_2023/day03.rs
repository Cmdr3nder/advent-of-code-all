use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

use crate::data::Point2D;
use crate::day::Day;

#[derive(Clone, Copy, Debug)]
struct Number {
    value: u32,
    y: usize,
    x_start: usize,
    x_stop: usize,
}

impl Number {
    fn new(value: u32, y: usize, x: usize) -> Self {
        Number {
            value,
            y,
            x_start: x,
            x_stop: x,
        }
    }
}

#[derive(Clone, Debug)]
struct Symbol {
    adjacent_nums: Vec<u32>,
    sym: char,
}

impl Symbol {
    fn new(sym: char) -> Self {
        Symbol {
            adjacent_nums: Vec::new(),
            sym,
        }
    }
}

pub struct Day03;

impl Day for Day03 {
    fn main() -> Result<()> {
        let input = BufReader::new(File::open("input/2023/day03.txt")?);
        let mut symbols: HashMap<Point2D<usize>, Symbol> = HashMap::new();
        let mut numbers: Vec<Number> = Vec::new();
        for (y, line) in input.lines().map(|l| l.unwrap()).enumerate() {
            let mut num_in_progress: Option<Number> = None;
            for (x, ch) in line.chars().enumerate() {
                match (ch, num_in_progress) {
                    ('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9', None) => {
                        num_in_progress = ch.to_digit(10).map(|d| Number::new(d, y, x));
                    }
                    ('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9', Some(mut num)) => {
                        num_in_progress = ch.to_digit(10).map(|d| {
                            num.value *= 10;
                            num.value += d;
                            num.x_stop = x;
                            num
                        });
                    }
                    ('.', None) => {}
                    ('.', Some(num)) => {
                        // End Number
                        numbers.push(num);
                        num_in_progress = None;
                    }
                    (sym, None) => {
                        symbols.insert(Point2D::new(x, y), Symbol::new(sym));
                    }
                    (sym, Some(num)) => {
                        symbols.insert(Point2D::new(x, y), Symbol::new(sym));
                        // End Number
                        numbers.push(num);
                        num_in_progress = None;
                    }
                }
            }
            if let Some(num) = num_in_progress {
                // End Number
                numbers.push(num);
            }
        }
        let mut sum_of_parts = 0;
        'number_sum: for num in numbers {
            let low_x = num.x_start.saturating_sub(1);
            let high_x = num.x_stop.saturating_add(1);
            // check row above
            if let Some(y) = num.y.checked_sub(1) {
                for x in low_x..=high_x {
                    if let Some(sym) = symbols.get_mut(&Point2D::new(x, y)) {
                        sym.adjacent_nums.push(num.value);
                        sum_of_parts += num.value;
                        continue 'number_sum;
                    }
                }
            }
            // check left
            if let Some(sym) = symbols.get_mut(&Point2D::new(low_x, num.y)) {
                sym.adjacent_nums.push(num.value);
                sum_of_parts += num.value;
                continue 'number_sum;
            }
            // check right
            if let Some(sym) = symbols.get_mut(&Point2D::new(high_x, num.y)) {
                sym.adjacent_nums.push(num.value);
                sum_of_parts += num.value;
                continue 'number_sum;
            }
            // check row below
            if let Some(y) = num.y.checked_add(1) {
                for x in low_x..=high_x {
                    if let Some(sym) = symbols.get_mut(&Point2D::new(x, y)) {
                        sym.adjacent_nums.push(num.value);
                        sum_of_parts += num.value;
                        continue 'number_sum;
                    }
                }
            }
        }
        println!("Sum of parts: {sum_of_parts}");
        let ratio_sum: u32 = symbols
            .values()
            .filter(|sym| sym.sym == '*' && sym.adjacent_nums.len() == 2)
            .map(|sym| sym.adjacent_nums[0] * sym.adjacent_nums[1])
            .sum();
        println!("Sum of ratios: {ratio_sum}");
        Ok(())
    }
}
