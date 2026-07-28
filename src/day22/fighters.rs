//! Day 22: Wizard Simulator 20XX
//! Fighters lib

use crate::day22::fighters::Outcome::NotEnoughMana;
use crate::day22::magic::{ALL_MAGIC, Magic};
use rand::seq::IteratorRandom;
use std::collections::HashSet;
use std::mem::discriminant;
use tracing::debug;

/// Player implementation
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Player {
    hit_points: i32,
    mana: i32,
    armor: i32,
    magic: Vec<Magic>,
    effects: Vec<Magic>,
    pub spent_mana: i32,
}
impl Player {
    pub fn new(hit_points: i32, mana: i32) -> Self {
        Self {
            hit_points,
            mana,
            armor: 0,
            magic: ALL_MAGIC.to_vec(),
            effects: Vec::new(),
            spent_mana: 0,
        }
    }
    pub fn is_alive(&self) -> bool {
        self.hit_points > 0
    }
    // Make one turn (wait effects and cast magic if possible)
    fn make_turn<F>(&mut self, boss: &mut Boss, strategy: &mut F, hard: bool) -> Option<Outcome>
    where
        F: FnMut(&Player, &Boss) -> Option<Magic>,
    {
        debug!("");
        debug!("-- Player turn --");
        debug!(
            "- Player has {} hit points, {} armor, {} mana",
            self.hit_points, self.armor, self.mana
        );
        debug!("- Boss has {} hit points", boss.hit_points);

        // part two varisnt (hard = true)
        if hard {
            self.hit_points -= 1; // part two
        }

        // check if player still alive after decrement (part two)
        // in part one it is alwais true
        let state = Outcome::get_outcome(self, boss);
        if state.is_none() {
            self.apply_effects();
            boss.apply_effects();
        } else {
            return state; // player is dead ofter decrement
        }

        // check state before casting magic (maybe someone is already dead)
        let state = Outcome::get_outcome(self, boss);
        if state.is_none() {
            // get magic that is allowd
            if let Some(magic) = strategy(self, boss) {
                // cast magic
                self.cast_magic(boss, magic)
            } else {
                Some(NotEnoughMana) // player has lost
            }
        } else {
            state // after applying effects someone won
        }
    }
    /// Apply effects that works on Player
    fn apply_effects(&mut self) {
        // armor should act in the tune when it wears off
        self.armor = 0;

        for effect in &mut self.effects {
            match effect {
                Magic::Shield {
                    rearmoring,
                    duration,
                    ..
                } => {
                    self.armor = *rearmoring;
                    *duration -= 1;
                    debug!("Shield's timer is now {}", duration);
                }
                Magic::Recharge { mana, duration, .. } => {
                    self.mana += *mana;
                    *duration -= 1;
                    debug!(
                        "Recharge provides {} mana; its timer is now {}",
                        mana, duration
                    );
                }
                _ => unreachable!(),
            }
        }
        // delete effects that wears off
        self.effects.retain(|effect| match effect {
            Magic::Shield { duration, .. } => *duration > 0,
            Magic::Recharge { duration, .. } => *duration > 0,
            _ => true, // Если появятся эффекты без длительности, они не удалятся
        });
    }

    /// Cast magic on boss
    fn cast_magic(&mut self, boss: &mut Boss, magic: Magic) -> Option<Outcome> {
        match magic {
            Magic::Missile { cost, damage } => {
                self.mana -= cost;
                self.spent_mana += cost;
                boss.get_damage(damage);
                debug!("Player casts Magic Missile, dealing {} damage", damage);
            }
            Magic::Drain {
                cost,
                damage,
                healing,
            } => {
                self.mana -= cost;
                self.spent_mana += cost;
                boss.get_damage(damage);
                self.hit_points += healing;
                debug!(
                    "Player casts Drain, dealing {} damage, and healing {} hit points",
                    damage, healing
                );
            }
            Magic::Shield {
                cost, rearmoring, ..
            } => {
                self.mana -= cost;
                self.spent_mana += cost;
                self.effects.push(magic);
                debug!("Player casts Shield, increasing armor by {}", rearmoring);
            }
            Magic::Poison { cost, .. } => {
                self.mana -= cost;
                self.spent_mana += cost;
                boss.effects.push(magic);
                debug!("Player casts Poison");
            }
            Magic::Recharge { cost, .. } => {
                self.mana -= cost;
                self.spent_mana += cost;
                self.effects.push(magic);
                debug!("Player casts Recharge");
            }
        }

        // check if someone is dead after magic cast
        let state = Outcome::get_outcome(self, boss);
        if let Some(outcome) = state
            && outcome == Outcome::PlayerWon
        {
            debug!("This kills the boss, and the player wins.");
        }
        state
    }
    /// Get random magic that is not in use at the moment an has appropriate cost
    pub fn get_random_magic(&self, boss: &Boss) -> Option<Magic> {
        let magic_in_use1: HashSet<_> = self.effects.iter().map(discriminant).collect();
        let magic_in_use2: HashSet<_> = boss.effects.iter().map(discriminant).collect();
        let mut rng = rand::rng(); // move

        // find the magic that can be used
        self.magic
            .iter()
            .filter(|mag| {
                mag.cost() <= self.mana // the player has enough mana to cats
                    && !magic_in_use1.contains(&discriminant(*mag)) // in use at the player
                    && !magic_in_use2.contains(&discriminant(*mag)) // in use at the boss
            })
            .choose(&mut rng) // make random choise
            .cloned()
        // None means that it's not affordable magic
    }
    /// Register damage on player
    fn get_damage(&mut self, loss: i32) {
        self.hit_points -= loss;
    }
}

/// The Boss implementation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Boss {
    hit_points: i32,
    damage: i32,
    effects: Vec<Magic>,
}
impl Boss {
    pub fn new(hit_points: i32, damage: i32) -> Self {
        Self {
            hit_points,
            damage,
            effects: Vec::new(),
        }
    }
    pub fn is_alive(&self) -> bool {
        self.hit_points > 0
    }
    // Make one turn (wait effects and cast magic if possible)
    fn make_turn(&mut self, player: &mut Player) -> Option<Outcome> {
        debug!("");
        debug!("-- Boss turn --");
        debug!(
            "- Player has {} hit points, {} armor, {} mana",
            player.hit_points, player.armor, player.mana
        );
        debug!("- Boss has {} hit points", self.hit_points);

        // Apply all effects
        self.apply_effects();
        player.apply_effects();

        // Check if someone is dead after applying effects
        let status = Outcome::get_outcome(player, self);
        // everyone still alive
        if status.is_none() {
            // attack the player
            self.attack(player);
        }
        // Check if someone is dead after attack
        Outcome::get_outcome(player, self)
    }
    /// Applying effects that on boss side
    fn apply_effects(&mut self) {
        for effect in &mut self.effects {
            match effect {
                Magic::Poison {
                    damage, duration, ..
                } => {
                    self.hit_points -= *damage;
                    *duration -= 1;
                    debug!(
                        "Poison deals {} damage; its timer is now {}",
                        damage, duration
                    );
                    if self.hit_points <= 0 {
                        debug!("This kills the boss, and the player wins.")
                    }
                }
                _ => unreachable!(),
            }
        }
        // delete all effects that wears off
        self.effects.retain(|effect| match effect {
            Magic::Poison { duration, .. } => *duration > 0,
            _ => true, // Если появятся эффекты без длительности, они не удалятся
        });
    }
    /// Attack the player
    fn attack(&self, player: &mut Player) {
        let loss = if self.damage > player.armor {
            self.damage - player.armor
        } else {
            1
        };
        player.get_damage(loss);
        debug!("Boss attacks for {} damage", loss);
        if player.hit_points <= 0 {
            debug!("This kills the player, and the boss wins.")
        }
    }
    /// Register damage on the boss side
    fn get_damage(&mut self, loss: i32) {
        self.hit_points -= loss;
    }
}

/// The outcome of the battle
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
#[repr(u8)]
pub enum Outcome {
    PlayerWon,
    PlayerLost,
    NotEnoughMana,
}
impl Outcome {
    /// Calc if the outcome is acheived (None = all alive)
    pub fn get_outcome(player: &Player, boss: &Boss) -> Option<Outcome> {
        match (player.is_alive(), boss.is_alive()) {
            (true, true) => None,
            (true, false) => Some(Outcome::PlayerWon),
            (false, _) => Some(Outcome::PlayerLost),
        }
    }
}

/// Battle implementation
pub struct Battle {
    pub player: Player,
    pub boss: Boss,
}
impl Battle {
    pub fn new(player: &Player, boss: &Boss) -> Self {
        Self {
            player: player.clone(),
            boss: boss.clone(),
        }
    }
    /// Fight until someone is dead
    pub fn fight<F>(&mut self, strategy: &mut F, hard: bool) -> Outcome
    where
        F: FnMut(&Player, &Boss) -> Option<Magic>,
    {
        loop {
            if let Some(outcome) = self.player.make_turn(&mut self.boss, strategy, hard) {
                break outcome;
            }
            if let Some(outcome) = self.boss.make_turn(&mut self.player) {
                break outcome;
            };
        }
    }
}
