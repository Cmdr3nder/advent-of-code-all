use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{bail, Context, Result};

use crate::day::Day;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn parse(ch: char) -> Result<Self> {
        Ok(match ch {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => bail!("Unexpected Card char '{ch}'"),
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn parse(s: &str) -> Result<Self> {
        let mut cards: [Card; 5] = [Card::Ace; 5];
        if s.len() != 5 {
            bail!("Unexpected string length '{s}' when parsing Hand");
        }
        for (i, ch) in s.chars().enumerate() {
            cards[i] = Card::parse(ch)?;
        }
        Ok(Hand { cards })
    }

    fn hand_type(&self) -> HandType {
        let mut counts: HashMap<Card, u8> = HashMap::new();
        for card in self.cards {
            *counts.entry(card).or_insert(0) += 1;
        }
        match counts.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                let first_value = counts.values().next().map(|v| *v);
                if first_value == Some(1) || first_value == Some(4) {
                    HandType::FourOfAKind
                } else {
                    // 2 | 3
                    HandType::FullHouse
                }
            }
            3 => {
                let first_value = counts.values().map(|v| *v).filter(|v| *v != 1).next();
                if first_value == Some(3) {
                    HandType::ThreeOfAKind
                } else {
                    // 2
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            std::cmp::Ordering::Equal => {
                let mut ord = std::cmp::Ordering::Equal;
                for i in 0..5 {
                    ord = self.cards[i].cmp(&other.cards[i]);
                    if ord != std::cmp::Ordering::Equal {
                        break;
                    }
                }
                ord
            }
            x => x,
        }
    }
}

pub struct Day07;

impl Day for Day07 {
    fn main() -> Result<()> {
        let input = BufReader::new(File::open("input/2023/day07.txt")?);
        let mut hands = Vec::new();
        for line in input.lines().map(|l| l.unwrap()) {
            let mut parts = line.split_whitespace();
            let hand = Hand::parse(parts.next().with_context(|| "Expected hand chars")?)?;
            let bet: usize = parts
                .next()
                .with_context(|| "Expected bet chars")?
                .parse()?;
            hands.push((hand, bet));
        }
        hands.sort_by_key(|(hand, _)| *hand);
        let total_winnings: usize = hands
            .iter()
            .enumerate()
            .map(|(i, (_, bid))| (i + 1) * bid)
            .sum();
        println!("Total Winnings in Camel Cards! {total_winnings}");
        Ok(())
    }
}
