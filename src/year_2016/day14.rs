use std::fs;

use anyhow::Result;
use md5;

use crate::day::Day;

fn calculate_md5_hash<'a>(
    cache: &'a mut Vec<String>,
    index: usize,
    salt: &str,
    iterations: usize,
) -> &'a str {
    while index >= cache.len() {
        let i = cache.len();
        let mut hash = format!("{salt}{i}");
        for _ in 0..iterations {
            hash = format!("{:x}", md5::compute(hash));
        }
        cache.push(hash);
    }
    &cache[index]
}

fn get_triple_char(s: &str) -> Option<char> {
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        let mut count = 1;
        while let Some(next) = chars.peek() {
            if *next == ch {
                count += 1;
                chars.next();
            } else {
                break;
            }
        }
        if count >= 3 {
            return Some(ch);
        }
    }
    None
}

fn check_five(s: &str, ch: char) -> bool {
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == ch {
            let mut count = 1;
            while let Some(next) = chars.peek() {
                if *next == ch {
                    count += 1;
                    chars.next();
                } else {
                    break;
                }
            }
            if count >= 5 {
                return true;
            }
        }
    }
    false
}

fn index_of_64th(salt: &str, iterations: usize) -> usize {
    let mut index = 0;
    let mut pad_key_count = 0;
    let mut hash_cache: Vec<String> = Vec::new();
    loop {
        let hash = calculate_md5_hash(&mut hash_cache, index, salt, iterations);
        if let Some(ch) = get_triple_char(hash) {
            let start = index + 1;
            for i in start..start + 1000 {
                let h = calculate_md5_hash(&mut hash_cache, i, salt, iterations);
                if check_five(h, ch) {
                    pad_key_count += 1;
                    break;
                }
            }
        }
        if pad_key_count == 64 {
            break;
        }
        index += 1;
    }
    index
}

pub struct Day14;

impl Day for Day14 {
    fn main() -> Result<()> {
        let input = fs::read_to_string("input/2016/day14.txt")?;
        let salt = input.trim();
        let index = index_of_64th(salt, 1);
        println!("Index {index} generates the 64th one-time pad key");
        let index = index_of_64th(salt, 2017);
        println!("Index {index} generates the 64th one-time pad key when using stretched hashing");
        Ok(())
    }
}
