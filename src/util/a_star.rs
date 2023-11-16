use std::cmp::Reverse;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;

use crate::util::priority_queue::PriorityQueue;

pub fn a_star<T, C>(
    start: T,
    goal: T,
    get_neighbors: &dyn Fn(&T) -> Vec<T>,
    get_cost: &dyn Fn(&T, &T) -> C,
    heuristic: &dyn Fn(&T, &T) -> C,
) -> Vec<T>
where
    T: Copy + Eq + Hash,
    C: Copy + Ord + Default + Add<Output = C>,
{
    let mut frontier: PriorityQueue<Reverse<C>, T> = PriorityQueue::new();
    frontier.push(start, Reverse(C::default()));
    let mut came_from: HashMap<T, T> = HashMap::new();
    let mut cost_so_far: HashMap<T, C> = HashMap::new();
    cost_so_far.insert(start, C::default());

    while let Some((current, _)) = frontier.pop() {
        if current == goal {
            break;
        }
        for next in get_neighbors(&current) {
            let new_cost = cost_so_far
                .get(&current)
                .map(|c| *c)
                .unwrap_or(C::default())
                + get_cost(&current, &next);
            let insert_new_cost = if let Some(csf) = cost_so_far.get(&next) {
                new_cost < *csf
            } else {
                true
            };
            if insert_new_cost {
                cost_so_far.insert(next, new_cost);
                let priority = new_cost + heuristic(&goal, &next);
                frontier.push(next, Reverse(priority));
                came_from.insert(next, current);
            }
        }
    }

    let mut path = Vec::new();
    let mut current = goal;
    path.insert(0, current);
    while let Some(c) = came_from.get(&current) {
        path.insert(0, *c);
        current = *c;
    }
    path
}
