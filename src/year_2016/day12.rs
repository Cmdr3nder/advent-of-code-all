use crate::util::input::get_input;
use std::io::{BufRead, BufReader};

use anyhow::{bail, Result};
use lazy_regex::regex_captures;

use crate::data::StringIdMap;
use crate::day::Day;
use crate::util::expand::expand;

#[derive(Clone, Copy)]
enum Instruction {
    CopyLiteral(i32, usize),             // value -> register
    CopyRegister(usize, usize),          // register -> register
    Increment(usize),                    // register++
    Decrement(usize),                    // register--
    JumpForwardIfNotZero(usize, usize),  // register, magnitude towards usize::MAX
    JumpBackwardIfNotZero(usize, usize), // register, magnitude towards 0
    JumpForward(usize),                  // magnitude
    JumpBackward(usize),                 // magnitude
    Empty,
}

fn execute(instructions: &[Instruction], registers: &[i32]) -> Vec<i32> {
    let mut registers: Vec<i32> = registers.iter().map(|r| *r).collect();
    let mut instruction_pointer: usize = 0;
    loop {
        match instructions[instruction_pointer] {
            Instruction::CopyLiteral(value, register) => {
                registers[register] = value;
                instruction_pointer += 1;
            }
            Instruction::CopyRegister(from, to) => {
                registers[to] = registers[from];
                instruction_pointer += 1;
            }
            Instruction::Increment(register) => {
                registers[register] += 1;
                instruction_pointer += 1;
            }
            Instruction::Decrement(register) => {
                registers[register] -= 1;
                instruction_pointer += 1;
            }
            Instruction::JumpForwardIfNotZero(register, magnitude) => {
                instruction_pointer += if registers[register] != 0 {
                    magnitude
                } else {
                    1
                };
            }
            Instruction::JumpBackwardIfNotZero(register, magnitude) => {
                if registers[register] != 0 {
                    if magnitude <= instruction_pointer {
                        instruction_pointer -= magnitude;
                    } else {
                        instruction_pointer = instructions.len();
                    }
                } else {
                    instruction_pointer += 1;
                }
            }
            Instruction::JumpForward(magnitude) => {
                instruction_pointer += magnitude;
            }
            Instruction::JumpBackward(magnitude) => {
                if magnitude <= instruction_pointer {
                    instruction_pointer -= 1;
                } else {
                    instruction_pointer = instructions.len();
                }
            }
            Instruction::Empty => {
                instruction_pointer += 1;
            }
        }
        if instruction_pointer >= instructions.len() {
            break;
        }
    }
    registers
}

pub struct Day12;

impl Day for Day12 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2016, 12)?);
        let mut register_names = StringIdMap::default();
        let mut registers: Vec<i32> = Vec::new();
        let a = register_names.to_id("a");
        let c = register_names.to_id("c");
        expand(&mut registers, a);
        expand(&mut registers, c);
        let mut instructions: Vec<Instruction> = Vec::new();
        for line in input.lines().map(|l| l.unwrap()) {
            if let Some((_, value, register)) = regex_captures!("cpy (-?[0-9]+) (a|b|c|d)", &line) {
                let value: i32 = value.parse()?;
                let register = register_names.to_id(register);
                expand(&mut registers, register);
                instructions.push(Instruction::CopyLiteral(value, register));
            } else if let Some((_, from, to)) = regex_captures!("cpy (a|b|c|d) (a|b|c|d)", &line) {
                let from = register_names.to_id(from);
                expand(&mut registers, from);
                let to = register_names.to_id(to);
                expand(&mut registers, to);
                instructions.push(Instruction::CopyRegister(from, to));
            } else if let Some((_, register)) = regex_captures!("inc (a|b|c|d)", &line) {
                let register = register_names.to_id(register);
                expand(&mut registers, register);
                instructions.push(Instruction::Increment(register));
            } else if let Some((_, register)) = regex_captures!("dec (a|b|c|d)", &line) {
                let register = register_names.to_id(register);
                expand(&mut registers, register);
                instructions.push(Instruction::Decrement(register));
            } else if let Some((_, register, neg, magnitude)) =
                regex_captures!("jnz (a|b|c|d) (-?)([0-9]+)", &line)
            {
                let register = register_names.to_id(register);
                expand(&mut registers, register);
                let magnitude: usize = magnitude.parse()?;
                instructions.push(match neg {
                    "-" => Instruction::JumpBackwardIfNotZero(register, magnitude),
                    _ => Instruction::JumpForwardIfNotZero(register, magnitude),
                });
            } else if let Some((_, value, neg, magnitude)) =
                regex_captures!("jnz (-?[0-9]+) (-?)([0-9]+)", &line)
            {
                instructions.push(if value == "0" {
                    Instruction::Empty
                } else {
                    let magnitude: usize = magnitude.parse()?;
                    match neg {
                        "-" => Instruction::JumpBackward(magnitude),
                        _ => Instruction::JumpForward(magnitude),
                    }
                });
            } else {
                bail!("Unexpected instruction '{line}'");
            }
        }

        // Run 1
        let run1 = execute(&instructions, &registers);
        println!("Result in 'a' register is: {}", run1[a]);

        // Run 2
        registers[c] = 1;
        let run2 = execute(&instructions, &registers);
        println!("Result in 'a' register when reg_c = 1 is: {}", run2[a]);

        Ok(())
    }
}
