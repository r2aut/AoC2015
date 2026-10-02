//! Day 20: Infinite Elves and Infinite Houses
//!
//! <https://adventofcode.com/2015/day/20>

use std::{cmp::min, io::BufRead};

use anyhow::Result;

/// Solve part one
fn part_one(target: usize) -> Option<usize> {
    let border: usize = target / 10;
    let mut presents = vec![0; border];

    for elf in 1..border {
        let elf_num = elf + 1;

        for house in (elf_num..border).step_by(elf_num) {
            // house number 0 set without presents, it's out of numeration
            presents[house] += elf_num * 10;
        }
    }

    presents.iter().position(|&v| v >= target)
}

/// Solve part two
fn part_two(target: usize) -> Option<usize> {
    let border: usize = target / 10;
    let mut presents = vec![0; border];

    for elf in 1..border {
        let elf_num = elf + 1;
        let end_house = min(border, elf * 50 + elf + 1);

        for house in (elf_num..end_house).step_by(elf_num) {
            presents[house] += elf_num * 11;
        }
    }

    presents.iter().position(|&v| v >= target).map(|v| v + 1)
}

pub fn solve(mut reader: impl BufRead) -> Result<(Option<String>, Option<String>)> {
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    let target: usize = buf.trim().parse()?;

    let res1 = part_one(target).map(|v| v.to_string());
    let res2 = part_two(target).map(|v| v.to_string());

    Ok((res1, res2))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let target = 60;
        let res = part_one(target);
        assert_eq!(res, Some(4))
    }
    #[test]
    fn test_part_two() {
        let target = 60;
        let res = part_two(target);
        assert_eq!(res, Some(5))
    }
}
