use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{bail, Result};
use lazy_regex::regex_captures;

use crate::day::Day;
use crate::util::expand::expand;

#[derive(Copy, Clone)]
enum ValueSource {
    Literal(u32),
    BotHigh(usize),
    BotLow(usize),
}

#[derive(Copy, Clone)]
enum ValueTarget {
    Output,
    Bot,
}

#[derive(Copy, Clone, Default)]
struct Bot {
    input_a: Option<ValueSource>,
    input_b: Option<ValueSource>,
    output_high: Option<ValueTarget>,
    output_low: Option<ValueTarget>,
}

impl Bot {
    fn add_source(&mut self, source: ValueSource) -> Result<()> {
        match (self.input_a, self.input_b) {
            (None, _) => {
                self.input_a = Some(source);
            }
            (_, None) => {
                self.input_b = Some(source);
            }
            _ => bail!("Too many sources!"),
        }
        Ok(())
    }

    fn add_output(&mut self, low: ValueTarget, high: ValueTarget) -> Result<()> {
        match (self.output_low, self.output_high) {
            (None, None) => {
                self.output_low = Some(low);
                self.output_high = Some(high);
            }
            _ => bail!("Too many outputs!"),
        }
        Ok(())
    }

    fn get_values(&mut self, bots: &mut [Bot]) -> Result<(u32, u32)> {
        let a = match self.input_a {
            Some(ValueSource::Literal(x)) => x,
            Some(ValueSource::BotHigh(i)) => {
                let mut bot = bots[i];
                let res = bot.get_high(bots)?;
                bots[i] = bot;
                res
            }
            Some(ValueSource::BotLow(i)) => {
                let mut bot = bots[i];
                let res = bot.get_low(bots)?;
                bots[i] = bot;
                res
            }
            _ => bail!("No input_a available"),
        };
        self.input_a = Some(ValueSource::Literal(a));
        let b = match self.input_b {
            Some(ValueSource::Literal(x)) => x,
            Some(ValueSource::BotHigh(i)) => {
                let mut bot = bots[i];
                let res = bot.get_high(bots)?;
                bots[i] = bot;
                res
            }
            Some(ValueSource::BotLow(i)) => {
                let mut bot = bots[i];
                let res = bot.get_low(bots)?;
                bots[i] = bot;
                res
            }
            _ => bail!("No input_b available"),
        };
        self.input_b = Some(ValueSource::Literal(b));
        if a < b {
            Ok((a, b))
        } else {
            Ok((b, a))
        }
    }

    fn get_low(&mut self, bots: &mut [Bot]) -> Result<u32> {
        self.get_values(bots).map(|(low, _)| low)
    }

    fn get_high(&mut self, bots: &mut [Bot]) -> Result<u32> {
        self.get_values(bots).map(|(_, high)| high)
    }
}

#[derive(Copy, Clone, Default)]
struct Output {
    source: Option<ValueSource>,
}

impl Output {
    fn add_source(&mut self, source: ValueSource) -> Result<()> {
        match self.source {
            None => {
                self.source = Some(source);
            }
            _ => bail!("Too many sources!"),
        }
        Ok(())
    }

    fn get_value(&mut self, bots: &mut [Bot]) -> Result<u32> {
        let value = match self.source {
            Some(ValueSource::Literal(x)) => x,
            Some(ValueSource::BotHigh(i)) => {
                let mut bot = bots[i];
                let res = bot.get_high(bots)?;
                bots[i] = bot;
                res
            }
            Some(ValueSource::BotLow(i)) => {
                let mut bot = bots[i];
                let res = bot.get_low(bots)?;
                bots[i] = bot;
                res
            }
            _ => bail!("No source found for Output"),
        };
        self.source = Some(ValueSource::Literal(value));
        Ok(value)
    }
}

pub struct Day10;

impl Day for Day10 {
    fn main() -> Result<()> {
        let input = BufReader::new(File::open("input/2016/day10.txt")?);
        let mut bots: Vec<Bot> = Vec::new();
        let mut outputs: Vec<Output> = Vec::new();
        for line in input.lines().map(|l| l.unwrap()) {
            if let Some((_, value, bot_num)) =
                regex_captures!("value ([0-9]+) goes to bot ([0-9]+)", &line)
            {
                let value: u32 = value.parse()?;
                let bot_num: usize = bot_num.parse()?;
                expand(&mut bots, bot_num);
                bots[bot_num].add_source(ValueSource::Literal(value))?;
            } else if let Some((_, bot_num, low_target, low_num, high_target, high_num)) = regex_captures!(
                "bot ([0-9]+) gives low to (output|bot) ([0-9]+) and high to (output|bot) ([0-9]+)",
                &line
            ) {
                let bot_num: usize = bot_num.parse()?;
                let low_num: usize = low_num.parse()?;
                let high_num: usize = high_num.parse()?;
                let low = match low_target {
                    "output" => {
                        expand(&mut outputs, low_num);
                        outputs[low_num].add_source(ValueSource::BotLow(bot_num))?;
                        ValueTarget::Output
                    }
                    "bot" => {
                        expand(&mut bots, low_num);
                        bots[low_num].add_source(ValueSource::BotLow(bot_num))?;
                        ValueTarget::Bot
                    }
                    _ => bail!("Unexpected target '{low_target}'"),
                };
                let high = match high_target {
                    "output" => {
                        expand(&mut outputs, high_num);
                        outputs[high_num].add_source(ValueSource::BotHigh(bot_num))?;
                        ValueTarget::Output
                    }
                    "bot" => {
                        expand(&mut bots, high_num);
                        bots[high_num].add_source(ValueSource::BotHigh(bot_num))?;
                        ValueTarget::Bot
                    }
                    _ => bail!("Unexpected target '{high_target}'"),
                };
                expand(&mut bots, bot_num);
                bots[bot_num].add_output(low, high)?;
            }
        }
        for i in 0..bots.len() {
            let mut bot = bots[i];
            let (low, high) = bot.get_values(&mut bots)?;
            bots[i] = bot;
            if low == 17 && high == 61 {
                println!("Bot {i} compares 17 to 61");
                break;
            }
        }
        let mut product = 1;
        for i in 0..=2 {
            product *= outputs[i].get_value(&mut bots)?;
        }
        println!("Product of first 3 chip outputs is {product}");
        Ok(())
    }
}
