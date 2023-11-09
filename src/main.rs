mod data;
mod day;
mod util;
mod year_2015;
mod year_2016;

use std::env;

use anyhow::Result;

use crate::day::Day;
use crate::year_2015::{
    Day01 as y2015_d01, Day02 as y2015_d02, Day03 as y2015_d03, Day04 as y2015_d04,
    Day05 as y2015_d05, Day06 as y2015_d06, Day07 as y2015_d07, Day08 as y2015_d08,
    Day09 as y2015_d09, Day10 as y2015_d10, Day11 as y2015_d11, Day12 as y2015_d12,
    Day13 as y2015_d13, Day14 as y2015_d14, Day15 as y2015_d15, Day16 as y2015_d16,
    Day17 as y2015_d17, Day18 as y2015_d18, Day19 as y2015_d19, Day20 as y2015_d20,
    Day21 as y2015_d21, Day22 as y2015_d22, Day23 as y2015_d23, Day24 as y2015_d24,
    Day25 as y2015_d25,
};
use crate::year_2016::{
    Day01 as y2016_d01, Day02 as y2016_d02, Day03 as y2016_d03, Day04 as y2016_d04,
    Day05 as y2016_d05, Day06 as y2016_d06, Day07 as y2016_d07, Day08 as y2016_d08,
    Day09 as y2016_d09, Day10 as y2016_d10,
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
            "2015-11" => y2015_d11::main()?,
            "2015-12" => y2015_d12::main()?,
            "2015-13" => y2015_d13::main()?,
            "2015-14" => y2015_d14::main()?,
            "2015-15" => y2015_d15::main()?,
            "2015-16" => y2015_d16::main()?,
            "2015-17" => y2015_d17::main()?,
            "2015-18" => y2015_d18::main()?,
            "2015-19" => y2015_d19::main()?,
            "2015-20" => y2015_d20::main()?,
            "2015-21" => y2015_d21::main()?,
            "2015-22" => y2015_d22::main()?,
            "2015-23" => y2015_d23::main()?,
            "2015-24" => y2015_d24::main()?,
            "2015-25" => y2015_d25::main()?,
            "2016-01" => y2016_d01::main()?,
            "2016-02" => y2016_d02::main()?,
            "2016-03" => y2016_d03::main()?,
            "2016-04" => y2016_d04::main()?,
            "2016-05" => y2016_d05::main()?,
            "2016-06" => y2016_d06::main()?,
            "2016-07" => y2016_d07::main()?,
            "2016-08" => y2016_d08::main()?,
            "2016-09" => y2016_d09::main()?,
            "2016-10" => y2016_d10::main()?,
            // "2016-11" => y2016_d11::main()?,
            // "2016-12" => y2016_d12::main()?,
            // "2016-13" => y2016_d13::main()?,
            // "2016-14" => y2016_d14::main()?,
            // "2016-15" => y2016_d15::main()?,
            // "2016-16" => y2016_d16::main()?,
            // "2016-17" => y2016_d17::main()?,
            // "2016-18" => y2016_d18::main()?,
            // "2016-19" => y2016_d19::main()?,
            // "2016-20" => y2016_d20::main()?,
            // "2016-21" => y2016_d21::main()?,
            // "2016-22" => y2016_d22::main()?,
            // "2016-23" => y2016_d23::main()?,
            // "2016-24" => y2016_d24::main()?,
            // "2016-25" => y2016_d25::main()?,
            "target/debug/advent-of-code-all" => {}
            x => println!("Unrecognized Argument: '{x}'"),
        };
    }
    Ok(())
}
