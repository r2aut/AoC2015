//! Day 9: All in a Single Night
//!
//! <https://adventofcode.com/2015/day/9>

use aoc2015::{P1, P2};
use colored::Colorize;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type City = String;

/// All cities with the distances between them
#[derive(Debug)]
struct Cities {
    all_cities: Vec<City>,
    all_distances: HashMap<(City, City), u32>,
}

impl Cities {
    fn new() -> Self {
        Cities {
            all_cities: Vec::new(),
            all_distances: HashMap::new(),
        }
    }

    fn add_city(&mut self, city: &str) {
        let city = city.to_string();
        if !self.all_cities.contains(&city) {
            self.all_cities.push(city);
        }
    }
    fn add_distance(&mut self, city1: &str, city2: &str, dist: u32) {
        self.add_city(city1);
        self.add_city(city2);
        let c1 = self.all_cities.iter().find(|i| **i == city1).unwrap().clone();
        let c2 = self.all_cities.iter().find(|i| **i == city2).unwrap().clone();

        let cc = (c1, c2);
        self.all_distances.entry(cc).insert_entry(dist);
    }

    /// Slice over all cities
    fn cities(&self) -> &[City] {
        &self.all_cities[..]
    }

    /// Calc total distance if moving on sequence of cities
    fn distance(&self, path: Vec<&City>) -> Option<u32> {
        path.windows(2)
            .map(|p| {
                self.all_distances
                    .get(&(p[0].clone(), p[1].clone()))
                    .or(self.all_distances.get(&(p[1].clone(), p[0].clone())))
            })
            .sum()
    }
}

/// Read data from file
fn read_city_distances(file_name: &str) -> Cities {
    let mut res = Cities::new();

    // pattern for "London to Dublin = 464"
    let pattern = Regex::new(r"(\w*) to (\w*) = (\d*)").unwrap();

    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let cap = pattern.captures(&line).unwrap();
        let city1 = &cap[1];
        let city2 = &cap[2];
        let dist: u32 = cap[3].parse().unwrap();
        res.add_distance(city1, city2, dist);
    }
    res
}

/// Solution for part one
fn part_one(cities: &Cities) -> Option<u32> {
    cities
        .cities()
        .into_iter()
        .permutations(cities.all_cities.len())
        .map(|v| cities.distance(v))
        .min()
        .flatten()
}

/// Solution for part two
fn part_two(cities: &Cities) -> Option<u32> {
    cities
        .cities()
        .iter()
        .permutations(cities.all_cities.len())
        .map(|v| cities.distance(v))
        .max()
        .flatten()
}

fn main() {
    let cities = read_city_distances(r"puzzles/day09.txt");

    let res_1 = part_one(&cities).unwrap();
    println!("{} The sortest distance is {}", P1.green(), res_1.to_string().green());

    let res_2 = part_two(&cities).unwrap();
    println!("{} The longest distance is {}", P2.green(), res_2.to_string().green());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prepare_data() -> Cities {
        let mut cities = Cities::new();
        cities.add_distance("London", "Dublin", 464);
        cities.add_distance("London", "Belfast", 518);
        cities.add_distance("Dublin", "Belfast", 141);
        cities
    }

    #[test]
    fn test_part_one() {
        let cities = prepare_data();
        assert_eq!(part_one(&cities), Some(605));
    }

    #[test]
    fn test_part_two() {
        let cities = prepare_data();
        assert_eq!(part_two(&cities), Some(982));
    }
}
