use crate::util::input::get_input;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};
use lazy_regex::regex_captures;

use crate::day::Day;

pub struct Day14;

struct Reindeer {
    fly_speed: u64,
    fly_time_seconds: u64,
    rest_time_seconds: u64,
}

impl Reindeer {
    fn distance_at_time(&self, seconds: u64) -> u64 {
        let full_cycle_seconds = self.fly_time_seconds + self.rest_time_seconds;
        let full_cycles = seconds / full_cycle_seconds;
        let remaining_seconds = seconds % full_cycle_seconds;
        let fly_time_seconds = if remaining_seconds > self.fly_time_seconds {
            self.fly_time_seconds
        } else {
            remaining_seconds
        };
        (full_cycles * (self.fly_time_seconds * self.fly_speed))
            + (fly_time_seconds * self.fly_speed)
    }
}

impl Day for Day14 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2015, 14)?);
        let mut reindeer: Vec<Reindeer> = Vec::with_capacity(10);
        let mut best_distance = u64::MIN;
        for line in input.lines().map(|l| l.unwrap()) {
            let (_, _name, fly_speed, fly_time_seconds, rest_time_seconds) = regex_captures!(
                "([A-Za-z]+) can fly ([0-9]+) km/s for ([0-9]+) seconds, but then must rest for ([0-9]+) seconds.",
                &line,
            ).with_context(|| "Could not match Reindeer speeds")?;
            let fly_speed: u64 = fly_speed.parse()?;
            let fly_time_seconds: u64 = fly_time_seconds.parse()?;
            let rest_time_seconds: u64 = rest_time_seconds.parse()?;
            let r = Reindeer {
                fly_speed,
                fly_time_seconds,
                rest_time_seconds,
            };
            let distance = r.distance_at_time(2503);
            if distance > best_distance {
                best_distance = distance;
            }
            reindeer.push(r);
        }
        println!("Winning distance: {best_distance}");
        let mut points: Vec<u64> = reindeer.iter().map(|_| 0).collect();
        for seconds in 1..=2503 {
            let mut leader_index: Vec<usize> = vec![0];
            let mut leader_distance = reindeer[0].distance_at_time(seconds);
            for index in 1..reindeer.len() {
                let distance = reindeer[index].distance_at_time(seconds);
                if distance > leader_distance {
                    leader_index = vec![index];
                    leader_distance = distance;
                } else if distance == leader_distance {
                    leader_index.push(index);
                }
            }
            for index in leader_index {
                points[index] += 1;
            }
        }
        println!("Winning points: {}", points.iter().max().unwrap());
        Ok(())
    }
}
