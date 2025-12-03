use std::env;
use std::fs;
use std::path;

use anyhow::{Context, Result};

pub fn get_input_path(year: u16, day: u8) -> Result<path::PathBuf> {
    let mut p = env::home_dir().with_context(|| format!("Could not get $HOME"))?;
    let s = format!(".cache/advent-of-code-all/{:0>4}/day{:0>2}.txt", year, day);
    p.push(&s);
    fs::canonicalize(p).with_context(|| format!("Could not canonicalize '$HOME/{}'", &s))
}

pub fn get_input_string(year: u16, day: u8) -> Result<String> {
    let p = get_input_path(year, day)?;
    fs::read_to_string(&p).with_context(|| {
        format!(
            "Expected an input file, but something went wrong '{}'",
            p.to_str().unwrap_or("bad-path")
        )
    })
}

pub fn get_input(year: u16, day: u8) -> Result<fs::File> {
    let p = get_input_path(year, day)?;
    fs::File::open(&p).with_context(|| {
        format!(
            "Expected an input file, but something went wrong '{}'",
            p.to_str().unwrap_or("bad-path")
        )
    })
}
