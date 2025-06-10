use anyhow::Result;

use crate::day::Day;
use crate::input::get_input;

pub struct Day10;

impl Day for Day10 {
    fn main() -> Result<()> {
        let mut sequence: Vec<char> = get_input(2015, 10)?
            .trim()
            .chars()
            .collect();
        let mut forty_len = 0;
        let mut fifty_len = 0;
        for i in 1..=50 {
            let mut new_sequence = Vec::new();
            {
                // lock mutability temporarily for this loop sequence since I had a bug related to appending to the wrong sequenc
                let sequence = sequence;
                let mut j = 0;
                while j < sequence.len() {
                    let ch = sequence[j];
                    let mark = j;
                    while j < sequence.len() && sequence[j] == ch {
                        j += 1;
                    }
                    for num_char in (j - mark).to_string().chars() {
                        new_sequence.push(num_char);
                    }
                    new_sequence.push(ch);
                }
            }
            sequence = new_sequence;
            if i == 40 {
                forty_len = sequence.len();
            } else if i == 50 {
                fifty_len = sequence.len();
            }
        }
        println!("Length @ 40th iteration: {forty_len}");
        println!("Length @ 50th iteration: {fifty_len}");
        Ok(())
    }
}
