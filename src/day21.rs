//! Day 21: RPG Simulator 20XX
//!
//! <https://adventofcode.com/2015/day/21>

pub mod depot;
pub mod fighter;

use crate::day21::fighter::BossParser;
use anyhow::Result;
use depot::{Depot, ItemFactory};
use fighter::{Battle, Boss, Player};
use std::{
    cmp::{max, min},
    io::BufRead,
};

/// Solve part one
fn part_one(boss: Boss, hit_points: i32, depot: &mut Depot) -> i32 {
    let number = 1000000;
    let mut min_gold = i32::MAX;
    for n in 0..number {
        let player = Player::new_random(format!("Player{n}"), hit_points, depot);
        let mut battle = Battle::new(&player, &boss);
        battle.fight();
        if battle.player.is_alive() {
            let cost = player.spent_gold;
            min_gold = min(min_gold, cost)
        }
    }
    min_gold
}

/// Solve part two
fn part_two(boss: Boss, hit_points: i32, depot: &mut Depot) -> i32 {
    let number = 1000000;
    let mut max_gold = 0;
    for n in 0..number {
        let player = Player::new_random(format!("Player{n}"), hit_points, depot);
        let mut battle = Battle::new(&player, &boss);
        battle.fight();
        if !battle.player.is_alive() {
            let cost = player.spent_gold;
            max_gold = max(max_gold, cost)
        }
    }
    max_gold
}

pub fn solve(reader: impl BufRead) -> Result<(Option<String>, Option<String>)> {
    let mut depot = Depot::new();
    ItemFactory::fill_depot(&mut depot);
    let boss = BossParser::try_from_aoc_reader(reader)?;

    let res1 = Some(part_one(boss.clone(), 100, &mut depot).to_string());
    let res2 = Some(part_two(boss, 100, &mut depot).to_string());
    Ok((res1, res2))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day21() {
        let player = Player::new("Test player".to_string(), 8, 5, 5);
        let boss = Boss::new(12, 7, 2);
        let mut battle = Battle::new(&player, &boss);
        battle.fight();
        assert!(battle.player.is_alive());
        assert_eq!(battle.player.get_hit_points(), 2);
    }
}
