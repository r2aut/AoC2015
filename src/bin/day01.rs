//! Day 1: Not Quite Lisp
//!
//! <https://adventofcode.com/2015/day/1>

use anyhow::Result;
use aoc2015::{AOCPrint, P1, P2, get_reader};
use std::io::BufRead;

fn follow_instructions(instructions: &str) -> (i32, Option<i32>) {
    let mut cur_step = 0;
    let mut cur_floor = 0;
    let mut first_step_on_basement: Option<i32> = None;

    for ch in instructions.chars() {
        match ch {
            '(' => cur_floor += 1,
            ')' => cur_floor -= 1,
            _ => panic!("Wrong input symbol: {ch}"),
        }
        cur_step += 1;

        if first_step_on_basement.is_none() && cur_floor < 0 {
            first_step_on_basement = Some(cur_step);
        }
    }

    (cur_floor, first_step_on_basement)
}

fn main() -> Result<()> {
    let mut reader = get_reader("puzzles/day01.txt")?;
    let mut buf = String::new();
    if reader.read_line(&mut buf)? > 0 {
        let instructions = buf.trim();

        let (res1, res2) = follow_instructions(instructions);
        res1.aoc_print(P1);
        res2.aoc_print(P2);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_to_finish() {
        assert_eq!(follow_instructions("(())").0, 0);
        assert_eq!(follow_instructions("()()").0, 0);
        assert_eq!(follow_instructions("(((").0, 3);
        assert_eq!(follow_instructions("))(((((").0, 3);
        assert_eq!(follow_instructions(")))").0, -3);
        assert_eq!(follow_instructions(")())())").0, -3);
    }

    #[test]
    fn test_go_to_basement() {
        assert_eq!(follow_instructions("()())").1, Some(5))
    }
}
