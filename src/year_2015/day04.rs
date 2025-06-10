use anyhow::Result;
use lazy_regex::regex_is_match;

use crate::day::Day;
use crate::input::get_input;

pub struct Day04;

impl Day for Day04 {
    fn main() -> Result<()> {
        let input = get_input(2015, 4)?;
        let input = input.trim();
        let mut five_zeros: usize = 0;
        let mut six_zeros: usize = 0;
        let mut num: usize = 1;
        while five_zeros == 0 || six_zeros == 0 {
            let digest = md5::compute(format!("{input}{num}"));
            let digest = format!("{digest:x}");
            if five_zeros == 0 && regex_is_match!("^0{5}", &digest) {
                five_zeros = num;
            }
            if six_zeros == 0 && regex_is_match!("^0{6}", &digest) {
                six_zeros = num;
            }
            num += 1;
        }
        println!("First number with leading 5 zeroes md5 digest: {five_zeros}");
        println!("First number with leading 6 zeroes md5 digest: {six_zeros}");
        Ok(())
    }
}
