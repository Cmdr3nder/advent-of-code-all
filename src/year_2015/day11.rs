use crate::util::input::get_input_string;

use anyhow::{bail, Context, Result};

use crate::day::Day;

pub struct Day11;

fn includes_straight(password: &[char]) -> bool {
    for index in 0..password.len() - 2 {
        if (password[index] as u32) + 1 == (password[index + 1] as u32)
            && (password[index] as u32) + 2 == (password[index + 2] as u32)
        {
            return true;
        }
    }
    false
}

fn excludes_confusing_letters(password: &[char]) -> bool {
    for ch in password {
        match ch {
            'i' | 'o' | 'l' => {
                return false;
            }
            _ => {}
        }
    }
    true
}

fn includes_pairs(password: &[char]) -> bool {
    let mut iter = password.iter().peekable();
    let mut has_first_pair = false;
    while let Some(ch) = iter.next() {
        if let Some(_) = iter.next_if_eq(&ch) {
            if has_first_pair {
                return true;
            } else {
                has_first_pair = true;
            }
        }
    }
    false
}

fn is_good_password(password: &[char]) -> bool {
    includes_straight(password) && excludes_confusing_letters(password) && includes_pairs(password)
}

fn increment_password(password: &mut Vec<char>) -> Result<()> {
    for index in (0..password.len()).rev() {
        match password[index] {
            'a'..='y' => {
                password[index] = std::char::from_u32(password[index] as u32 + 1)
                    .with_context(|| "Char math failed")?;
                break;
            }
            'z' => {
                password[index] = 'a';
            }
            ch => bail!("Unexpected char '{ch}'"),
        }
    }
    Ok(())
}

impl Day for Day11 {
    fn main() -> Result<()> {
        let input = get_input_string(2015, 11)?;
        let mut password: Vec<char> = input.trim().chars().collect();
        let mut good_passwords: Vec<String> = Vec::with_capacity(2);
        while good_passwords.len() < 2 {
            increment_password(&mut password)?;
            if is_good_password(&password) {
                good_passwords.push(password.iter().collect::<String>());
            }
        }
        println!("Next Password '{}'", good_passwords[0]);
        println!("Next-Next Password '{}'", good_passwords[1]);
        Ok(())
    }
}
