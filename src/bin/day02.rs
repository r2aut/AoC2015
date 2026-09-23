//! Day 2: I Was Told There Would Be No Math
//!
//! <https://adventofcode.com/2015/day/2>

use anyhow::{Ok, Result};
use aoc2015::{AOCPrint, P1, P2, get_reader, print_day};
use itertools::Itertools;
use regex::Regex;
use std::io::BufRead;

#[derive(Debug)]
struct Present {
    dims: Vec<i32>,
}

impl Present {
    fn squares(&self) -> Vec<i32> {
        let comb: Vec<Vec<&i32>> = self.dims.iter().combinations(2).collect();
        let squares: Vec<i32> = comb.into_iter().map(|item: Vec<&i32>| -> i32 { item[0] * item[1] }).collect();
        squares
    }
    fn perimeters(&self) -> Vec<i32> {
        let comb: Vec<Vec<&i32>> = self.dims.iter().combinations(2).collect();
        let perimeters: Vec<i32> = comb.into_iter().map(|item: Vec<&i32>| -> i32 { (item[0] + item[1]) * 2 }).collect();
        perimeters
    }
    fn volume(&self) -> i32 {
        self.dims.iter().product()
    }
}

fn read_data(reader: impl BufRead) -> Result<Vec<Present>> {
    let re = Regex::new(r"(\d*)x(\d*)x(\d*)")?;
    let mut result: Vec<Present> = Vec::new();
    for line_ in reader.lines() {
        let line = line_?;
        if let Some(caps) = re.captures(&line) {
            let c1 = caps[1].parse::<i32>()?;
            let c2 = caps[2].parse::<i32>()?;
            let c3 = caps[3].parse::<i32>()?;
            let present = Present { dims: vec![c1, c2, c3] };
            result.push(present);
        }
    }
    Ok(result)
}

fn part_one(data: &Vec<Present>) -> i32 {
    let mut counter = 0;
    for present in data {
        let present_sqares = present.squares();
        let square_sum = present_sqares.iter().sum::<i32>() * 2;
        if let Some(min_square) = present_sqares.iter().min() {
            counter += square_sum + min_square;
        }
    }
    counter
}

fn part_two(data: &Vec<Present>) -> i32 {
    let mut counter = 0;
    for present in data {
        let present_perimeters = present.perimeters();
        if let Some(min_perimetr) = present_perimeters.iter().min() {
            let volume = present.volume();
            counter += min_perimetr + volume;
        }
    }
    counter
}

fn main() -> Result<()> {
    let reader = get_reader(r"puzzles/day02.txt")?;
    let data = read_data(reader)?;
    print_day();
    part_one(&data).aoc_print(P1);
    part_two(&data).aoc_print(P2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_vec = vec![Present { dims: vec![2, 3, 4] }, Present { dims: vec![1, 1, 10] }];
        assert_eq!(part_one(&test_vec), 101)
    }

    #[test]
    fn test_part_two() {
        let test_vec = vec![Present { dims: vec![2, 3, 4] }, Present { dims: vec![1, 1, 10] }];
        assert_eq!(part_two(&test_vec), 48)
    }
}
