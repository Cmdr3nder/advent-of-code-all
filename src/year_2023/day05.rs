use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

use anyhow::{bail, Context, Result};

use crate::day::Day;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum ProdMapName {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl ProdMapName {
    fn first() -> ProdMapName {
        ProdMapName::SeedToSoil
    }

    fn next(&self) -> Option<ProdMapName> {
        match *self {
            ProdMapName::SeedToSoil => Some(ProdMapName::SoilToFertilizer),
            ProdMapName::SoilToFertilizer => Some(ProdMapName::FertilizerToWater),
            ProdMapName::FertilizerToWater => Some(ProdMapName::WaterToLight),
            ProdMapName::WaterToLight => Some(ProdMapName::LightToTemperature),
            ProdMapName::LightToTemperature => Some(ProdMapName::TemperatureToHumidity),
            ProdMapName::TemperatureToHumidity => Some(ProdMapName::HumidityToLocation),
            ProdMapName::HumidityToLocation => None,
        }
    }

    fn expected_label(&self) -> &'static str {
        match *self {
            ProdMapName::SeedToSoil => "seed-to-soil map:",
            ProdMapName::SoilToFertilizer => "soil-to-fertilizer map:",
            ProdMapName::FertilizerToWater => "fertilizer-to-water map:",
            ProdMapName::WaterToLight => "water-to-light map:",
            ProdMapName::LightToTemperature => "light-to-temperature map:",
            ProdMapName::TemperatureToHumidity => "temperature-to-humidity map:",
            ProdMapName::HumidityToLocation => "humidity-to-location map:",
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Phase {
    Seeds,
    ProdMap(ProdMapName),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Overlap {
    Before,
    After,
    Inside,
}

#[derive(Copy, Clone, Debug)]
struct ProdRange {
    destination: u64,
    source: u64,
    length: u64,
}

fn range_len(range: &Range<u64>) -> u64 {
    range.end - range.start
}

impl ProdRange {
    fn parse(line: &str) -> Result<Self> {
        let mut parts = line.split(" ");
        let destination: u64 = parts
            .next()
            .with_context(|| format!("Expected a destination number '{line}'"))?
            .parse()
            .with_context(|| format!("Could not parse a destination number '{line}'"))?;
        let source: u64 = parts
            .next()
            .with_context(|| format!("Expected a source number '{line}'"))?
            .parse()
            .with_context(|| format!("Could not parse a source number '{line}'"))?;
        let length: u64 = parts
            .next()
            .with_context(|| format!("Expected a length number '{line}'"))?
            .parse()
            .with_context(|| format!("Could not parse a length number '{line}'"))?;
        if let Some(part) = parts.next() {
            bail!("Unexpected extra part in range, '{part}' in '{line}'")
        }
        Ok(ProdRange {
            destination,
            source,
            length,
        })
    }

    fn overlap(&self, val: u64) -> Overlap {
        if val >= self.source {
            if val < self.source + self.length {
                Overlap::Inside
            } else {
                Overlap::After
            }
        } else {
            Overlap::Before
        }
    }

    // Returns (updated ranges, preserved ranges)
    fn map(&self, range: Range<u64>) -> Result<(Vec<Range<u64>>, Vec<Range<u64>>)> {
        Ok(
            match (self.overlap(range.start), self.overlap(range.end - 1)) {
                (Overlap::Before, Overlap::Before) => (Vec::new(), vec![range]),
                (Overlap::After, Overlap::After) => (Vec::new(), vec![range]),
                (Overlap::Inside, Overlap::Inside) => {
                    let new_start = range.start - self.source + self.destination;
                    (vec![new_start..new_start + range_len(&range)], Vec::new())
                }
                (Overlap::Before, Overlap::After) => {
                    // split into 3 parts
                    let left = range.start..self.source;
                    let middle = self.destination..self.destination + self.length;
                    let right = self.source + self.length..range.end;
                    (vec![middle], vec![left, right])
                }
                (Overlap::Before, Overlap::Inside) => {
                    // split into 2 parts
                    let left = range.start..self.source;
                    let middle =
                        self.destination..self.destination + range_len(&range) - range_len(&left);
                    (vec![middle], vec![left])
                }
                (Overlap::Inside, Overlap::After) => {
                    // split into 2 parts
                    let middle_start = range.start - self.source + self.destination;
                    let middle_end = self.destination + self.length;
                    let middle = middle_start..middle_end;
                    let right = self.source + self.length..range.end;
                    (vec![middle], vec![right])
                }
                _ => bail!("Range  {range:?} makes no sense against {self:?}"),
            },
        )
    }
}

#[derive(Clone, Debug, Default)]
struct ProdMap {
    ranges: Vec<ProdRange>,
}

impl ProdMap {
    fn map(&self, val: Range<u64>) -> Result<Vec<Range<u64>>> {
        let mut processing: Vec<Range<u64>> = vec![val];
        let mut resolved: Vec<Range<u64>> = Vec::new();
        for range in &self.ranges {
            let mut next_processing = Vec::new();
            for proc in processing {
                let (mut done, mut reproc) = range.map(proc)?;
                next_processing.append(&mut reproc);
                resolved.append(&mut done);
            }
            processing = next_processing;
        }
        resolved.append(&mut processing);
        Ok(resolved)
    }

    fn push(&mut self, range: ProdRange) {
        self.ranges.push(range);
    }
}

#[derive(Clone, Debug, Default)]
struct ProdMaps {
    maps: HashMap<ProdMapName, ProdMap>,
}

impl ProdMaps {
    fn push(&mut self, map_name: ProdMapName, range: ProdRange) {
        let map = if let Some(m) = self.maps.get_mut(&map_name) {
            m
        } else {
            self.maps.insert(map_name, ProdMap::default());
            self.maps.get_mut(&map_name).unwrap() // Safe because we just inserted it
        };
        map.push(range);
    }

    fn map(&self, seeds: Range<u64>) -> Result<Vec<Range<u64>>> {
        let mut res = vec![seeds];
        let mut next = Some(ProdMapName::first());
        while let Some(map_name) = next {
            res = if let Some(map) = self.maps.get(&map_name) {
                let mut new_res = Vec::new();
                for seed_range in res {
                    new_res.append(&mut map.map(seed_range)?);
                }
                new_res
            } else {
                res
            };
            next = map_name.next();
        }
        Ok(res)
    }
}

pub struct Day05;

impl Day for Day05 {
    fn main() -> Result<()> {
        let input = BufReader::new(File::open("input/2023/day05.txt")?);
        let mut seeds: Vec<u64> = Vec::new();
        let mut phase: Option<Phase> = Some(Phase::Seeds);
        let mut maps = ProdMaps::default();
        for line in input.lines().map(|l| l.unwrap()) {
            if let Some(p) = phase {
                match p {
                    Phase::Seeds => {
                        if line.starts_with("seeds: ") {
                            for seed_str in line.split(" ").filter(|seg| *seg != "seeds:") {
                                seeds.push(seed_str.parse().with_context(|| {
                                    format!("Expected to parse seed from '{seed_str}'")
                                })?);
                            }
                        } else if line.trim().is_empty() {
                            phase = Some(Phase::ProdMap(ProdMapName::first()));
                        } else {
                            bail!("Unexpected line, expecting seeds");
                        }
                    }
                    Phase::ProdMap(map_name) => {
                        if line.trim().is_empty() {
                            phase = map_name.next().map(|mn| Phase::ProdMap(mn));
                        } else if !line.starts_with(map_name.expected_label()) {
                            maps.push(map_name, ProdRange::parse(&line)?);
                        }
                    }
                }
            } else {
                bail!("Unexpected extra line '{line}'");
            }
        }
        let mut lowest_soil: u64 = u64::MAX;
        for seed in &seeds {
            for res in maps.map(*seed..seed + 1)? {
                if res.start < lowest_soil {
                    lowest_soil = res.start;
                }
            }
        }
        println!("Closest soil block to start at is: {lowest_soil}");
        let mut expanded_seeds = seeds.iter();
        let mut lowest_soil: u64 = u64::MAX;
        while let (Some(start), Some(len)) = (expanded_seeds.next(), expanded_seeds.next()) {
            for res in maps.map(*start..start + len)? {
                if res.start < lowest_soil {
                    lowest_soil = res.start;
                }
            }
        }
        println!("Closest soil block using expanded seeds is: {lowest_soil}");
        Ok(())
    }
}
