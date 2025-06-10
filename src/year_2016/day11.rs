use std::cmp::Ordering;
use std::collections::HashSet;

use anyhow::{bail, Result};
use lazy_regex::{regex, regex_captures};

use crate::data::StringIdMap;
use crate::day::Day;
use crate::input::get_input;
use crate::util::expand::expand;
use crate::util::ordered_vec::OrderedVec;
use crate::util::priority_queue::PriorityQueue;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Device {
    Generator(usize),
    Microchip(usize),
}

impl Ord for Device {
    fn cmp(&self, other: &Self) -> Ordering {
        match (*self, *other) {
            (Device::Generator(x), Device::Generator(y)) => x.cmp(&y),
            (Device::Microchip(x), Device::Microchip(y)) => x.cmp(&y),
            (Device::Generator(x), Device::Microchip(y)) => match x.cmp(&y) {
                Ordering::Equal => Ordering::Greater,
                z => z,
            },
            (Device::Microchip(x), Device::Generator(y)) => match x.cmp(&y) {
                Ordering::Equal => Ordering::Less,
                z => z,
            },
        }
    }
}

impl PartialOrd for Device {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Device {
    fn is_pair(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Device::Generator(x), Device::Microchip(y)) => x == y,
            (Device::Microchip(x), Device::Generator(y)) => x == y,
            _ => false,
        }
    }

    fn normalize(&self) -> NormalizedDevice {
        match self {
            Device::Generator(_) => NormalizedDevice::Generator,
            Device::Microchip(_) => NormalizedDevice::Microchip,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
enum NormalizedDevice {
    Generator,
    Microchip,
    Pair,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct State {
    floors: Vec<OrderedVec<Device>>,
    elevator_floor: usize,
}

#[derive(Eq, Hash, PartialEq)]
struct NormalizedState {
    floors: Vec<OrderedVec<NormalizedDevice>>,
    elevator_floor: usize,
}

impl State {
    fn complete(&self) -> bool {
        let last_floor = self.floors.len() - 1;
        for i in 0..last_floor {
            if !self.floors[i].is_empty() {
                return false;
            }
        }
        self.elevator_floor == last_floor
    }

    fn valid(&self) -> bool {
        for i in 0..self.floors.len() {
            match self.floors[i].len() {
                0 => continue,
                1 => continue,
                len => {
                    let mut has_generator = false;
                    let mut has_unpaired_microchip = false;
                    for j in 0..len {
                        match self.floors[i][j] {
                            Device::Generator(_) => {
                                has_generator = true;
                            }
                            Device::Microchip(_) => {
                                has_unpaired_microchip = if has_unpaired_microchip || j >= len - 1 {
                                    true
                                } else {
                                    !self.floors[i][j].is_pair(&self.floors[i][j + 1])
                                };
                            }
                        }
                    }
                    if has_generator && has_unpaired_microchip {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn move_each(&self) -> Vec<Self> {
        let mut new_states = Vec::new();
        let floor_len = self.floors[self.elevator_floor].len();

        if self.elevator_floor < self.floors.len() - 1 {
            let upper = self.elevator_floor + 1;

            // Try Moving Pairs Up
            let mut pairs_moved = 0;
            for i in 0..floor_len - 1 {
                for j in i + 1..floor_len {
                    let mut new_state = self.clone();
                    let b = new_state.floors[new_state.elevator_floor].remove(j);
                    let a = new_state.floors[new_state.elevator_floor].remove(i);
                    new_state.floors[upper].push(a);
                    new_state.floors[upper].push(b);
                    new_state.elevator_floor = upper;
                    if new_state.valid() {
                        new_states.push(new_state);
                        pairs_moved += 1;
                    }
                }
            }

            if pairs_moved == 0 {
                // Try Moving Singletons Up
                for i in 0..floor_len {
                    let mut new_state = self.clone();
                    let moved = new_state.floors[new_state.elevator_floor].remove(i);
                    new_state.floors[upper].push(moved);
                    new_state.elevator_floor = upper;
                    if new_state.valid() {
                        new_states.push(new_state);
                    }
                }
            }
        }

        if self.elevator_floor > 0 {
            let lower = self.elevator_floor - 1;
            let mut is_empty = true;
            for i in 0..=lower {
                if !self.floors[i].is_empty() {
                    is_empty = false;
                    break;
                }
            }
            if !is_empty {
                // Try Moving Singletons Down
                let mut singletons_moved = 0;
                for i in 0..floor_len {
                    let mut new_state = self.clone();
                    let moved = new_state.floors[new_state.elevator_floor].remove(i);
                    new_state.floors[lower].push(moved);
                    new_state.elevator_floor = lower;
                    if new_state.valid() {
                        new_states.push(new_state);
                        singletons_moved += 1;
                    }
                }

                if singletons_moved == 0 {
                    // Try Moving Pairs Down
                    for i in 0..floor_len - 1 {
                        for j in i..floor_len {
                            let mut new_state = self.clone();
                            let b = new_state.floors[new_state.elevator_floor].remove(j);
                            let a = new_state.floors[new_state.elevator_floor].remove(i);
                            new_state.floors[lower].push(a);
                            new_state.floors[lower].push(b);
                            new_state.elevator_floor = lower;
                            if new_state.valid() {
                                new_states.push(new_state);
                            }
                        }
                    }
                }
            }
        }

        new_states
    }

    fn closeness(&self) -> usize {
        let mut empty_floor_count = 0;
        for i in 0..self.floors.len() {
            if self.floors[i].is_empty() {
                empty_floor_count += 1;
            } else {
                break;
            }
        }
        let floor_score: usize = self
            .floors
            .iter()
            .enumerate()
            .map(|(i, f)| ((i + 1) * 2 * f.len()))
            .sum();
        empty_floor_count * 10000 + floor_score
    }

    fn normalize(&self) -> NormalizedState {
        let floors: Vec<OrderedVec<NormalizedDevice>> = self
            .floors
            .iter()
            .map(|f| normalize_devices(f.as_slice()))
            .collect();
        NormalizedState {
            floors,
            elevator_floor: self.elevator_floor,
        }
    }
}

fn normalize_devices(devices: &[Device]) -> OrderedVec<NormalizedDevice> {
    match devices.len() {
        0 => OrderedVec::new(),
        1 => OrderedVec::from_one(devices[0].normalize()),
        len => {
            // 2+
            let mut res = OrderedVec::new();
            let mut i = 0;
            while i < len - 1 {
                if devices[i].is_pair(&devices[i + 1]) {
                    i += 2;
                    res.push(NormalizedDevice::Pair);
                } else {
                    i += 1;
                    res.push(devices[i].normalize());
                }
            }
            if i < devices.len() {
                res.push(devices[i].normalize());
            }
            res
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct StatePriority {
    closeness: usize,
    steps: usize,
}

impl Ord for StatePriority {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.steps == other.steps {
            if self.closeness == other.closeness {
                Ordering::Equal
            } else if self.closeness < other.closeness {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else if self.steps < other.steps {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for StatePriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn read_input() -> Result<(State, StringIdMap)> {
    let reg_microchip = regex!("([a-z]+)-compatible microchip");
    let reg_generator = regex!("([a-z]+) generator");
    let input = get_input(2016, 11)?;
    let mut keywords = StringIdMap::default();
    let mut state = State::default();
    for line in input.lines() {
        if let Some((_, floor)) = regex_captures!("(first|second|third|fourth) floor", &line) {
            let floor = match floor {
                "first" => 0,
                "second" => 1,
                "third" => 2,
                "fourth" => 3,
                _ => bail!("Unexpected floor '{floor}'"),
            };
            expand(&mut state.floors, floor);
            for caps in reg_microchip.captures_iter(&line) {
                let id = keywords.to_id(&caps[1]);
                state.floors[floor].push(Device::Microchip(id));
            }
            for caps in reg_generator.captures_iter(&line) {
                let id = keywords.to_id(&caps[1]);
                state.floors[floor].push(Device::Generator(id));
            }
        }
    }
    Ok((state, keywords))
}

fn minimum_steps(initial_state: &State) -> usize {
    let mut seen: HashSet<NormalizedState> = HashSet::new();
    let mut states = PriorityQueue::new();
    states.push(initial_state.clone(), StatePriority::default());
    while let Some((state, priority)) = states.pop() {
        let normalized = state.normalize();
        if seen.contains(&normalized) {
            // Don't reprocess the same state again
            continue;
        } else {
            seen.insert(normalized);
        }
        if state.complete() {
            return priority.steps;
        }
        for new_state in state.move_each() {
            let p = StatePriority {
                steps: priority.steps + 1,
                closeness: new_state.closeness(),
            };
            states.push(new_state, p);
        }
        if seen.len() % 10000 == 0 {
            println!("Processing... {} / {}", states.len(), seen.len());
        }
    }
    return usize::MAX;
}

pub struct Day11;

impl Day for Day11 {
    fn main() -> Result<()> {
        let (mut initial_state, mut keywords) = read_input()?;
        let steps = minimum_steps(&initial_state);
        println!("It took {steps} steps to collect all objects safely");
        let elerium = keywords.to_id("elerium");
        let dilithium = keywords.to_id("dilithium");
        initial_state.floors[0].push(Device::Generator(elerium));
        initial_state.floors[0].push(Device::Microchip(elerium));
        initial_state.floors[0].push(Device::Generator(dilithium));
        initial_state.floors[0].push(Device::Microchip(dilithium));
        let steps = minimum_steps(&initial_state);
        println!("It took {steps} steps to collect additional objects safely");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn device_ordering() {
        let data = vec![
            (
                Device::Generator(1),
                Device::Microchip(1),
                Ordering::Greater,
            ),
            (Device::Microchip(1), Device::Generator(1), Ordering::Less),
            (Device::Generator(1), Device::Generator(2), Ordering::Less),
            (
                Device::Generator(2),
                Device::Generator(1),
                Ordering::Greater,
            ),
            (Device::Generator(1), Device::Generator(1), Ordering::Equal),
            (Device::Microchip(1), Device::Microchip(2), Ordering::Less),
            (
                Device::Microchip(2),
                Device::Microchip(1),
                Ordering::Greater,
            ),
            (Device::Microchip(1), Device::Microchip(1), Ordering::Equal),
        ];
        for (left, right, expected) in data {
            assert_eq!(
                left.cmp(&right),
                expected,
                "Expected {left:?} {expected:?} {right:?}"
            );
        }
    }

    #[test]
    fn device_pairing() {
        let data = vec![
            (Device::Generator(1), Device::Microchip(1), true),
            (Device::Generator(2), Device::Microchip(1), false),
            (Device::Microchip(1), Device::Generator(1), true),
            (Device::Microchip(2), Device::Generator(1), false),
        ];
        for (left, right, expected) in data {
            assert_eq!(
                left.is_pair(&right),
                expected,
                "Expected{} a pair {left:?}, {right:?}",
                if !expected { " not" } else { "" }
            );
        }
    }

    #[test]
    fn device_normalization() {
        let data = vec![
            (Device::Generator(1), NormalizedDevice::Generator),
            (Device::Microchip(1), NormalizedDevice::Microchip),
        ];

        for (given, expected) in data {
            assert_eq!(given.normalize(), expected);
        }
    }

    #[test]
    fn state_complete() {
        let mut state = State::default();
        expand(&mut state.floors, 3); // 4 Floors
        state.elevator_floor = 3;
        state.floors[3].push(Device::Generator(1));
        state.floors[3].push(Device::Microchip(1));
        assert!(state.complete(), "Should be marked complete {state:?}");
    }

    #[test]
    fn state_incomplete() {
        let mut state = State::default();
        expand(&mut state.floors, 3); // 4 Floors
        state.elevator_floor = 1;
        state.floors[0].push(Device::Generator(1));
        state.floors[1].push(Device::Microchip(1));
        state.floors[2].push(Device::Microchip(2));
        state.floors[2].push(Device::Generator(2));
        assert!(!state.complete(), "Should be marked incomplete {state:?}");
    }

    #[test]
    fn state_valid() {
        let mut state = State::default();
        expand(&mut state.floors, 3); // 4 Floors
        state.elevator_floor = 1;
        state.floors[0].push(Device::Generator(1));
        state.floors[1].push(Device::Microchip(1));
        state.floors[2].push(Device::Microchip(2));
        state.floors[2].push(Device::Generator(2));
        assert!(state.valid(), "Should be marked valid {state:?}");
    }

    #[test]
    fn state_invalid() {
        let mut state = State::default();
        expand(&mut state.floors, 3); // 4 Floors
        state.elevator_floor = 1;
        state.floors[1].push(Device::Generator(1));
        state.floors[1].push(Device::Microchip(2));
        state.floors[2].push(Device::Microchip(1));
        state.floors[2].push(Device::Generator(2));
        assert!(!state.valid(), "Should be marked invalid {state:?}");
    }

    #[test]
    fn state_clone_deep() {
        let mut state = State::default();
        expand(&mut state.floors, 3); // 4 Floors
        state.elevator_floor = 1;
        state.floors[0].push(Device::Generator(1));
        state.floors[1].push(Device::Microchip(1));
        state.floors[2].push(Device::Microchip(2));
        state.floors[2].push(Device::Generator(2));
        let mut state2 = state.clone();
        state2.floors[0].push(Device::Generator(3));
        assert_ne!(state, state2, "Clone should be deep");
    }

    #[test]
    fn priority_ordering_bug_fix() {
        let a = StatePriority {
            steps: 1,
            closeness: 38,
        };
        let b = StatePriority {
            steps: 2,
            closeness: 42,
        };
        assert_eq!(std::cmp::Ordering::Greater, a.cmp(&b));
    }
}
