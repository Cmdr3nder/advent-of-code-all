mod data;
mod day;
mod util;
mod year_2015;

use std::env;

use anyhow::Result;

use crate::day::Day;
use crate::year_2015::{
    Day01 as y2015_d01, Day02 as y2015_d02, Day03 as y2015_d03, Day04 as y2015_d04,
    Day05 as y2015_d05, Day06 as y2015_d06, Day07 as y2015_d07, Day08 as y2015_d08,
    Day09 as y2015_d09, Day10 as y2015_d10,
};

fn main() -> Result<()> {
    for argument in env::args() {
        match argument.as_str() {
            "2015-01" => y2015_d01::main()?,
            "2015-02" => y2015_d02::main()?,
            "2015-03" => y2015_d03::main()?,
            "2015-04" => y2015_d04::main()?,
            "2015-05" => y2015_d05::main()?,
            "2015-06" => y2015_d06::main()?,
            "2015-07" => y2015_d07::main()?,
            "2015-08" => y2015_d08::main()?,
            "2015-09" => y2015_d09::main()?,
            "2015-10" => y2015_d10::main()?,
            "target/debug/advent-of-code-all" => {}
            x => println!("Unrecognized Argument: '{x}'"),
        };
    }
    Ok(())
}
