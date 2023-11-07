use std::fs;
use std::iter::Peekable;

use anyhow::{bail, Context, Result};

use crate::day::Day;

pub struct Day09;

fn read_usize<I>(chars: &mut Peekable<I>) -> usize
where
    I: Iterator<Item = char>,
{
    let mut num: usize = 0;
    while let Some(ch) = chars.peek() {
        match ch {
            '0' => {}
            '1' => {
                num += 1;
            }
            '2' => {
                num += 2;
            }
            '3' => {
                num += 3;
            }
            '4' => {
                num += 4;
            }
            '5' => {
                num += 5;
            }
            '6' => {
                num += 6;
            }
            '7' => {
                num += 7;
            }
            '8' => {
                num += 8;
            }
            '9' => {
                num += 9;
            }
            _ => break,
        }
        chars.next();
        num *= 10;
    }
    num / 10
}

fn decompress_len(str: &str) -> Result<usize> {
    let mut char_count = 0;
    let mut chars = str.chars().peekable();
    while let Some(ch) = chars.peek() {
        if *ch == '(' {
            // Start reading compress sequence
            chars.next();
            let count = read_usize(&mut chars);
            if Some('x') != chars.next() {
                bail!("Expected 'x' after compress count");
            }
            let times = read_usize(&mut chars);
            if Some(')') != chars.next() {
                bail!("Expected ')' after compress times");
            }
            char_count += count * times;
            for _ in 0..count {
                chars.next().with_context(|| "Expected a char to cache")?;
            }
        } else {
            chars.next();
            char_count += 1;
        }
    }
    Ok(char_count)
}

fn full_decompress_len(str: &str) -> Result<usize> {
    let mut char_count = 0;
    let mut chars = str.chars().peekable();
    while let Some(ch) = chars.peek() {
        if *ch == '(' {
            // Start reading compress sequence
            chars.next();
            let count = read_usize(&mut chars);
            if Some('x') != chars.next() {
                bail!("Expected 'x' after compress count");
            }
            let times = read_usize(&mut chars);
            if Some(')') != chars.next() {
                bail!("Expected ')' after compress times");
            }
            let mut cache: Vec<char> = Vec::with_capacity(count);
            for _ in 0..count {
                let ch = chars.next().with_context(|| "Expected a char to cache")?;
                cache.push(ch);
            }
            let cache: String = cache.iter().collect();
            let count = full_decompress_len(&cache)?;
            char_count += count * times;
        } else {
            chars.next();
            char_count += 1;
        }
    }
    Ok(char_count)
}

impl Day for Day09 {
    fn main() -> Result<()> {
        let input = fs::read_to_string("input/2016/day09.txt")?;
        let input = input.trim();
        println!("{} chars in decompressed output", decompress_len(input)?);
        println!(
            "{} chars in fully decompressed output",
            full_decompress_len(input)?
        );
        Ok(())
    }
}
