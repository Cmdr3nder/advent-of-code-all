use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;

use crate::util::priority_queue::PriorityQueue;

pub trait WeightedGraph<K, C> {
    fn neighbors(&self, current: K) -> Vec<K>;

    fn cost(&self, current: K, next: K) -> C;

    fn heuristic(&self, next: K, goal: K) -> C;
}

pub fn a_star_search<K, C>(
    graph: impl WeightedGraph<K, C>,
    start: K,
    goal: K,
) -> (HashMap<K, Option<K>>, HashMap<K, C>)
where
    C: Default + PartialOrd + Add<Output = C> + Copy,
    K: PartialEq + Eq + Hash + Copy,
{
    let mut frontier = PriorityQueue::new();
    frontier.push(start, C::default());
    let mut came_from = HashMap::new();
    let mut cost_so_far = HashMap::new();
    came_from.insert(start, None);
    cost_so_far.insert(start, C::default());

    while let Some(current) = frontier.pop() {
        if current == goal {
            break;
        }

        for next in graph.neighbors(current) {
            let new_cost: C = cost_so_far[&current] + graph.cost(current, next);
            // if next not in cost_so_far or new_cost < cost_so_far[next]
            // if not (next in cost_so_far and cost_so_far better than new_cost)
            if None
                == cost_so_far
                    .get(&next)
                    .filter(|cost_so_far| new_cost >= **cost_so_far)
            {
                cost_so_far.insert(next, new_cost);
                let priority = new_cost + graph.heuristic(next, goal);
                frontier.push(next, priority);
                came_from.insert(next, Some(current));
            }
        }
    }

    (came_from, cost_so_far)
}
