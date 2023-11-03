use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{bail, Result};

use crate::day::Day;

pub struct Day02;

fn format_code(code: &[u8]) -> String {
    code.iter().map(|key| format!("{key:X}")).collect()
}

impl Day for Day02 {
    fn main() -> Result<()> {
        let input = BufReader::new(File::open("input/2016/day02.txt")?);
        let mut key: u8 = 5;
        let mut code: Vec<u8> = Vec::new();
        let mut key_actual: u8 = 5;
        let mut code_actual: Vec<u8> = Vec::new();
        for line in input.lines().map(|l| l.unwrap()) {
            for ch in line.trim().chars() {
                key = match (ch, key) {
                    ('U', 1) => 1,
                    ('U', 2) => 2,
                    ('U', 3) => 3,
                    ('U', 4) => 1,
                    ('U', 5) => 2,
                    ('U', 6) => 3,
                    ('U', 7) => 4,
                    ('U', 8) => 5,
                    ('U', 9) => 6,
                    ('R', 1) => 2,
                    ('R', 2) => 3,
                    ('R', 3) => 3,
                    ('R', 4) => 5,
                    ('R', 5) => 6,
                    ('R', 6) => 6,
                    ('R', 7) => 8,
                    ('R', 8) => 9,
                    ('R', 9) => 9,
                    ('D', 1) => 4,
                    ('D', 2) => 5,
                    ('D', 3) => 6,
                    ('D', 4) => 7,
                    ('D', 5) => 8,
                    ('D', 6) => 9,
                    ('D', 7) => 7,
                    ('D', 8) => 8,
                    ('D', 9) => 9,
                    ('L', 1) => 1,
                    ('L', 2) => 1,
                    ('L', 3) => 2,
                    ('L', 4) => 4,
                    ('L', 5) => 4,
                    ('L', 6) => 5,
                    ('L', 7) => 7,
                    ('L', 8) => 7,
                    ('L', 9) => 8,
                    _ => bail!("Unrecognized expected combination ({ch}, {key:X})"),
                };
                key_actual = match (ch, key_actual) {
                    ('U', 0x1) => 0x1,
                    ('U', 0x2) => 0x2,
                    ('U', 0x3) => 0x1,
                    ('U', 0x4) => 0x4,
                    ('U', 0x5) => 0x5,
                    ('U', 0x6) => 0x2,
                    ('U', 0x7) => 0x3,
                    ('U', 0x8) => 0x4,
                    ('U', 0x9) => 0x9,
                    ('U', 0xA) => 0x6,
                    ('U', 0xB) => 0x7,
                    ('U', 0xC) => 0x8,
                    ('U', 0xD) => 0xB,
                    ('R', 0x1) => 0x1,
                    ('R', 0x2) => 0x3,
                    ('R', 0x3) => 0x4,
                    ('R', 0x4) => 0x4,
                    ('R', 0x5) => 0x6,
                    ('R', 0x6) => 0x7,
                    ('R', 0x7) => 0x8,
                    ('R', 0x8) => 0x9,
                    ('R', 0x9) => 0x9,
                    ('R', 0xA) => 0xB,
                    ('R', 0xB) => 0xC,
                    ('R', 0xC) => 0xC,
                    ('R', 0xD) => 0xD,
                    ('D', 0x1) => 0x3,
                    ('D', 0x2) => 0x6,
                    ('D', 0x3) => 0x7,
                    ('D', 0x4) => 0x8,
                    ('D', 0x5) => 0x5,
                    ('D', 0x6) => 0xA,
                    ('D', 0x7) => 0xB,
                    ('D', 0x8) => 0xC,
                    ('D', 0x9) => 0x9,
                    ('D', 0xA) => 0xA,
                    ('D', 0xB) => 0xD,
                    ('D', 0xC) => 0xC,
                    ('D', 0xD) => 0xD,
                    ('L', 0x1) => 0x1,
                    ('L', 0x2) => 0x2,
                    ('L', 0x3) => 0x2,
                    ('L', 0x4) => 0x3,
                    ('L', 0x5) => 0x5,
                    ('L', 0x6) => 0x5,
                    ('L', 0x7) => 0x6,
                    ('L', 0x8) => 0x7,
                    ('L', 0x9) => 0x8,
                    ('L', 0xA) => 0xA,
                    ('L', 0xB) => 0xA,
                    ('L', 0xC) => 0xB,
                    ('L', 0xD) => 0xD,
                    _ => bail!("Unrecognized actual combination ({ch}, {key_actual:X})"),
                };
            }
            code.push(key);
            code_actual.push(key_actual);
        }
        println!("Bathroom code for expected keypad: {}", format_code(&code));
        println!(
            "Bathroom code for actual keypad: {}",
            format_code(&code_actual)
        );
        Ok(())
    }
}
