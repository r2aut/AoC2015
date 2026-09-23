//! Day 6: Probably a Fire Hazard
//!
//! <https://adventofcode.com/2015/day/6>

use anyhow::{Result, anyhow};
use aoc2015::{AOCPrint, P1, P2, get_reader, print_day};
use std::cmp::{max, min};
use std::fmt::Display;
use std::io::BufRead;

/// Corner points of screen zone
#[derive(Debug, Clone, Copy)]
struct Corners {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

/// Position o screen
#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    /// Calc corners coordinates from corner points
    fn corners(self, q: Position) -> Corners {
        let min_x = min(self.x, q.x);
        let max_x = max(self.x, q.x);
        let min_y = min(self.y, q.y);
        let max_y = max(self.y, q.y);
        Corners { min_x, max_x, min_y, max_y }
    }
}

/// Screen for the holiday house decorating contest
#[derive(Debug)]
struct Screen {
    lights: Vec<Vec<u32>>,
}

impl Screen {
    fn new(size: usize) -> Self {
        Self { lights: vec![vec![0; size]; size] }
    }

    /// Turn on lights on screen zone
    fn turn_on(&mut self, q1: Position, q2: Position) {
        let borders = q1.corners(q2);
        for y in borders.min_y..=borders.max_y {
            for x in borders.min_x..=borders.max_x {
                if self.lights[y][x] == 0 {
                    self.lights[y][x] = 1;
                }
            }
        }
    }

    /// Turn off lights on screen zone
    fn turn_off(&mut self, q1: Position, q2: Position) {
        let borders = q1.corners(q2);
        for y in borders.min_y..=borders.max_y {
            for x in borders.min_x..=borders.max_x {
                if self.lights[y][x] > 0 {
                    self.lights[y][x] = 0;
                }
            }
        }
    }

    /// Toggle lights on screen zone
    fn toggle(&mut self, q1: Position, q2: Position) {
        let borders = q1.corners(q2);
        for y in borders.min_y..=borders.max_y {
            for x in borders.min_x..=borders.max_x {
                if self.lights[y][x] == 0 {
                    self.lights[y][x] = 1;
                } else {
                    self.lights[y][x] = 0;
                }
            }
        }
    }

    /// Decrease lightness of screen zone by 1
    fn dim(&mut self, q1: Position, q2: Position) {
        let borders = q1.corners(q2);
        for y in borders.min_y..=borders.max_y {
            for x in borders.min_x..=borders.max_x {
                if self.lights[y][x] > 0 {
                    self.lights[y][x] -= 1;
                }
            }
        }
    }

    /// Increase lightness of screen zone by 1
    fn brighten(&mut self, q1: Position, q2: Position) {
        let borders = q1.corners(q2);
        for y in borders.min_y..=borders.max_y {
            for x in borders.min_x..=borders.max_x {
                self.lights[y][x] += 1;
            }
        }
    }

    /// Increase lightness of screen zone by 2
    fn illuminate(&mut self, q1: Position, q2: Position) {
        let borders = q1.corners(q2);
        for y in borders.min_y..=borders.max_y {
            for x in borders.min_x..=borders.max_x {
                self.lights[y][x] += 2;
            }
        }
    }

    /// Get number of lighting lights
    fn get_lights_number(&self) -> u32 {
        let mut counter = 0_u32;
        for row in &self.lights {
            for ch in row {
                if *ch > 0 {
                    counter += 1;
                }
            }
        }
        counter
    }

    /// Get the total brightness of screen
    fn get_total_brightness(&self) -> u32 {
        let mut counter = 0_u32;
        for row in &self.lights {
            for ch in row {
                if ch > &0_u32 {
                    counter += ch;
                }
            }
        }
        counter
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.lights {
            for ch in row {
                write!(
                    f,
                    "{}",
                    match ch {
                        0 => '.',
                        1..9 => {
                            let cccc = ch + '0' as u32;
                            if let Some(res) = std::char::from_u32(cccc) { res } else { unreachable!() }
                        }
                        _ => '#',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// Instruction to control the screen lights
#[derive(Debug)]
struct Instruction {
    cmd: String,
    from_pos: Position,
    to_pos: Position,
}

/// Read instructions from file
fn read_instructions(reader: impl BufRead) -> Result<Vec<Instruction>> {
    use regex::Regex;
    let pattern = r"(\D*) (\d*),(\d*) through (\d*),(\d*)";
    let re = Regex::new(pattern)?;

    let mut instructions = Vec::<Instruction>::new();
    for line in reader.lines() {
        let line = line?;
        let caps = re.captures(&line).ok_or(anyhow!("Nothing found"))?;
        instructions.push(Instruction {
            cmd: caps[1].to_string(),
            from_pos: Position { x: caps[2].parse::<usize>()?, y: caps[3].parse::<usize>()? },
            to_pos: Position { x: caps[4].parse::<usize>()?, y: caps[5].parse::<usize>()? },
        });
    }
    Ok(instructions)
}

/// Part One solution
fn part_one(screen: &mut Screen, instructions: &Vec<Instruction>) -> u32 {
    for instruction in instructions {
        match (instruction.cmd).as_str() {
            "turn on" => screen.turn_on(instruction.from_pos, instruction.to_pos),
            "turn off" => screen.turn_off(instruction.from_pos, instruction.to_pos),
            "toggle" => screen.toggle(instruction.from_pos, instruction.to_pos),
            _ => panic!(),
        }
    }
    screen.get_lights_number()
}

/// Part Two solution
fn part_two(screen: &mut Screen, instructions: &Vec<Instruction>) -> u32 {
    for instruction in instructions {
        match (instruction.cmd).as_str() {
            "turn on" => screen.brighten(instruction.from_pos, instruction.to_pos),
            "turn off" => screen.dim(instruction.from_pos, instruction.to_pos),
            "toggle" => screen.illuminate(instruction.from_pos, instruction.to_pos),
            _ => panic!(),
        }
    }
    screen.get_total_brightness()
}

fn main() -> Result<()> {
    let reader = get_reader(r"puzzles/day06.txt")?;
    let instructions = read_instructions(reader)?;

    print_day();

    let mut screen1 = Screen::new(1000);
    let res_1 = part_one(&mut screen1, &instructions);
    res_1.aoc_print(P1);

    let mut screen2 = Screen::new(1000);
    let res_2 = part_two(&mut screen2, &instructions);
    res_2.aoc_print(P2);

    Ok(())
}

/// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_screen() {
        let s = Screen::new(5);
        assert_eq!(s.get_total_brightness(), 0);
        assert_eq!(s.get_lights_number(), 0);
    }

    #[test]
    fn test_part_one_methods() {
        let mut screen = Screen::new(1000);
        screen.turn_on(Position { x: 0, y: 0 }, Position { x: 0, y: 999 });
        assert_eq!(screen.get_lights_number(), 1000);
        screen.toggle(Position { x: 0, y: 0 }, Position { x: 999, y: 0 });
        assert_eq!(screen.get_lights_number(), 1998);
        screen.turn_off(Position { x: 0, y: 0 }, Position { x: 0, y: 999 });
        assert_eq!(screen.get_lights_number(), 999);
    }

    #[test]
    fn test_part_two_methods() {
        let mut screen = Screen::new(1000);
        screen.brighten(Position { x: 0, y: 0 }, Position { x: 0, y: 0 });
        assert_eq!(screen.get_total_brightness(), 1);
        screen.illuminate(Position { x: 0, y: 0 }, Position { x: 999, y: 999 });
        assert_eq!(screen.get_total_brightness(), 2000001);
        screen.dim(Position { x: 0, y: 0 }, Position { x: 999, y: 999 });
        assert_eq!(screen.get_total_brightness(), 1000001);
    }
}
