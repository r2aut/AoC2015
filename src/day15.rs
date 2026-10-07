//! Day 15: Science for Hungry People
//!
//! <https://adventofcode.com/2015/day/15>

use anyhow::Result;
use regex::Regex;
use std::{
    cmp::{
        Ordering::{Equal, Greater, Less},
        max,
    },
    io::BufRead,
};

#[derive(Debug, Clone)]
struct Ingredient {
    _name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

struct IngredientsParser;
impl IngredientsParser {
    fn try_from_aoc_reader(reader: impl BufRead) -> Result<Vec<Ingredient>> {
        let pattern = Regex::new(
            r"(\w*): capacity (-?\d*), durability (-?\d*), flavor (-?\d*), texture (-?\d*), calories (-?\d*)",
        )?;
        let mut ingredients = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if let Some(caps) = pattern.captures(&line) {
                let name = caps[1].to_string();
                let capacity = caps[2].parse()?;
                let durability = caps[3].parse()?;
                let flavor = caps[4].parse()?;
                let texture = caps[5].parse()?;
                let calories = caps[6].parse()?;

                let ingredient = Ingredient { _name: name, capacity, durability, flavor, texture, calories };
                ingredients.push(ingredient);
            }
        }
        Ok(ingredients)
    }
}

struct Recipe<'a> {
    ingredients: &'a Vec<Ingredient>,
    quantity: &'a Vec<i32>,
}
impl<'a> Recipe<'a> {
    fn score(&self) -> i32 {
        let pairs: Vec<(&Ingredient, &i32)> = self.ingredients.iter().zip(self.quantity).collect();

        let total_capacity = max(pairs.iter().map(|(ing, q)| ing.capacity * *q).sum::<i32>(), 0);
        let total_durability = max(pairs.iter().map(|(ing, q)| ing.durability * *q).sum::<i32>(), 0);
        let total_flavor = max(pairs.iter().map(|(ing, q)| ing.flavor * *q).sum::<i32>(), 0);
        let total_texture = max(pairs.iter().map(|(ing, q)| ing.texture * *q).sum::<i32>(), 0);

        total_capacity * total_durability * total_flavor * total_texture
    }

    fn calories(&self) -> i32 {
        self.ingredients.iter().zip(self.quantity).map(|(ing, q)| ing.calories * *q).sum::<i32>()
    }
}

struct RecipeIter {
    cells: Vec<i32>,
    total: i32,
    finished: bool,
}
impl RecipeIter {
    fn new(size: usize, total: i32) -> Self {
        Self { cells: vec![0; size], total, finished: false }
    }
    fn increase_number(&mut self, pos: usize) -> Option<Vec<i32>> {
        if self.finished || pos >= self.cells.len() {
            self.finished = true;
            return None;
        }
        self.cells[pos] += 1;
        let mut total_sum = self.cells.iter().sum::<i32>();
        while self.total != total_sum {
            match total_sum.cmp(&self.total) {
                Less => {
                    let inc = self.total - total_sum;
                    self.cells[0] += inc;
                }
                Equal => {
                    return Some(self.cells.clone());
                }
                Greater => {
                    self.cells[pos] = 0;
                    return self.increase_number(pos + 1);
                }
            }
            total_sum = self.cells.iter().sum::<i32>();
        }
        Some(self.cells.clone())
    }
}
impl Iterator for RecipeIter {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        self.increase_number(0)
    }
}

fn part_one(ingredients: &Vec<Ingredient>) -> Option<i32> {
    let mut max_score: Option<i32> = None;
    for rec in RecipeIter::new(ingredients.len(), 100) {
        let recipe = Recipe { ingredients, quantity: &rec };
        max_score = Some(max(max_score.unwrap_or(0), recipe.score()));
    }
    max_score
}

fn part_two(ingredients: &Vec<Ingredient>) -> Option<i32> {
    let mut max_score: Option<i32> = None;
    for rec in RecipeIter::new(ingredients.len(), 100) {
        let recipe = Recipe { ingredients, quantity: &rec };
        if recipe.calories() == 500 {
            max_score = Some(max(max_score.unwrap_or(0), recipe.score()));
        }
    }
    max_score
}

pub fn solve(reader: impl BufRead) -> Result<(Option<String>, Option<String>)> {
    let ingredients = IngredientsParser::try_from_aoc_reader(reader)?;
    let res1 = part_one(&ingredients).map(|r| r.to_string());
    let res2 = part_two(&ingredients).map(|r| r.to_string());
    Ok((res1, res2))
}
// fn main() -> Result<()> {
//     let reader = aoc2015::get_reader("puzzles/day15.txt")?;
//     let ingredients = IngredientsParser::try_from_aoc_reader(reader)?;

//     let t0 = std::time::Instant::now();

//     let res1 = part_one(&ingredients);
//     println!("{:?}", res1);
//     let res2 = part_two(&ingredients);
//     println!("{:?}", res2);

//     let dur = t0.elapsed().as_millis();
//     println!("{}", dur);

//     Ok(())
// }

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "\
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
";

    #[test]
    fn test_part_one() {
        let ingredients = IngredientsParser::try_from_aoc_reader(DATA.as_bytes()).unwrap();
        assert_eq!(part_one(&ingredients).unwrap(), 62842880);
    }

    #[test]
    fn test_part_two() {
        let ingredients = IngredientsParser::try_from_aoc_reader(DATA.as_bytes()).unwrap();
        assert_eq!(part_two(&ingredients).unwrap(), 57600000);
    }
}
