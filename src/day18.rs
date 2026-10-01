//! Day 18: Like a GIF For Your Yard
//!
//! <https://adventofcode.com/2015/day/18>

use std::{
    fmt::{Display, Write},
    io::BufRead,
    ops::Add,
};

use anyhow::{Result, bail};
use log::debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Light {
    On,
    Off,
    AlwaysOn,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}
impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Pos { x: x as i32, y: y as i32 }
    }
}
impl Add<(i32, i32)> for Pos {
    type Output = Pos;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Pos { x: self.x + rhs.0, y: self.y + rhs.1 }
    }
}

/// Screen with lights
#[derive(Debug, Clone)]
struct Lights(Vec<Vec<Light>>);
impl Lights {
    /// Columns number
    fn r_len(&self) -> usize {
        self.0.len()
    }
    /// Rows Number
    fn c_len(&self) -> usize {
        if !self.0.is_empty() { self.0[0].len() } else { 0 }
    }
    /// Get one light from screen
    fn get(&self, pos: &Pos) -> Option<Light> {
        if pos.x >= 0 && pos.x < self.c_len() as i32 && pos.y >= 0 && pos.y < self.r_len() as i32 {
            Some(self.0[pos.y as usize][pos.x as usize])
        } else {
            None
        }
    }
    /// Get one light from screen for mutation
    fn get_mut(&mut self, pos: &Pos) -> Option<&mut Light> {
        if pos.x >= 0 && pos.x < self.c_len() as i32 && pos.y >= 0 && pos.y < self.r_len() as i32 {
            self.0[pos.y as usize].get_mut(pos.x as usize)
        } else {
            None
        }
    }
    /// Nomber of neighbours which are on
    fn calc_neighbours_on(&self, pos: &Pos) -> usize {
        let mut on_counter = 0;
        let neighbour_shifts = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        for shift in neighbour_shifts {
            let n_pos = *pos + shift;
            if let Some(b) = self.get(&n_pos)
                && (b == Light::On || b == Light::AlwaysOn)
            {
                on_counter += 1;
            }
        }
        on_counter
    }
    /// Calc new light from current and neighbours
    fn new_light(&self, pos: Pos) -> Option<Light> {
        if let Some(prev_light) = self.get(&pos) {
            if prev_light == Light::AlwaysOn {
                return Some(Light::AlwaysOn);
            }
            let nb_on = self.calc_neighbours_on(&pos);
            if prev_light == Light::On {
                if (2..=3).contains(&nb_on) { Some(Light::On) } else { Some(Light::Off) }
            } else {
                if nb_on == 3 { Some(Light::On) } else { Some(Light::Off) }
            }
        } else {
            None
        }
    }
    /// Get next generation of Lights
    fn clone_with_rules(&self) -> Self {
        let mut new_lights = self.clone();
        for (n_row, row) in self.0.iter().enumerate() {
            for (n_col, _) in row.iter().enumerate() {
                if let Some(new_light) = self.new_light(Pos::new(n_col, n_row))
                    && let Some(nl) = new_lights.get_mut(&Pos::new(n_col, n_row))
                {
                    *nl = new_light;
                }
            }
        }
        debug!("Lights are cloned");
        new_lights
    }
    /// Get number of on-lights
    fn count_on(&self) -> i32 {
        let mut counter = 0;
        for row in self.0.iter() {
            for &v in row.iter() {
                if v == Light::On || v == Light::AlwaysOn {
                    counter += 1
                }
            }
        }
        counter
    }
    /// Set corners to always_on
    fn fix_corners(&mut self) {
        if self.r_len() > 0 && self.c_len() > 0 {
            let mess = "Lidgh's size checked";
            *self.get_mut(&Pos::new(0, 0)).expect(mess) = Light::AlwaysOn;
            *self.get_mut(&Pos::new(0, self.r_len() - 1)).expect(mess) = Light::AlwaysOn;
            *self.get_mut(&Pos::new(self.c_len() - 1, 0)).expect(mess) = Light::AlwaysOn;
            *self.get_mut(&Pos::new(self.c_len() - 1, self.r_len() - 1)).expect(mess) = Light::AlwaysOn;
        }
        debug!("Lights' corners are fixed");
    }

    /// IntoIterator
    fn into_generations(self) -> Generations {
        Generations { current: self }
    }
}
impl Display for Lights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            for &v in row.iter() {
                match v {
                    Light::On => f.write_char('#')?,
                    Light::Off => f.write_char('.')?,
                    Light::AlwaysOn => f.write_char('#')?,
                }
            }
            f.write_char('\n')?
        }
        Ok(())
    }
}

/// Iterator implementation
struct Generations {
    current: Lights,
}

impl Iterator for Generations {
    type Item = Lights;

    fn next(&mut self) -> Option<Self::Item> {
        let new_lights = self.current.clone_with_rules();
        self.current = new_lights;

        Some(self.current.clone())
    }
}

/// Parser for Lights from aoc
struct LightsParser;
impl LightsParser {
    fn try_from_aoc_reader(reader: impl BufRead) -> Result<Lights> {
        let mut lights = Lights(Vec::new());
        for line in reader.lines() {
            let mut row: Vec<Light> = Vec::new();
            for ch in line?.chars() {
                match ch {
                    '#' => row.push(Light::On),
                    '.' => row.push(Light::Off),
                    _ => bail!("Unknown character in input string {}", ch),
                };
            }
            lights.0.push(row);
        }
        Ok(lights)
    }
}

pub fn solve(reader: impl BufRead) -> Result<(Option<String>, Option<String>)> {
    let mut lights = LightsParser::try_from_aoc_reader(reader)?;
    // part one
    let res1 = lights.clone().into_generations().nth(99).map(|r| r.count_on().to_string());
    // part two
    lights.fix_corners();
    let res2 = lights.into_generations().nth(99).map(|r| r.count_on().to_string());

    Ok((res1, res2))
}

#[cfg(test)]

mod test {
    use super::*;

    const DATA: &str = "\
.#.#.#
...##.
#....#
..#...
#.#..#
####..
";

    #[test]
    fn test_part_one() {
        let reader = DATA.as_bytes();
        let lights = LightsParser::try_from_aoc_reader(reader).unwrap();
        assert_eq!(lights.clone().into_generations().nth(3).map(|r| r.count_on()), Some(4));
    }
    #[test]
    fn test_part_two() {
        let reader = DATA.as_bytes();
        let mut lights = LightsParser::try_from_aoc_reader(reader).unwrap();

        lights.fix_corners();
        assert_eq!(lights.clone().into_generations().nth(4).map(|r| r.count_on()), Some(17));
    }
}
