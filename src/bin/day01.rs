//! Day 1: Not Quite Lisp
//!
//! <https://adventofcode.com/2015/day/1>

use aoc2015::{P1, P2};
use std::fs::File;
use std::io::{BufRead, BufReader};

use colored::Colorize;

pub fn go_to_finish(instructions: &str) -> i32 {
    let mut floor = 0;
    for command in instructions.chars() {
        match command {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }
    floor
}

fn go_to_basement(instructions: &str) -> Option<i32> {
    let mut step = 0;
    let mut floor = 0;

    for ch in instructions.chars() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Wrong input symbol: {ch}"),
        }
        step += 1;
        // println!("Floor {floor}");

        if floor < 0 {
            break;
        }
    }

    Some(step)
}

fn main() {
    let file = File::open("puzzles/day01.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    if reader.read_line(&mut line).unwrap() > 0 {
        let instructions = line.trim();

        let r1 = go_to_finish(instructions);
        println!("{} The instructions take Santa to the {} floor.", P1.green(), r1.to_string().green());

        let r2 = go_to_basement(instructions);
        if let Some(res) = r2 {
            println!("{} The position of the character is {}.", P2.green(), res.to_string().green())
        } else {
            print!("{} Santa didn't enter the basement.", P2.green())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_to_finish() {
        assert_eq!(go_to_finish("(())"), 0);
        assert_eq!(go_to_finish("()()"), 0);
        assert_eq!(go_to_finish("((("), 3);
        assert_eq!(go_to_finish("))((((("), 3);
        assert_eq!(go_to_finish(")))"), -3);
        assert_eq!(go_to_finish(")())())"), -3);
    }

    #[test]
    fn test_go_to_basement() {
        assert_eq!(go_to_basement("()())"), Some(5))
    }
}
