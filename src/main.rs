mod day;
mod year_2015;

use std::env;

use anyhow::Result;

use crate::day::Day;
use crate::year_2015::{Day01 as y2015_d1, Day02 as y2015_d2};

fn main() -> Result<()> {
    for argument in env::args() {
        match argument.as_str() {
            "2015-01" => y2015_d1::main()?,
            "2015-02" => y2015_d2::main()?,
            "target/debug/advent-of-code-all" => {}
            x => println!("Unrecognized Argument: '{x}'"),
        };
    }
    Ok(())
}
