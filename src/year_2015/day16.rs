use anyhow::{bail, Context, Result};
use lazy_regex::regex_captures;

use crate::day::Day;
use crate::input::get_input;

pub struct Day16;

#[derive(Copy, Clone, Debug, Default)]
struct Aunt {
    num: i32,
    akitas: Option<i32>,
    cars: Option<i32>,
    cats: Option<i32>,
    children: Option<i32>,
    goldfish: Option<i32>,
    samoyeds: Option<i32>,
    trees: Option<i32>,
    vizslas: Option<i32>,
    pomeranians: Option<i32>,
    perfumes: Option<i32>,
}

fn eq(expected: Option<i32>, actual: Option<i32>) -> bool {
    actual == None || actual == expected
}

fn gt(expected: Option<i32>, actual: Option<i32>) -> bool {
    actual == None || actual > expected
}

fn lt(expected: Option<i32>, actual: Option<i32>) -> bool {
    actual == None || actual < expected
}

fn match_simple(expected: Aunt, actual: Aunt) -> bool {
    eq(expected.akitas, actual.akitas)
        && eq(expected.cars, actual.cars)
        && eq(expected.cats, actual.cats)
        && eq(expected.children, actual.children)
        && eq(expected.goldfish, actual.goldfish)
        && eq(expected.samoyeds, actual.samoyeds)
        && eq(expected.trees, actual.trees)
        && eq(expected.vizslas, actual.vizslas)
        && eq(expected.pomeranians, actual.pomeranians)
        && eq(expected.perfumes, actual.perfumes)
}

fn match_complex(expected: Aunt, actual: Aunt) -> bool {
    eq(expected.akitas, actual.akitas)
        && eq(expected.cars, actual.cars)
        && gt(expected.cats, actual.cats)
        && eq(expected.children, actual.children)
        && lt(expected.goldfish, actual.goldfish)
        && eq(expected.samoyeds, actual.samoyeds)
        && gt(expected.trees, actual.trees)
        && eq(expected.vizslas, actual.vizslas)
        && lt(expected.pomeranians, actual.pomeranians)
        && eq(expected.perfumes, actual.perfumes)
}

impl Day for Day16 {
    fn main() -> Result<()> {
        let expected = Aunt {
            num: -1,
            akitas: Some(0),
            cars: Some(2),
            cats: Some(7),
            children: Some(3),
            goldfish: Some(5),
            perfumes: Some(1),
            pomeranians: Some(3),
            samoyeds: Some(2),
            trees: Some(3),
            vizslas: Some(0),
        };
        let input_str = get_input(2015, 16)?;
        for line in input_str.lines() {
            let (_, num, stats) = regex_captures!("^Sue ([0-9]+): (.*)$", &line,)
                .with_context(|| "Could not parse Sue")?;
            let mut aunt = Aunt::default();
            aunt.num = num.parse()?;
            for stat in stats.split(", ") {
                let pair: Vec<_> = stat.split(": ").collect();
                if pair.len() != 2 {
                    bail!("Expected '{stat}' to contain name and count");
                }
                let value: Option<i32> = Some(pair[1].parse()?);
                match pair[0] {
                    "akitas" => aunt.akitas = value,
                    "cars" => aunt.cars = value,
                    "cats" => aunt.cats = value,
                    "children" => aunt.children = value,
                    "goldfish" => aunt.goldfish = value,
                    "samoyeds" => aunt.samoyeds = value,
                    "trees" => aunt.trees = value,
                    "vizslas" => aunt.vizslas = value,
                    "pomeranians" => aunt.pomeranians = value,
                    "perfumes" => aunt.perfumes = value,
                    x => bail!("Unexpected stat name '{x}'"),
                }
            }
            if match_simple(expected, aunt) {
                println!("Aunt Sue #{}, is a simple match", aunt.num);
            }
            if match_complex(expected, aunt) {
                println!("Aunt Sue #{}, is a complex match", aunt.num);
            }
        }
        Ok(())
    }
}
