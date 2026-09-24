use crate::day21::depot::{Armor, Depot, Ring, Weapon};
use anyhow::Result;
use std::{
    fmt::{Debug, Display},
    io::BufRead,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    name: String,
    hit_points: i32,
    attack_level: i32,
    defence_level: i32,
    pub spent_gold: i32,
    weapon: Weapon,
    armor: Armor,
    rings: Vec<Ring>,
}
impl Player {
    #[allow(dead_code)]
    pub fn new(name: String, hit_points: i32, attack_level: i32, defence_level: i32) -> Self {
        Self {
            name,
            hit_points,
            attack_level,
            defence_level,
            spent_gold: 0,
            weapon: Weapon::default(),
            armor: Armor::default(),
            rings: Vec::new(),
        }
    }

    pub fn new_with_weapon(name: String, hit_points: i32, weapon: Weapon, armor: Armor, rings: Vec<Ring>) -> Self {
        let attack_level = weapon.damage + rings.iter().map(|r| r.damage).sum::<i32>();
        let defence_level = armor.armor + rings.iter().map(|r| r.armor).sum::<i32>();
        let spent_gold = weapon.cost + armor.cost + rings.iter().map(|r| r.cost).sum::<i32>();
        Self { name, hit_points, attack_level, defence_level, spent_gold, weapon, armor, rings }
    }
    pub fn new_random(name: String, hit_points: i32, depot: &mut Depot) -> Self {
        Self::new_with_weapon(
            name,
            hit_points,
            depot.get_random_weapon(),
            depot.get_random_armor(),
            depot.get_random_rings(),
        )
    }
    #[allow(dead_code)]
    pub fn get_hit_points(&self) -> i32 {
        self.hit_points
    }

    pub fn is_alive(&self) -> bool {
        self.hit_points > 0
    }
    fn attacks(&self, boss: &mut Boss) {
        let al = self.attack_level;
        let dl = boss.defence_level;
        let loss = if al > dl { al - dl } else { 1 };
        boss.lose_hit_point(loss);
    }
    fn lose_hit_point(&mut self, loss: i32) {
        self.hit_points -= loss;
    }
}
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boss {
    name: String,
    hit_points: i32,
    attack_level: i32,
    defence_level: i32,
}
impl Boss {
    pub fn new(hit_points: i32, attack_level: i32, defence_level: i32) -> Self {
        Self { name: "Boss".to_string(), hit_points, attack_level, defence_level }
    }
    pub fn is_alive(&self) -> bool {
        self.hit_points > 0
    }
    fn attacks(&self, player: &mut Player) {
        let al = self.attack_level;
        let dl = player.defence_level;
        let loss = if al > dl { al - dl } else { 1 };
        player.lose_hit_point(loss);
    }
    fn lose_hit_point(&mut self, loss: i32) {
        self.hit_points -= loss;
    }
}
impl Display for Boss {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}

pub struct Battle {
    pub player: Player,
    pub boss: Boss,
}
impl Battle {
    pub fn new(player: &Player, boss: &Boss) -> Self {
        Self { player: player.clone(), boss: boss.clone() }
    }
    pub fn fight(&mut self) {
        while self.player.is_alive() && self.boss.is_alive() {
            // fighter1 attacks
            self.player.attacks(&mut self.boss);
            if !self.boss.is_alive() {
                break;
            }
            // fighter2 attacks
            self.boss.attacks(&mut self.player);
        }
    }
}

pub struct BossParser;
impl BossParser {
    pub fn try_from_aoc_reader(reader: impl BufRead) -> Result<Boss> {
        let mut hit_points: i32 = 0;
        let mut damage: i32 = 0;
        let mut armor: i32 = 0;
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<_> = line.split(':').collect();
            match parts[0].trim() {
                "Hit Points" => hit_points = parts[1].trim().parse()?,
                "Damage" => damage = parts[1].trim().parse()?,
                "Armor" => armor = parts[1].trim().parse()?,
                _ => panic!("Wrong data in input file: {}", line),
            }
        }
        Ok(Boss::new(hit_points, damage, armor))
    }
}
