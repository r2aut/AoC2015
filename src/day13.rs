//! Day 13: Knights of the Dinner Table
//!
//! <https://adventofcode.com/2015/day/13>

use anyhow::{Result, bail};
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, io::BufRead};

type Guests = HashMap<String, HashMap<String, i32>>;

fn read_guests(reader: impl BufRead) -> Result<Guests> {
    let mut guests = Guests::default();
    let pattern = Regex::new(r"(\w*) would (\w*) (\d*) happiness units by sitting next to (\w*).")?;
    for line in reader.lines() {
        if let Some(cap) = pattern.captures(&line?) {
            let guest_name = cap[1].to_string();
            let raiting: i32 = match &cap[2] {
                "gain" => cap[3].parse()?,
                "lose" => -cap[3].parse()?,
                _ => bail!("wrong word"),
            };
            let neighbour = cap[4].to_string();

            let f = guests.entry(guest_name).or_default();
            f.insert(neighbour, raiting);
        }
    }
    Ok(guests)
}

fn get_permutation_raiting(guests: &Guests, perm: &Vec<&String>) -> i32 {
    let mut cnt = 0;
    for (a, b) in perm.iter().circular_tuple_windows() {
        cnt += guests.get(*a).and_then(|m| m.get(*b)).unwrap_or(&0);
        cnt += guests.get(*b).and_then(|m| m.get(*a)).unwrap_or(&0);
    }
    cnt
}

fn calc_best_happiness_change(guests: &Guests) -> i32 {
    let mut cnt: Vec<i32> = Vec::new();
    for perm in guests.keys().permutations(guests.len()) {
        cnt.push(get_permutation_raiting(guests, &perm));
    }
    *cnt.iter().max().unwrap_or(&0)
}

pub fn solve(reader: impl BufRead) -> Result<(Option<String>, Option<String>)> {
    let mut guests = read_guests(reader)?;
    let res1 = Some(calc_best_happiness_change(&guests).to_string());

    guests.insert("Me".to_string(), HashMap::new());
    let res2 = Some(calc_best_happiness_change(&guests).to_string());

    Ok((res1, res2))
}

#[cfg(test)]
mod test {
    use super::*;
    const DATA: &str = "\
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
";

    #[test]
    fn test_part_one() {
        let guests = read_guests(DATA.as_bytes()).unwrap();
        assert_eq!(calc_best_happiness_change(&guests), 330);
    }
    #[test]
    fn test_part_two() {
        let mut guests = read_guests(DATA.as_bytes()).unwrap();
        guests.insert("Me".to_string(), HashMap::new());
        assert_eq!(calc_best_happiness_change(&guests), 286);
    }
}
