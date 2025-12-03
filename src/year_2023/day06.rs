use crate::util::input::get_input;
use std::io::{BufRead, BufReader};

use anyhow::{bail, Result};
use itertools::Itertools;

use crate::day::Day;

#[derive(Copy, Clone, Debug)]
struct Race {
    time: u64,
    champion_distance: u64,
}

impl Race {
    fn new(time: u64, champion_distance: u64) -> Self {
        Race {
            time,
            champion_distance,
        }
    }
}

fn find_ways(race: Race) -> u64 {
    let mut ways = 0;
    for hold_time in 0..=race.time {
        let travel_time = race.time - hold_time;
        let covered_distance = hold_time * travel_time;
        if covered_distance > race.champion_distance {
            ways += 1;
        } else if ways > 0 {
            // Once we start winning we are done if we start losing
            break;
        }
    }
    ways
}

pub struct Day06;

impl Day for Day06 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2023, 06)?);
        let races: Vec<Race> = {
            let mut lines = input.lines().map(|l| l.unwrap());
            let times = if let Some(line) = lines.next() {
                if !line.starts_with("Time:") {
                    bail!("Expected list of times");
                }
                let mut times: Vec<u64> = Vec::new();
                for part in line.split_whitespace().filter(|p| *p != "Time:") {
                    times.push(part.parse()?);
                }
                times
            } else {
                bail!("Expected another line");
            };
            let distances = if let Some(line) = lines.next() {
                if !line.starts_with("Distance:") {
                    bail!("Expected list of distances");
                }
                let mut distances: Vec<u64> = Vec::new();
                for part in line.split_whitespace().filter(|p| *p != "Distance:") {
                    distances.push(part.parse()?);
                }
                distances
            } else {
                bail!("Expected another line");
            };
            times
                .iter()
                .zip(distances.iter())
                .map(|(time, distance)| Race::new(*time, *distance))
                .collect()
        };
        let product_of_ways: u64 = races.iter().map(|race| find_ways(*race)).product();
        println!("Product of ways to win races: {product_of_ways}");
        let long_race = Race::new(
            races.iter().map(|r| r.time.to_string()).join("").parse()?,
            races
                .iter()
                .map(|r| r.champion_distance.to_string())
                .join("")
                .parse()?,
        );
        let long_race_ways: u64 = find_ways(long_race);
        println!("Ways to win the long race: {long_race_ways}");
        Ok(())
    }
}
