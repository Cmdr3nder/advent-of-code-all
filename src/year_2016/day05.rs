use std::fs;

use anyhow::{Context, Result};
use lazy_regex::{regex_captures, regex_is_match};
use md5;

use crate::day::Day;

pub struct Day05;

impl Day for Day05 {
    fn main() -> Result<()> {
        let input = fs::read_to_string("input/2016/day05.txt")?;
        let input = input.trim();
        let mut password: Vec<char> = Vec::new();
        let mut passcode: [Option<char>; 8] = [None; 8];
        for i in 0..usize::MAX {
            let hash = format!("{:x}", md5::compute(format!("{input}{i}")));
            if let Some((_, a, b)) = regex_captures!("^00000(.)(.)", &hash) {
                if password.len() < 8 {
                    password.push(a.chars().nth(0).with_context(|| "No character!")?);
                }
                if regex_is_match!("^[0-7]$", a) {
                    let pos: usize = a.parse()?;
                    if passcode[pos].is_none() {
                        passcode[pos] = Some(b.chars().nth(0).with_context(|| "No character!")?);
                    }
                }
                if password.len() == 8 && passcode.iter().all(|x| x.is_some()) {
                    break;
                }
                println!("{}", password.len());
            }
        }
        let password: String = password.iter().collect();
        println!("Password is {password}");
        let passcode: String = passcode.iter().map(|x| x.unwrap()).collect();
        println!("Passcode is {passcode}");
        Ok(())
    }
}
