use std::collections::HashMap;

use anyhow::{bail, Context, Result};
use lazy_regex::regex_captures;

use crate::day::Day;
use crate::input::get_input;

pub struct Day07;

#[derive(Clone, Debug)]
enum Operand {
    Ref(String),
    Const(u16),
}

#[derive(Clone, Debug)]
enum Instruction {
    Value(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Lshift(Operand, Operand),
    Rshift(Operand, Operand),
    Not(Operand),
}

fn into_operand(name: &str, num: &str) -> Result<Operand> {
    Ok(match (name.is_empty(), num.is_empty()) {
        (false, false) => bail!("Indeterminate between Const and Ref"),
        (false, true) => Operand::Ref(name.to_string()),
        (true, false) => Operand::Const(
            num.parse()
                .with_context(|| format!("Couldn't convert operand '{num}' into u16"))?,
        ),
        (true, true) => bail!("Need a value to bind for the operand"),
    })
}

fn execute_operand(
    instructions: &HashMap<String, Instruction>,
    result_cache: &mut HashMap<String, u16>,
    operand: &Operand,
) -> Result<u16> {
    match operand {
        Operand::Const(v) => Ok(*v),
        Operand::Ref(name) => match result_cache.get(name) {
            Some(v) => Ok(*v),
            None => execute(instructions, result_cache, name),
        },
    }
}

fn execute(
    instructions: &HashMap<String, Instruction>,
    result_cache: &mut HashMap<String, u16>,
    name: &str,
) -> Result<u16> {
    let instruction = instructions
        .get(name)
        .with_context(|| format!("Instruction not found: '{name}'"))?;
    let res = match instruction {
        Instruction::Value(o) => execute_operand(instructions, result_cache, o)?,
        Instruction::And(lh, rh) => {
            let lh = execute_operand(instructions, result_cache, lh)?;
            let rh = execute_operand(instructions, result_cache, rh)?;
            lh & rh
        }
        Instruction::Or(lh, rh) => {
            let lh = execute_operand(instructions, result_cache, lh)?;
            let rh = execute_operand(instructions, result_cache, rh)?;
            lh | rh
        }
        Instruction::Lshift(lh, rh) => {
            let lh = execute_operand(instructions, result_cache, lh)?;
            let rh = execute_operand(instructions, result_cache, rh)?;
            lh << rh
        }
        Instruction::Rshift(lh, rh) => {
            let lh = execute_operand(instructions, result_cache, lh)?;
            let rh = execute_operand(instructions, result_cache, rh)?;
            lh >> rh
        }
        Instruction::Not(o) => !execute_operand(instructions, result_cache, o)?,
    };
    result_cache.insert(name.to_string(), res);
    Ok(res)
}

impl Day for Day07 {
    fn main() -> Result<()> {
        let input_str = get_input(2015, 7)?;
        let mut instructions = HashMap::new();
        for line in input_str.lines() {
            let (_, lh_name, lh_num, op, rh_name, rh_num, out) = regex_captures!(
                "([a-z]+)?([0-9]+)? ?(AND|OR|LSHIFT|RSHIFT|NOT)? ?([a-z]+)?([0-9]+)? -> ([a-z]+)",
                &line
            )
            .with_context(|| format!("Failed to match line regex {line}"))?;
            instructions.insert(
                out.to_string(),
                match op {
                    "" => Instruction::Value(into_operand(lh_name, lh_num)?),
                    "AND" => Instruction::And(
                        into_operand(lh_name, lh_num)?,
                        into_operand(rh_name, rh_num)?,
                    ),
                    "OR" => Instruction::Or(
                        into_operand(lh_name, lh_num)?,
                        into_operand(rh_name, rh_num)?,
                    ),
                    "LSHIFT" => Instruction::Lshift(
                        into_operand(lh_name, lh_num)?,
                        into_operand(rh_name, rh_num)?,
                    ),
                    "RSHIFT" => Instruction::Rshift(
                        into_operand(lh_name, lh_num)?,
                        into_operand(rh_name, rh_num)?,
                    ),
                    "NOT" => Instruction::Not(into_operand(rh_name, rh_num)?),
                    _ => panic!("Unexpected op code, '{op}'"),
                },
            );
        }
        let res = execute(&instructions, &mut HashMap::new(), "a")?;
        println!("wire a: {res}");
        let mut cache = HashMap::new();
        cache.insert("b".to_string(), res);
        println!("wire a, lv2: {}", execute(&instructions, &mut cache, "a")?);
        Ok(())
    }
}
