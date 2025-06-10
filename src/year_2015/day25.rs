use anyhow::{Context, Result};
use lazy_regex::regex_captures;

use crate::day::Day;
use crate::input::get_input;

struct DiagonalIterator {
    row: usize,
    col: usize,
}

impl DiagonalIterator {
    fn new() -> Self {
        DiagonalIterator { row: 1, col: 1 }
    }
}

impl Iterator for DiagonalIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let res = (self.row, self.col);
        self.row -= 1;
        self.col += 1;
        if self.row == 0 {
            self.row = self.col;
            self.col = 1;
        }
        Some(res)
    }
}

pub struct Day25;

impl Day for Day25 {
    fn main() -> Result<()> {
        let input = get_input(2015, 25)?;
        let (_, target_row, target_col) =
            regex_captures!(".+?row ([0-9]+).+?column ([0-9]+)", &input)
                .with_context(|| "Couldn't parse input")?;
        let target: (usize, usize) = (target_row.parse()?, target_col.parse()?);
        let mut code: u64 = 20151125;
        let mut di = DiagonalIterator::new();
        di.next();
        for p in di {
            code *= 252533;
            code %= 33554393;
            if p == target {
                break;
            }
        }
        println!("Code at {target:?} is {code}");
        Ok(())
    }
}
