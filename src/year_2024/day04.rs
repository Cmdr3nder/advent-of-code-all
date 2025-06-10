use anyhow::{Context, Result};

use crate::day::Day;
use crate::input::get_input;

struct XmasCounter {
    cache: [char; 4],
    count: u32,
}

impl XmasCounter {
    fn new() -> Self {
        XmasCounter {
            cache: ['.', '.', '.', '.'],
            count: 0,
        }
    }

    fn reset_cache(&mut self) {
        self.cache = ['.', '.', '.', '.'];
    }

    fn process(&mut self, ch: char) {
        // Do Shift
        self.cache[0] = self.cache[1];
        self.cache[1] = self.cache[2];
        self.cache[2] = self.cache[3];
        self.cache[3] = ch;

        // Examine for 'XMAS'
        if (self.cache[0] == 'X'
            && self.cache[1] == 'M'
            && self.cache[2] == 'A'
            && self.cache[3] == 'S')
            || (self.cache[3] == 'X'
                && self.cache[2] == 'M'
                && self.cache[1] == 'A'
                && self.cache[0] == 'S')
        {
            self.count += 1;
        }
    }
}

pub struct Day04;

impl Day for Day04 {
    fn main() -> Result<()> {
        let input = get_input(2024, 4)?;
        let mat: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let y_len: usize = mat.len();
        let x_len: usize = mat
            .iter()
            .map(|line| line.len())
            .reduce(|max_len, len| if len >= max_len { len } else { max_len })
            .with_context(|| "Unexpected failure in reduce.")?;
        let mut counter = XmasCounter::new();
        // East->West Check (+ West->East)
        for y in 0..y_len {
            for x in 0..x_len {
                counter.process(mat[y][x]);
            }
            counter.reset_cache();
        }
        // North->South Check (+ South->North)
        for x in 0..x_len {
            for y in 0..y_len {
                counter.process(mat[y][x]);
            }
            counter.reset_cache();
        }
        // ->South East Diagonal Check (+ ->North West)
        for x_start in 0..x_len {
            // Top
            let mut x = x_start;
            let mut y = 0;
            while x < x_len && y < y_len {
                counter.process(mat[y][x]);
                x += 1;
                y += 1;
            }
            counter.reset_cache();
        }
        for y_start in 1..y_len {
            // Left-Side Skipping 0,0 common start
            let mut y = y_start;
            let mut x = 0;
            while x < x_len && y < y_len {
                counter.process(mat[y][x]);
                x += 1;
                y += 1;
            }
            counter.reset_cache();
        }
        // ->South West Diagonal Check (+ ->North East)
        for x_start in 0..x_len {
            // Top
            let mut x = x_start;
            let mut y = 0;
            while y < y_len {
                counter.process(mat[y][x]);
                if x == 0 {
                    break;
                }
                x -= 1;
                y += 1;
            }
            counter.reset_cache();
        }
        for y_start in 1..y_len {
            // Right-Side Skipping x_len-1,0 common start
            let mut y = y_start;
            let mut x = x_len - 1;
            while y < y_len {
                counter.process(mat[y][x]);
                if x == 0 {
                    break;
                }
                x -= 1;
                y += 1;
            }
            counter.reset_cache();
        }
        println!("XMAS Instances: {}", counter.count);
        let mut xmas_count = 0;
        for x in 1..(x_len - 1) {
            for y in 1..(y_len - 1) {
                if mat[y][x] == 'A' {
                    let nw = mat[y - 1][x - 1];
                    let ne = mat[y - 1][x + 1];
                    let sw = mat[y + 1][x - 1];
                    let se = mat[y + 1][x + 1];
                    let d1 = (nw == 'M' && se == 'S') || (nw == 'S' && se == 'M');
                    let d2 = (ne == 'M' && sw == 'S') || (ne == 'S' && sw == 'M');
                    if d1 && d2 {
                        xmas_count += 1;
                    }
                }
            }
        }
        println!("X-MAS Instances: {xmas_count}");
        Ok(())
    }
}
