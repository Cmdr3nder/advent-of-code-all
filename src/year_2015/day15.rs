use std::ops::{Add, Mul};

use anyhow::{Context, Result};
use lazy_regex::regex_captures;

use crate::day::Day;
use crate::input::get_input;

pub struct Day15;

#[derive(Debug, Clone, Copy, Default)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Mul<i32> for Ingredient {
    type Output = Ingredient;

    fn mul(self, other: i32) -> Ingredient {
        Ingredient {
            capacity: self.capacity * other,
            durability: self.durability * other,
            flavor: self.flavor * other,
            texture: self.texture * other,
            calories: self.calories * other,
        }
    }
}

impl Add<Ingredient> for Ingredient {
    type Output = Ingredient;

    fn add(self, other: Ingredient) -> Ingredient {
        Ingredient {
            capacity: self.capacity + other.capacity,
            durability: self.durability + other.durability,
            flavor: self.flavor + other.flavor,
            texture: self.texture + other.texture,
            calories: self.calories + other.calories,
        }
    }
}

fn simple_score(cookie: Ingredient) -> i32 {
    if cookie.capacity > 0 && cookie.durability > 0 && cookie.flavor > 0 && cookie.texture > 0 {
        cookie.capacity * cookie.durability * cookie.flavor * cookie.texture
    } else {
        0
    }
}

fn calorie_score(cookie: Ingredient) -> i32 {
    if cookie.calories == 500 {
        simple_score(cookie)
    } else {
        0
    }
}

fn best_cookie(
    cookie: Ingredient,
    ingredients: &[Ingredient],
    teaspoons: i32,
    score_fn: &dyn Fn(Ingredient) -> i32,
) -> i32 {
    if ingredients.len() < 1 {
        score_fn(cookie)
    } else if ingredients.len() == 1 {
        score_fn(cookie + (ingredients[0] * teaspoons))
    } else {
        let mut best = 0;
        for t in 0..=teaspoons {
            let next = best_cookie(
                cookie + (ingredients[0] * t),
                &ingredients[1..],
                teaspoons - t,
                score_fn,
            );
            if next > best {
                best = next;
            }
        }
        best
    }
}

impl Day for Day15 {
    fn main() -> Result<()> {
        let input_str = get_input(2015, 15)?;
        let mut ingredients: Vec<Ingredient> = Vec::new();
        for line in input_str.lines() {
            let (_, _name, capacity, durability, flavor, texture, calories) = regex_captures!(
                "([A-Za-z]+): capacity ([-0-9]+), durability ([-0-9]+), flavor ([-0-9]+), texture ([-0-9]+), calories ([-0-9]+)",
                &line,
            ).with_context(|| "Could not parse ingredient stats")?;
            ingredients.push(Ingredient {
                capacity: capacity.parse()?,
                durability: durability.parse()?,
                flavor: flavor.parse()?,
                texture: texture.parse()?,
                calories: calories.parse()?,
            })
        }
        let best_score = best_cookie(Ingredient::default(), &ingredients, 100, &simple_score);
        println!("Calorie ignorant highest-scoring cookie: {best_score}");
        let best_meal_score = best_cookie(Ingredient::default(), &ingredients, 100, &calorie_score);
        println!("Calorie included highest-scoring cookie: {best_meal_score}");
        Ok(())
    }
}
