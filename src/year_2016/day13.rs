use std::collections::HashSet;
use std::fs;

use anyhow::Result;

use crate::data::Point2D;
use crate::day::Day;
use crate::util::a_star::a_star;

fn is_wall(point: Point2D<usize>, favorite_number: usize) -> bool {
    let x = point.x;
    let y = point.y;
    let location_number = (x * x) + (3 * x) + (2 * x * y) + y + (y * y) + favorite_number;
    let bits = location_number.count_ones();
    bits % 2 != 0
}

pub struct Day13;

fn cardinal_neighbors(point: Point2D<usize>, favorite_number: usize) -> Vec<Point2D<usize>> {
    let mut neighbors = Vec::new();
    if point.x > usize::MIN {
        let p = Point2D::new(point.x - 1, point.y);
        if !is_wall(p, favorite_number) {
            neighbors.push(p);
        }
    }
    if point.x < usize::MAX {
        let p = Point2D::new(point.x + 1, point.y);
        if !is_wall(p, favorite_number) {
            neighbors.push(p);
        }
    }
    if point.y > usize::MIN {
        let p = Point2D::new(point.x, point.y - 1);
        if !is_wall(p, favorite_number) {
            neighbors.push(p);
        }
    }
    if point.y < usize::MAX {
        let p = Point2D::new(point.x, point.y + 1);
        if !is_wall(p, favorite_number) {
            neighbors.push(p);
        }
    }
    neighbors
}

impl Day for Day13 {
    fn main() -> Result<()> {
        let input = fs::read_to_string("input/2016/day13.txt")?;
        let input = input.trim();
        let favorite_number: usize = input.parse()?;
        let start = Point2D::new(1, 1);
        // Work up to A* using the red star games site and our imported priority queue?
        let path = a_star(
            start,
            Point2D::new(31, 39),
            &|p: &Point2D<usize>| cardinal_neighbors(*p, favorite_number),
            &|_, _| 1,
            &|a: &Point2D<usize>, b: &Point2D<usize>| {
                let (x1, x2) = if a.x > b.x { (a.x, b.x) } else { (b.x, a.x) };
                let (y1, y2) = if a.y > b.y { (a.y, b.y) } else { (b.y, a.y) };
                (x1 - x2) + (y1 - y2)
            },
        );
        for y in 0..50 {
            for x in 0..50 {
                let p = Point2D::new(x, y);
                if path.contains(&p) {
                    print!("·");
                } else if is_wall(p, favorite_number) {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
        println!("Fewest steps required: {}", path.len() - 1);
        let mut frontier = Vec::new();
        frontier.push(start);
        let mut reached = HashSet::new();
        reached.insert(start);
        for _ in 0..50 {
            let mut new_frontier = Vec::new();
            for current in frontier {
                for next in cardinal_neighbors(current, favorite_number) {
                    if !reached.contains(&next) {
                        new_frontier.push(next);
                        reached.insert(next);
                    }
                }
            }
            frontier = new_frontier;
        }
        println!("Within 50 steps: {}", reached.len());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_example() {
        let path = a_star(
            Point2D::new(1, 1),
            Point2D::new(7, 4),
            &|p: &Point2D<usize>| cardinal_neighbors(*p, 10),
            &|_, _| 1,
            &|a: &Point2D<usize>, b: &Point2D<usize>| {
                let (x1, x2) = if a.x > b.x { (a.x, b.x) } else { (b.x, a.x) };
                let (y1, y2) = if a.y > b.y { (a.y, b.y) } else { (b.y, a.y) };
                (x1 - x2) + (y1 - y2)
            },
        );
        assert_eq!(11, path.len() - 1);
        assert_eq!(
            vec![
                Point2D::new(1, 1),
                Point2D::new(1, 2),
                Point2D::new(2, 2),
                Point2D::new(3, 2),
                Point2D::new(3, 3),
                Point2D::new(3, 4),
                Point2D::new(4, 4),
                Point2D::new(4, 5),
                Point2D::new(5, 5),
                Point2D::new(6, 5),
                Point2D::new(7, 5),
                Point2D::new(7, 4),
            ],
            path
        );
    }
}
