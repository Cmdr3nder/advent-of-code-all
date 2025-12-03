use crate::util::input::get_input;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use lazy_regex::regex_captures;

use crate::day::Day;

pub struct Day21;

#[derive(Copy, Clone)]
struct Item {
    armor: u32,
    cost: u32,
    damage: u32,
}

#[derive(Copy, Clone, Default)]
struct Creature {
    armor: u32,
    damage: u32,
    health: u32,
}

impl Creature {
    pub fn from_items(items: &[Item], health: u32) -> (Self, u32) {
        let mut damage = 0;
        let mut armor = 0;
        let mut cost = 0;
        for item in items {
            armor += item.armor;
            cost += item.cost;
            damage += item.damage;
        }
        (
            Creature {
                armor,
                damage,
                health,
            },
            cost,
        )
    }
}

fn fight(player: Creature, boss: Creature) -> bool {
    let mut player = player;
    let mut boss = boss;
    loop {
        // player turn
        let dmg = if player.damage > boss.armor {
            player.damage - boss.armor
        } else {
            1
        };
        if dmg >= boss.health {
            return true;
        }
        boss.health -= dmg;
        // boss turn
        let dmg = if boss.damage > player.armor {
            boss.damage - player.armor
        } else {
            1
        };
        if dmg >= player.health {
            return false;
        }
        player.health -= dmg;
    }
}

const PLAYER_HEALTH: u32 = 100;

impl Day for Day21 {
    fn main() -> Result<()> {
        let input = BufReader::new(get_input(2015, 21)?);
        let mut boss = Creature::default();
        for line in input.lines().map(|l| l.unwrap()) {
            if let Some((_, armor)) = regex_captures!("Armor: ([0-9]+)", &line) {
                boss.armor = armor.parse()?;
            }
            if let Some((_, damage)) = regex_captures!("Damage: ([0-9]+)", &line) {
                boss.damage = damage.parse()?;
            }
            if let Some((_, health)) = regex_captures!("Hit Points: ([0-9]+)", &line) {
                boss.health = health.parse()?;
            }
        }
        let weapons = [
            Item {
                cost: 8,
                damage: 4,
                armor: 0,
            },
            Item {
                cost: 10,
                damage: 5,
                armor: 0,
            },
            Item {
                cost: 25,
                damage: 6,
                armor: 0,
            },
            Item {
                cost: 40,
                damage: 7,
                armor: 0,
            },
            Item {
                cost: 74,
                damage: 8,
                armor: 0,
            },
        ];
        let armors = [
            Item {
                cost: 13,
                damage: 0,
                armor: 1,
            },
            Item {
                cost: 31,
                damage: 0,
                armor: 2,
            },
            Item {
                cost: 53,
                damage: 0,
                armor: 3,
            },
            Item {
                cost: 75,
                damage: 0,
                armor: 4,
            },
            Item {
                cost: 102,
                damage: 0,
                armor: 5,
            },
        ];
        let rings = [
            Item {
                cost: 25,
                damage: 1,
                armor: 0,
            },
            Item {
                cost: 50,
                damage: 2,
                armor: 0,
            },
            Item {
                cost: 100,
                damage: 3,
                armor: 0,
            },
            Item {
                cost: 20,
                damage: 0,
                armor: 1,
            },
            Item {
                cost: 40,
                damage: 0,
                armor: 2,
            },
            Item {
                cost: 80,
                damage: 0,
                armor: 3,
            },
        ];
        let mut best_cost = u32::MAX;
        let mut worst_cost = 0;
        for weapon in weapons {
            // No Armor
            // with 0 rings?
            let (player, cost) = Creature::from_items(&[weapon], PLAYER_HEALTH);
            if fight(player, boss) {
                if cost < best_cost {
                    best_cost = cost;
                }
            } else {
                if cost > worst_cost {
                    worst_cost = cost;
                }
            }
            // with 1 ring?
            for ring in rings {
                let (player, cost) = Creature::from_items(&[weapon, ring], PLAYER_HEALTH);
                if fight(player, boss) {
                    if cost < best_cost {
                        best_cost = cost;
                    }
                } else {
                    if cost > worst_cost {
                        worst_cost = cost;
                    }
                }
            }
            // with 2 rings?
            for i in 0..rings.len() - 1 {
                for j in i + 1..rings.len() {
                    let (player, cost) =
                        Creature::from_items(&[weapon, rings[i], rings[j]], PLAYER_HEALTH);
                    if fight(player, boss) {
                        if cost < best_cost {
                            best_cost = cost;
                        }
                    } else {
                        if cost > worst_cost {
                            worst_cost = cost;
                        }
                    }
                }
            }
            // Yes Armor
            for armor in armors {
                // with 0 rings?
                let (player, cost) = Creature::from_items(&[weapon, armor], PLAYER_HEALTH);
                if fight(player, boss) {
                    if cost < best_cost {
                        best_cost = cost;
                    }
                } else {
                    if cost > worst_cost {
                        worst_cost = cost;
                    }
                }
                // with 1 ring?
                for ring in rings {
                    let (player, cost) =
                        Creature::from_items(&[weapon, armor, ring], PLAYER_HEALTH);
                    if fight(player, boss) {
                        if cost < best_cost {
                            best_cost = cost;
                        }
                    } else {
                        if cost > worst_cost {
                            worst_cost = cost;
                        }
                    }
                }
                // with 2 rings?
                for i in 0..rings.len() - 1 {
                    for j in i + 1..rings.len() {
                        let (player, cost) = Creature::from_items(
                            &[weapon, armor, rings[i], rings[j]],
                            PLAYER_HEALTH,
                        );
                        if fight(player, boss) {
                            if cost < best_cost {
                                best_cost = cost;
                            }
                        } else {
                            if cost > worst_cost {
                                worst_cost = cost;
                            }
                        }
                    }
                }
            }
        }
        println!("Best winning cost: {best_cost}");
        println!("Worst losing cost: {worst_cost}");
        Ok(())
    }
}
