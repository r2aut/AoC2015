//! Day 22: Wizard Simulator 20XX
//!
//! <https://adventofcode.com/2015/day/22>

use anyhow::{Result, anyhow};
use aoc2015::{
    AOCPrint, P1, P2,
    day22::{
        fighters::{Battle, Boss, Outcome, Player},
        magic::Magic,
    },
    get_reader,
};
use std::{cmp::min, io::BufRead};

/// Read boss characteristics from file
fn read_boss(reader: &mut impl BufRead) -> Result<Boss> {
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    let hit_points = buf.trim().split(':').map(|s| s.trim()).nth(1).ok_or(anyhow!("Parcing error"))?.parse::<i32>()?;
    buf.clear();
    reader.read_line(&mut buf)?;
    let damage = buf.trim().split(':').map(|s| s.trim()).nth(1).ok_or(anyhow!("Parcing error"))?.parse::<i32>()?;
    Ok(Boss::new(hit_points, damage))
}

/// Solve part one
fn part_one<F>(player: &Player, boss: &Boss, battle_num: usize, strategy: &mut F) -> i32
where
    F: FnMut(&Player, &Boss) -> Option<Magic>,
{
    let mut mana = i32::MAX;
    for _ in 0..battle_num {
        let mut battle = Battle::new(player, boss);

        if battle.fight(strategy, false) == Outcome::PlayerWon {
            mana = min(mana, battle.player.spent_mana);
        }
    }
    mana
}

/// Solve part two
fn part_two<F>(player: &Player, boss: &Boss, battle_num: usize, strategy: &mut F) -> i32
where
    F: FnMut(&Player, &Boss) -> Option<Magic>,
{
    let mut mana = i32::MAX;
    for _ in 0..battle_num {
        let mut battle = Battle::new(player, boss);

        if battle.fight(strategy, true) == Outcome::PlayerWon {
            mana = min(mana, battle.player.spent_mana);
        }
    }
    mana
}

fn main() -> Result<()> {
    let mut reader = get_reader("puzzles/day22.txt")?;
    let boss = read_boss(&mut reader)?;

    let player = Player::new(50, 500);
    let battle_num = 100000;

    let res1 = part_one(&player, &boss, battle_num, &mut Player::get_random_magic);
    res1.aoc_print(P1);
    let res2 = part_two(&player, &boss, battle_num, &mut Player::get_random_magic);
    res2.aoc_print(P2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2015::day22::magic::*;

    struct DetermenedMagic {
        magic: Vec<Magic>,
        next_magic: usize,
    }
    impl DetermenedMagic {
        fn new(magic: &[Magic]) -> Self {
            Self { magic: magic.to_vec(), next_magic: 0 }
        }

        fn get_determened_magic(&mut self, _player: &Player, _boss: &Boss) -> Option<Magic> {
            if self.next_magic < self.magic.len() {
                let cur_magic = self.next_magic;
                self.next_magic += 1;
                Some(self.magic[cur_magic].clone())
            } else {
                None
            }
        }
    }

    #[test]
    fn test_part_one_1() {
        let player = Player::new(10, 250);
        let boss = Boss::new(13, 8);
        let mut det = DetermenedMagic::new(&[POISON, MISSILE]);
        let res1 = part_one(&player, &boss, 1, &mut |player, boss| det.get_determened_magic(player, boss));
        assert_eq!(res1, 226);
    }
    #[test]
    fn test_part_one_2() {
        let player = Player::new(10, 250);
        let boss = Boss::new(14, 8);
        let mut det = DetermenedMagic::new(&[RECHARGE, SHIELD, DRAIN, POISON, MISSILE]);
        let res1 = part_one(&player, &boss, 1, &mut |player, boss| det.get_determened_magic(player, boss));
        assert_eq!(res1, 641);
    }
}
