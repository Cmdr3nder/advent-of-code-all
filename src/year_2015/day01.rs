use crate::util::input::get_input_string;

use anyhow::Result;

use crate::day::Day;

pub struct Day01;

impl Day for Day01 {
    fn main() -> Result<()> {
        let input = get_input_string(2015, 01)?;
        let mut floor: i64 = 0;
        let mut pos: usize = 0;
        for (idx, ch) in input.chars().enumerate() {
            floor += match ch {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };
            if pos == 0 && floor < 0 {
                pos = idx + 1;
            }
        }
        println!("The final floor for part 1 is {floor}");
        println!("The position that causes basement first is {pos}");
        Ok(())
    }
}
