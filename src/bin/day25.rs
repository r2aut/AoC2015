//! Day 25: Let It Snow
//!
//! <https://adventofcode.com/2015/day/25>

use anyhow::{Result, anyhow};
use aoc2015::P1;
use colored::Colorize;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

/// Number that starts the sequence
#[derive(Debug, Clone, Copy)]
struct Code(u128); // y, x, value

impl IntoIterator for Code {
    type Item = u128;

    type IntoIter = CodeGenerator;

    fn into_iter(self) -> Self::IntoIter {
        CodeGenerator {
            row: 1,
            col: 1,
            value: self.0,
            is_first: true,
        }
    }
}

struct CodeGenerator {
    row: usize,
    col: usize,
    value: u128,
    is_first: bool,
}
impl Iterator for CodeGenerator {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
            return Some(self.value);
        }

        if self.row == 1 {
            self.row = self.col + 1;
            self.col = 1;
        } else {
            self.row -= 1;
            self.col += 1;
        }
        self.value = (self.value * 252533) % 33554393;
        Some(self.value)
    }
}

/// Calculate item number in sequence (zero-based)
fn number_in_sequence(row: usize, col: usize) -> usize {
    if row == 0 || col == 0 {
        panic!("All coordinates start from 1");
    }
    let diag = col + row - 1; // current diagonal
    let items_num = diag * (diag - 1) / 2; // number of items in previous diagonals
    items_num + col - 1 // number of current item in sequense (0-based)
}

/// Solve Part One
fn part_one(num: &Code, row: usize, col: usize) -> u128 {
    let seq_num = number_in_sequence(row, col);
    num.into_iter().nth(seq_num).expect("Should be infinite")
}

/// Read data from file
fn read_data(reader: &mut impl BufRead) -> Result<(usize, usize)> {
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    let pattern = Regex::new(r"Enter the code at row (\d*), column (\d*).")?;
    if let Some(cap) = pattern.captures(&buf) {
        let row: usize = cap[1].parse()?;
        let col: usize = cap[2].parse()?;
        Ok((row, col))
    } else {
        Err(anyhow!("Cannnot find"))
    }
}
fn main() -> Result<()> {
    let first_num = 20151125;
    let num = Code(first_num);

    let path = Path::new("puzzles/day25.txt");
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let (row, col) = read_data(&mut reader)?;

    let res1 = part_one(&num, row, col);
    println!("{} {}", P1.blue(), res1.to_string().green());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const START_NUM: u128 = 20151125;

    #[test]
    fn test_read() {
        let data = "To continue, please consult the code grid in the manual.  Enter the code at row 2981, column 3075.";
        let mut reader = data.as_bytes();
        let res = read_data(&mut reader);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), (2981, 3075));
    }

    #[test]
    #[should_panic]
    fn test_day25_zero_00() {
        let num = Code(START_NUM);
        assert_eq!(part_one(&num, 0, 0), 20151125);
    }
    #[test]
    #[should_panic]
    fn test_day25_zero_01() {
        let num = Code(START_NUM);
        assert_eq!(part_one(&num, 0, 1), 20151125);
    }
    #[test]
    #[should_panic]
    fn test_day25_zero_10() {
        let num = Code(START_NUM);
        assert_eq!(part_one(&num, 1, 0), 20151125);
    }
    #[test]
    fn test_day25() {
        let num = Code(START_NUM);
        assert_eq!(part_one(&num, 1, 1), 20151125);
        assert_eq!(part_one(&num, 1, 5), 10071777);
        assert_eq!(part_one(&num, 2, 4), 7726640);
        assert_eq!(part_one(&num, 3, 3), 1601130);
        assert_eq!(part_one(&num, 6, 1), 33071741);
    }
}
