use std::fs;
use std::path;

use anyhow::{Context, Result};

pub fn get_input_path(year: u16, day: u8) -> path::PathBuf {
    path::PathBuf::from(format!(
        "~/.cache/advent-of-code-all/{}/day{}.txt",
        year, day
    ))
}

pub fn get_input_string(year: u16, day: u8) -> Result<String> {
    let p = get_input_path(year, day);
    fs::read_to_string(p).with_context(|| "Expected an input file, but something went wrong")
}

pub fn get_input(year: u16, day: u8) -> Result<fs::File> {
    let p = get_input_path(year, day);
    fs::File::open(p).with_context(|| "Expected an input file, but something went wrong")
}
