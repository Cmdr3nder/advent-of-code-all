use anyhow::{Context, Result};
use itertools::Itertools;
use lazy_regex::regex_captures;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

use crate::day::Day;
use crate::util::input::get_input;
use crate::util::ordered_vec::OrderedVec;

#[derive(Clone, Copy)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl Point {
    pub fn abs_distance(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

type PointID = usize;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Distance {
    distance: u64,
    a: PointID,
    b: PointID,
}

type Circuit = Vec<PointID>;

type CircuitID = usize;

pub struct Day08;

impl Day for Day08 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2025, 8)?);

        // Parse Points & Measure Distances
        let mut points: Vec<Point> = Vec::new();
        let mut distances: OrderedVec<Distance> = OrderedVec::new();
        for (line_num, line) in input.lines().map(|l| l.unwrap()).enumerate() {
            let (_, x_str, y_str, z_str) = regex_captures!(r"([0-9]+),([0-9]+),([0-9]+)", &line)
                .with_context(|| format!("No point on line {line_num}"))?;
            let point = Point {
                x: x_str.parse()?,
                y: y_str.parse()?,
                z: z_str.parse()?,
            };
            let point_id: PointID = points.len();
            for (pid, p) in points.iter().enumerate() {
                let distance = Distance {
                    distance: point.abs_distance(p),
                    a: point_id,
                    b: pid,
                };
                distances.insert(distance);
                if distances.len() > 1000 {
                    distances.pop();
                }
            }
            points.push(point);
        }

        // Build Circuits
        let mut circuits: Vec<Circuit> = Vec::new();
        let mut reusable_circuits: Vec<CircuitID> = Vec::new();
        let mut point_to_circuit: HashMap<PointID, CircuitID> = HashMap::new();
        for distance in distances.into_iter().take(1000) {
            let mut extend_map: Vec<(PointID, CircuitID)> = Vec::new();
            match (
                point_to_circuit.get(&distance.a),
                point_to_circuit.get(&distance.b),
            ) {
                (None, None) => {
                    let circuit_id: CircuitID =
                        if let Some(reusable_circuit_id) = reusable_circuits.pop() {
                            reusable_circuit_id
                        } else {
                            circuits.push(Vec::new());
                            circuits.len() - 1
                        };
                    circuits[circuit_id].push(distance.a);
                    circuits[circuit_id].push(distance.b);
                    extend_map.push((distance.a, circuit_id));
                    extend_map.push((distance.b, circuit_id));
                }
                (Some(circuit_id), None) => {
                    circuits[*circuit_id].push(distance.b);
                    point_to_circuit.insert(distance.b, *circuit_id);
                }
                (None, Some(circuit_id)) => {
                    circuits[*circuit_id].push(distance.a);
                    point_to_circuit.insert(distance.a, *circuit_id);
                }
                (Some(circuit_a_id), Some(circuit_b_id)) => {
                    if circuit_a_id != circuit_b_id {
                        // Move Items from circuit_b to circuit_a
                        let mut circuit_b = circuits[*circuit_b_id].clone();
                        circuits[*circuit_a_id].append(&mut circuit_b);
                        circuits[*circuit_b_id].clear();
                        // Extends point_to_circuit map with updated id
                        extend_map = circuits[*circuit_a_id]
                            .iter()
                            .map(|point_id| (*point_id, *circuit_a_id))
                            .collect();
                        // Free circuit_b for reuse
                        reusable_circuits.push(*circuit_b_id);
                    }
                }
            }
            if extend_map.len() > 0 {
                point_to_circuit.extend(extend_map.iter().map(|x| *x));
            }
        }

        let largest_product = circuits
            .iter()
            .map(|c| c.len())
            .sorted_unstable()
            .rev()
            .take(3)
            .reduce(|a, b| a * b)
            .with_context(|| "No circuits?!")?;

        println!("Product of largest 3 circuits, {largest_product}");

        Ok(())
    }
}
