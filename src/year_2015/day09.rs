use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{bail, Context, Result};
use lazy_regex::regex_captures;

use crate::data::StringIdMap;
use crate::day::Day;
use crate::util::peek::Peek;

pub struct Day09;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Route {
    a: usize,
    b: usize,
}

impl Route {
    const fn new(a: usize, b: usize) -> Self {
        if a < b {
            Route { a, b }
        } else {
            Route { a: b, b: a }
        }
    }
}

fn to_route(destinations: &mut StringIdMap, a: &str, b: &str) -> Route {
    let a = destinations.to_id(a);
    let b = destinations.to_id(b);
    Route::new(a, b)
}

impl Day for Day09 {
    fn main() -> Result<()> {
        let input = BufReader::new(File::open("input/2015/day09.txt")?);
        let mut destinations = StringIdMap::default();
        let mut edges: HashMap<Route, usize> = HashMap::new();
        let mut connections: HashMap<usize, Vec<usize>> = HashMap::new();
        for line in input.lines().map(|l| l.unwrap()) {
            let (_, a, b, length) = regex_captures!("(.+) to (.+) = ([0-9]+)", &line)
                .with_context(|| format!("Failed to match line regex {line}"))?;
            let length: usize = length.parse()?;
            let route = to_route(&mut destinations, a, b);
            let a = destinations.to_id(a);
            let b = destinations.to_id(b);
            if let Some(_) = edges.insert(route, length) {
                bail!("Unexpected duplicate route {route:?}");
            }
            match connections.get_mut(&a) {
                Some(conn) => {
                    conn.push(b);
                }
                None => {
                    connections.insert(a, vec![b]);
                }
            }
            match connections.get_mut(&b) {
                Some(conn) => {
                    conn.push(a);
                }
                None => {
                    connections.insert(b, vec![a]);
                }
            }
        }
        let mut paths: Vec<(Vec<usize>, usize)> = Vec::new();
        for destination_id in 0..destinations.count() {
            paths.push((vec![destination_id], 0))
        }
        let mut shortest = usize::MAX;
        let mut longest = usize::MIN;
        while let Some((path, cost_so_far)) = paths.pop() {
            let end = path
                .peek()
                .with_context(|| format!("Path should never be empty"))?;
            if path.len() == destinations.count() {
                if cost_so_far < shortest {
                    shortest = cost_so_far;
                }
                if cost_so_far > longest {
                    longest = cost_so_far;
                }
            }
            for conn in connections
                .get(end)
                .with_context(|| format!("Connections should always exist"))?
            {
                if !path.contains(conn) {
                    let mut new_path = path.clone();
                    new_path.push(*conn);
                    let route = Route::new(*end, *conn);
                    let cost_so_far = cost_so_far
                        + edges
                            .get(&route)
                            .with_context(|| format!("Edge should always exist"))?;
                    paths.push((new_path, cost_so_far));
                }
            }
        }
        println!("Shortest route: {shortest}");
        println!("Longest route: {longest}");
        Ok(())
    }
}
