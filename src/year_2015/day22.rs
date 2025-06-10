use crate::input::get_input;

use anyhow::Result;
use lazy_regex::regex_captures;

use crate::day::Day;
use crate::util::priority_queue::PriorityQueue;

pub struct Day22;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Game {
    boss_damage: u32,
    boss_health: u32,
    drain: u32,
    mana_history: u32,
    player_health: u32,
    player_mana: u32,
    poison: u8,
    recharge: u8,
    shielded: bool,
    shield: u8,
}

const SHIELD_TURNS: u8 = 6;
const POISON_TURNS: u8 = 6;
const RECHARGE_TURNS: u8 = 5;

impl Game {
    fn new() -> Self {
        Game {
            boss_damage: 0,
            boss_health: 0,
            drain: 0,
            mana_history: 0,
            player_health: 50,
            player_mana: 500,
            poison: POISON_TURNS,
            recharge: RECHARGE_TURNS,
            shielded: false,
            shield: SHIELD_TURNS,
        }
    }

    fn start_turn(&self) -> Self {
        let mut game = *self;
        if game.shield < SHIELD_TURNS {
            game.shielded = true;
            game.shield += 1;
        }
        if game.poison < POISON_TURNS {
            game = game.damage_boss(3);
            game.poison += 1;
        }
        if game.recharge < RECHARGE_TURNS {
            game.player_mana += 101;
            game.recharge += 1;
        }
        game
    }

    fn end_turn(&self) -> Self {
        let mut game = *self;
        game.shielded = false;
        game
    }

    fn use_mana(&self, cost: u32) -> Option<Self> {
        if self.player_mana >= cost {
            let mut game = *self;
            game.player_mana -= cost;
            game.mana_history += cost;
            Some(game)
        } else {
            None
        }
    }

    fn damage_boss(&self, dmg: u32) -> Self {
        let mut game = *self;
        game.boss_health = game.boss_health.saturating_sub(dmg);
        game
    }

    fn damage_player(&self) -> Self {
        let mut game = *self;
        let dmg = if game.shielded {
            if game.boss_damage <= 7 {
                1
            } else {
                game.boss_damage - 7
            }
        } else {
            game.boss_damage
        };
        game.player_health = game.player_health.saturating_sub(dmg);
        game
    }

    fn drain_health(&self) -> Self {
        let mut game = *self;
        game.player_health = game.player_health.saturating_sub(game.drain);
        game
    }

    fn heal_player(&self, health: u32) -> Self {
        let mut game = *self;
        game.player_health += health;
        game
    }

    fn cast_magic_missile(&self) -> Option<Self> {
        Some(self.use_mana(53)?.damage_boss(4))
    }

    fn cast_drain(&self) -> Option<Self> {
        Some(self.use_mana(73)?.damage_boss(2).heal_player(2))
    }

    fn cast_shield(&self) -> Option<Self> {
        if self.shield >= SHIELD_TURNS {
            let mut game = self.use_mana(113)?;
            game.shield = 0;
            Some(game)
        } else {
            None
        }
    }

    fn cast_poison(&self) -> Option<Self> {
        if self.poison >= POISON_TURNS {
            let mut game = self.use_mana(173)?;
            game.poison = 0;
            Some(game)
        } else {
            None
        }
    }

    fn cast_recharge(&self) -> Option<Self> {
        if self.recharge >= RECHARGE_TURNS {
            let mut game = self.use_mana(229)?;
            game.recharge = 0;
            Some(game)
        } else {
            None
        }
    }
}

fn find_best_mana_path(game: Game) -> u32 {
    let mut games = PriorityQueue::new();
    // Don't Reverse the mana_history value, late game first works better for calculating our results
    games.push(game, game.mana_history);
    let mut best_mana = u32::MAX;
    while let Some((game, _)) = games.pop() {
        if game.mana_history >= best_mana {
            // Don't need to chase down worse options
            continue;
        }
        let game = game.drain_health();
        if game.player_health == 0 {
            continue;
        }
        let game = game.start_turn(); // Start Player Turn
        if game.boss_health == 0 {
            if game.mana_history < best_mana {
                best_mana = game.mana_history;
            }
            continue;
        }
        let possible_games = [
            // Do Player Action
            game.cast_magic_missile(),
            game.cast_drain(),
            game.cast_shield(),
            game.cast_poison(),
            game.cast_recharge(),
        ];
        for game in possible_games {
            if let Some(game) = game {
                if game.player_mana == 0 {
                    continue;
                }
                if game.boss_health == 0 {
                    if game.mana_history < best_mana {
                        best_mana = game.mana_history;
                    }
                    continue;
                }
                let mut game = game.end_turn(); // End Player Turn
                game = game.start_turn(); // Start Boss Turn
                if game.boss_health == 0 {
                    if game.mana_history < best_mana {
                        best_mana = game.mana_history;
                    }
                    continue;
                }
                game = game.damage_player();
                if game.player_health == 0 {
                    continue;
                }
                game = game.end_turn(); // End Boss Turn
                games.push(game, game.mana_history);
            }
        }
    }
    best_mana
}

impl Day for Day22 {
    fn main() -> Result<()> {
        let input_str = get_input(2015, 22)?;
        let mut game = Game::new();
        for line in input_str.lines() {
            if let Some((_, damage)) = regex_captures!("Damage: ([0-9]+)", &line) {
                game.boss_damage = damage.parse()?;
            }
            if let Some((_, health)) = regex_captures!("Hit Points: ([0-9]+)", &line) {
                game.boss_health = health.parse()?;
            }
        }
        let best_mana = find_best_mana_path(game);
        if best_mana == u32::MAX {
            println!("Kobayashi Maru!");
        }
        println!("Best mana to win: {best_mana}");
        game.drain = 1;
        let best_mana = find_best_mana_path(game);
        if best_mana == u32::MAX {
            println!("Kobayashi Maru!");
        }
        println!("Best mana to win with drain: {best_mana}");
        Ok(())
    }
}
