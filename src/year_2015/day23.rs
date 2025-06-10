use crate::input::get_input;

use anyhow::{bail, Result};
use lazy_regex::regex_captures;

use crate::day::Day;

#[derive(Copy, Clone)]
enum Register {
    A,
    B,
}

impl Register {
    fn from_arg(arg: &str) -> Result<Self> {
        match arg {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            _ => bail!("Unexpected register {arg}"),
        }
    }
}

#[derive(Copy, Clone)]
enum Offset {
    Forward(usize),
    Backward(usize),
}

impl Offset {
    fn from_arg(sign: &str, arg: &str) -> Result<Self> {
        let magnitude: usize = arg.parse()?;
        match sign {
            "+" => Ok(Offset::Forward(magnitude)),
            "-" => Ok(Offset::Backward(magnitude)),
            _ => bail!("Unexpected sign {sign}"),
        }
    }
}

#[derive(Copy, Clone)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(Offset),
    JumpIfEven(Register, Offset),
    JumpIfOne(Register, Offset),
}

impl Instruction {
    fn from_line(line: &str) -> Result<Self> {
        if let Some((_, reg)) = regex_captures!("hlf (a|b)", &line) {
            let register = Register::from_arg(reg)?;
            Ok(Instruction::Half(register))
        } else if let Some((_, reg)) = regex_captures!("tpl (a|b)", &line) {
            let register = Register::from_arg(reg)?;
            Ok(Instruction::Triple(register))
        } else if let Some((_, reg)) = regex_captures!("inc (a|b)", &line) {
            let register = Register::from_arg(reg)?;
            Ok(Instruction::Increment(register))
        } else if let Some((_, sign, off)) = regex_captures!("jmp ([+-]{1})([0-9]+)", &line) {
            let offset = Offset::from_arg(sign, off)?;
            Ok(Instruction::Jump(offset))
        } else if let Some((_, reg, sign, off)) =
            regex_captures!("jie (a|b), ([+-]{1})([0-9]+)", &line)
        {
            let register = Register::from_arg(reg)?;
            let offset = Offset::from_arg(sign, off)?;
            Ok(Instruction::JumpIfEven(register, offset))
        } else if let Some((_, reg, sign, off)) =
            regex_captures!("jio (a|b), ([+-]{1})([0-9]+)", &line)
        {
            let register = Register::from_arg(reg)?;
            let offset = Offset::from_arg(sign, off)?;
            Ok(Instruction::JumpIfOne(register, offset))
        } else {
            bail!("Unrecognized instruction {line}")
        }
    }
}

pub struct Day23;

fn execute(program: &[Instruction], a: u32, b: u32) -> (u32, u32) {
    let mut a = a;
    let mut b = b;
    let mut ins: usize = 0;
    while ins < program.len() {
        match program[ins] {
            Instruction::Half(Register::A) => {
                a /= 2;
                ins += 1;
            }
            Instruction::Half(Register::B) => {
                b /= 2;
                ins += 1;
            }
            Instruction::Triple(Register::A) => {
                a *= 3;
                ins += 1;
            }
            Instruction::Triple(Register::B) => {
                b *= 3;
                ins += 1;
            }
            Instruction::Increment(Register::A) => {
                a += 1;
                ins += 1;
            }
            Instruction::Increment(Register::B) => {
                b += 1;
                ins += 1;
            }
            Instruction::Jump(Offset::Forward(x)) => {
                ins += x;
            }
            Instruction::Jump(Offset::Backward(x)) => {
                ins = if x > ins {
                    // Terminate Program
                    program.len()
                } else {
                    ins - x
                };
            }
            Instruction::JumpIfEven(register, offset) => {
                let value = match register {
                    Register::A => a,
                    Register::B => b,
                };
                ins = if value % 2 == 0 {
                    match offset {
                        Offset::Forward(x) => ins + x,
                        Offset::Backward(x) => {
                            if x > ins {
                                // Terminate Program
                                program.len()
                            } else {
                                ins - x
                            }
                        }
                    }
                } else {
                    ins + 1
                };
            }
            Instruction::JumpIfOne(register, offset) => {
                let value = match register {
                    Register::A => a,
                    Register::B => b,
                };
                ins = if value == 1 {
                    match offset {
                        Offset::Forward(x) => ins + x,
                        Offset::Backward(x) => {
                            if x > ins {
                                // Terminate Program
                                program.len()
                            } else {
                                ins - x
                            }
                        }
                    }
                } else {
                    ins + 1
                };
            }
        }
    }
    (a, b)
}

impl Day for Day23 {
    fn main() -> Result<()> {
        let input = get_input(2015, 23)?;
        let mut program: Vec<Instruction> = Vec::new();
        for line in input.lines() {
            program.push(Instruction::from_line(&line)?);
        }
        let (_, b) = execute(&program, 0, 0);
        println!("Register b: {b}");
        let (_, b) = execute(&program, 1, 0);
        println!("Register b when a=1: {b}");
        Ok(())
    }
}
