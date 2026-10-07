//! Day 16: Aunt Sue
//!
//! <https://adventofcode.com/2015/day/16>

use anyhow::{Result, bail};
use std::io::BufRead;

/// Aunt implementation with attributes
#[derive(Debug)]
struct Aunt {
    number: i32,
    children: Option<i32>,
    cats: Option<i32>,
    samoyeds: Option<i32>,
    pomeranians: Option<i32>,
    akitas: Option<i32>,
    vizslas: Option<i32>,
    goldfish: Option<i32>,
    trees: Option<i32>,
    cars: Option<i32>,
    perfumes: Option<i32>,
}
impl Aunt {
    fn new(number: i32) -> Self {
        Self {
            number,
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        }
    }
    /// Check if aunt is compartible according part one conditions
    fn is_compatible(&self, pattern: &Aunt) -> bool {
        !(self.children.is_some() && self.children != pattern.children
            || self.cats.is_some() && self.cats != pattern.cats
            || self.samoyeds.is_some() && self.samoyeds != pattern.samoyeds
            || self.pomeranians.is_some() && self.pomeranians != pattern.pomeranians
            || self.akitas.is_some() && self.akitas != pattern.akitas
            || self.vizslas.is_some() && self.vizslas != pattern.vizslas
            || self.goldfish.is_some() && self.goldfish != pattern.goldfish
            || self.trees.is_some() && self.trees != pattern.trees
            || self.cars.is_some() && self.cars != pattern.cars
            || self.perfumes.is_some() && self.perfumes != pattern.perfumes)
    }
    /// Check if aunt is compartible according part two conditions
    fn is_compatible2(&self, pattern: &Aunt) -> bool {
        !(self.children.is_some() && self.children != pattern.children
            || self.cats.is_some() && self.cats <= pattern.cats
            || self.samoyeds.is_some() && self.samoyeds != pattern.samoyeds
            || self.pomeranians.is_some() && self.pomeranians >= pattern.pomeranians
            || self.akitas.is_some() && self.akitas != pattern.akitas
            || self.vizslas.is_some() && self.vizslas != pattern.vizslas
            || self.goldfish.is_some() && self.goldfish >= pattern.goldfish
            || self.trees.is_some() && self.trees <= pattern.trees
            || self.cars.is_some() && self.cars != pattern.cars
            || self.perfumes.is_some() && self.perfumes != pattern.perfumes)
    }
}

/// Parser for Aunts
struct AuntVecParser;
impl AuntVecParser {
    fn try_from_aoc_reader(reader: impl BufRead) -> Result<Vec<Aunt>> {
        let mut result = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.trim().split(' ').collect();
            let number = parts[1].strip_suffix(':').expect("Every aunt number ends with :").parse()?;
            let mut aunt = Aunt::new(number);
            for i in (2..parts.len()).step_by(2) {
                let attribute = parts[i].strip_suffix(':').expect("Every attribute ends with :");
                let value: i32 = parts[i + 1].strip_suffix(',').unwrap_or(parts[i + 1]).trim().parse()?;
                match attribute {
                    "children" => aunt.children = Some(value),
                    "cats" => aunt.cats = Some(value),
                    "samoyeds" => aunt.samoyeds = Some(value),
                    "pomeranians" => aunt.pomeranians = Some(value),
                    "akitas" => aunt.akitas = Some(value),
                    "vizslas" => aunt.vizslas = Some(value),
                    "goldfish" => aunt.goldfish = Some(value),
                    "trees" => aunt.trees = Some(value),
                    "cars" => aunt.cars = Some(value),
                    "perfumes" => aunt.perfumes = Some(value),
                    _ => bail!("Unknown attribute {}", attribute),
                }
            }
            result.push(aunt);
        }
        Ok(result)
    }
}

/// Solve part one
fn part_one(aunts: &[Aunt], pattern: &Aunt) -> Option<i32> {
    aunts.iter().find(|a| a.is_compatible(pattern)).map(|a| a.number)
}

/// Solve part two
fn part_two(aunts: &[Aunt], pattern: &Aunt) -> Option<i32> {
    aunts.iter().find(|a| a.is_compatible2(pattern)).map(|a| a.number)
}

pub fn solve(reader: impl BufRead) -> Result<(Option<String>, Option<String>)> {
    let aunts = AuntVecParser::try_from_aoc_reader(reader)?;
    let pattern = Aunt {
        number: 0,
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };
    let res1 = part_one(&aunts, &pattern).map(|i| i.to_string());
    let res2 = part_two(&aunts, &pattern).map(|i| i.to_string());
    Ok((res1, res2))
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "\
Sue 1: children: 1, cars: 8, vizslas: 7
Sue 2: akitas: 10, perfumes: 10, children: 5
Sue 3: cars: 5, pomeranians: 4, vizslas: 1
Sue 4: goldfish: 5, children: 3, perfumes: 3
Sue 5: vizslas: 2, akitas: 7, perfumes: 6
Sue 6: cats: 8, samoyeds: 2, trees: 4
";

    const PATTERN: Aunt = Aunt {
        number: 0,
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(3),
    };

    #[test]
    fn test_part_one() {
        let aunts = AuntVecParser::try_from_aoc_reader(DATA.as_bytes()).unwrap();
        assert!(!aunts[1].is_compatible(&PATTERN));
        assert!(aunts[3].is_compatible(&PATTERN));
        assert_eq!(part_one(&aunts, &PATTERN).unwrap(), 4);
    }

    #[test]
    fn test_part_two() {
        let aunts = AuntVecParser::try_from_aoc_reader(DATA.as_bytes()).unwrap();
        assert!(!aunts[1].is_compatible2(&PATTERN));
        assert!(aunts[5].is_compatible2(&PATTERN));
        assert_eq!(part_two(&aunts, &PATTERN).unwrap(), 6);
    }
}
