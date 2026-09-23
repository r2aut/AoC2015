//! Day 3: Perfectly Spherical Houses in a Vacuum
//!
//! <https://adventofcode.com/2015/day/3>

use anyhow::Result;
use aoc2015::{AOCPrint, P1, P2, get_reader, print_day};
use std::collections::HashSet;
use std::io::BufRead;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Default, Debug)]
struct Runner {
    pos: Point,
    trace: Vec<Point>,
}

impl Runner {
    fn new() -> Self {
        Self { pos: Point::new(), trace: vec![Point::new()] }
    }

    fn step(&mut self, dir: &Direction) {
        let pos = self.pos;
        self.pos = match dir {
            Direction::North => Point { x: pos.x, y: pos.y + 1 },
            Direction::East => Point { x: pos.x + 1, y: pos.y },
            Direction::South => Point { x: pos.x, y: pos.y - 1 },
            Direction::West => Point { x: pos.x - 1, y: pos.y },
        };
        self.trace.push(self.pos);
    }

    fn unique_points(&self) -> HashSet<&Point> {
        HashSet::from_iter(&self.trace)
    }
}

/// Translate and execute instruction in char form
fn step_runner(runner: &mut Runner, ch: char) {
    let dir: Option<Direction> = match ch {
        '^' => Some(Direction::North),
        '>' => Some(Direction::East),
        'v' => Some(Direction::South),
        '<' => Some(Direction::West),
        _ => None,
    };
    if let Some(d) = dir {
        runner.step(&d);
    } else {
        eprint!("Wrong input symbol {}", ch);
    }
}

/// Solution for part one
fn part_one(instructions: &str) -> i32 {
    let mut runner = Runner::new();
    for ch in instructions.chars() {
        step_runner(&mut runner, ch);
    }
    runner.unique_points().len() as i32
}

/// Solution for part two
fn part_two(instructions: &str) -> i32 {
    // Process Santa movings
    let santa_instructions = instructions
        .to_string()
        .chars()
        .enumerate()
        .filter(|(pos, _)| pos % 2 == 0)
        .map(|(_, ch)| ch)
        .collect::<Vec<char>>();
    let mut santa = Runner::new();
    for ch in santa_instructions {
        step_runner(&mut santa, ch);
    }
    // Process robot movings
    let robot_instructions = instructions
        .to_string()
        .chars()
        .enumerate()
        .filter(|(pos, _)| pos % 2 == 1)
        .map(|(_, ch)| ch)
        .collect::<Vec<char>>();
    let mut robot = Runner::new();
    for ch in robot_instructions {
        step_runner(&mut robot, ch);
    }
    // Unite sets of visited places and count them
    santa.unique_points().union(&robot.unique_points()).count() as i32
}

fn main() -> Result<()> {
    let mut reader = get_reader("puzzles/day03.txt")?;
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    let instructions = buffer.trim();

    print_day();

    let res_1 = part_one(instructions);
    res_1.aoc_print(P1);

    let res_2 = part_two(instructions);
    res_2.aoc_print(P2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(">"), 2);
        assert_eq!(part_one("^>v<"), 4);
        assert_eq!(part_one("^v^v^v^v^v"), 2);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two("^v"), 3);
        assert_eq!(part_two("^>v<"), 3);
        assert_eq!(part_two("^v^v^v^v^v"), 11);
    }
}
