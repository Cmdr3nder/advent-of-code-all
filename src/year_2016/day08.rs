use crate::util::input::get_input;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use lazy_regex::regex_captures;

use crate::day::Day;

pub struct Day08;

#[derive(Copy, Clone, PartialEq)]
enum Pixel {
    On,
    Off,
}

impl Day for Day08 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2016, 08)?);
        let mut display = [[Pixel::Off; 50]; 6];
        for line in input.lines().map(|l| l.unwrap()) {
            if let Some((_, a, b)) = regex_captures!("rect ([0-9]+)x([0-9]+)", &line) {
                let a: usize = a.parse()?;
                let b: usize = b.parse()?;
                for y in 0..b {
                    for x in 0..a {
                        display[y][x] = Pixel::On;
                    }
                }
            } else if let Some((_, y, by)) =
                regex_captures!("rotate row y=([0-9]+) by ([0-9]+)", &line)
            {
                let y: usize = y.parse()?;
                let by: usize = by.parse()?;
                display[y].rotate_right(by % 50);
            } else if let Some((_, x, by)) =
                regex_captures!("rotate column x=([0-9]+) by ([0-9]+)", &line)
            {
                let x: usize = x.parse()?;
                let by: usize = by.parse()?;
                let mut column: Vec<Pixel> = (0..6).map(|y| display[y][x]).collect();
                column.rotate_right(by % 6);
                for y in 0..6 {
                    display[y][x] = column[y];
                }
            }
        }
        let lit = display
            .iter()
            .flatten()
            .filter(|pixel| **pixel == Pixel::On)
            .count();
        println!("{lit} pixels lit");
        for y in 0..6 {
            for x in 0..50 {
                print!(
                    "{}",
                    if display[y][x] == Pixel::On {
                        '█'
                    } else {
                        ' '
                    }
                );
            }
            println!("");
        }
        Ok(())
    }
}
