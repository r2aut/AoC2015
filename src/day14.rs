//! Day 14: Reindeer Olympics
//!
//! <https://adventofcode.com/2015/day/14>

use anyhow::Result;
use regex::Regex;
use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
enum State {
    Flying(i32),
    Resting(i32),
}

#[derive(Debug, Clone)]
struct Reindeer {
    _name: String,
    speed: i32,
    max_fly_time: i32,
    rest_time_needed: i32,
    state: State,

    position: i32,
    points: i32,
}
impl Reindeer {
    fn new(name: &str, speed: i32, max_fly_time: i32, rest_time_needed: i32) -> Self {
        Self {
            _name: name.to_string(),
            speed,
            max_fly_time,
            rest_time_needed,
            state: State::Flying(0),
            position: 0,
            points: 0,
        }
    }
    fn give_point(&mut self) {
        self.points += 1;
    }
    fn fly_one(&mut self) {
        match self.state {
            State::Flying(time) => {
                self.position += self.speed;
                self.state = if time + 1 < self.max_fly_time { State::Flying(time + 1) } else { State::Resting(0) }
            }
            State::Resting(time) => {
                self.state = if time + 1 < self.rest_time_needed { State::Resting(time + 1) } else { State::Flying(0) }
            }
        }
    }
    fn fly_n(&mut self, tick_num: i32) {
        for _ in 0..tick_num {
            self.fly_one();
        }
    }
}

struct ReindeersParser;
impl ReindeersParser {
    fn try_from_aoc_reader(reader: impl BufRead) -> Result<Vec<Reindeer>> {
        let pattern = Regex::new(r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.")?;
        let mut result = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if let Some(caps) = pattern.captures(&line) {
                let name = &caps[1];
                let speed = caps[2].parse()?;
                let fly_time = caps[3].parse()?;
                let rest_time = caps[4].parse()?;
                let reindeer = Reindeer::new(name, speed, fly_time, rest_time);
                result.push(reindeer);
            }
        }
        Ok(result)
    }
}

fn part_one(mut reindeers: Vec<Reindeer>, fly_time: i32) -> Option<i32> {
    for r in &mut reindeers {
        r.fly_n(fly_time);
    }
    reindeers.into_iter().map(|r| r.position).max()
}

fn part_two(mut reindeers: Vec<Reindeer>, fly_time: i32) -> Option<i32> {
    for _ in 0..fly_time {
        for r in &mut reindeers {
            r.fly_one();
        }
        if let Some(max_position) = reindeers.iter().map(|r| r.position).max() {
            let mut leaders: Vec<_> = reindeers.iter_mut().filter(|r| r.position == max_position).collect();
            leaders.iter_mut().for_each(|r| r.give_point());
        }
    }
    reindeers.into_iter().map(|r| r.points).max()
}

pub fn solve(reader: impl BufRead) -> Result<(Option<String>, Option<String>)> {
    let reindeers = ReindeersParser::try_from_aoc_reader(reader)?;
    let fly_time = 2503;
    let res1 = part_one(reindeers.clone(), fly_time).map(|r| r.to_string());
    let res2 = part_two(reindeers, fly_time).map(|r| r.to_string());
    Ok((res1, res2))
}
// fn main() -> Result<()> {
//     let reader = aoc2015::get_reader("puzzles/day14.txt")?;
//     let reindeers = ReindeersParser::try_from_aoc_reader(reader)?;
//     let fly_time = 2503;
//     let res1 = part_one(reindeers.clone(), fly_time);
//     println!("{:?}", res1);
//     let res2 = part_two(reindeers, fly_time);
//     println!("{:?}", res2);
//     Ok(())
// }

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "\
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
";

    #[test]
    fn test_part_one() {
        let reindeers = ReindeersParser::try_from_aoc_reader(DATA.as_bytes()).unwrap();
        let res = part_one(reindeers, 1000).unwrap();
        assert_eq!(res, 1120);
    }

    #[test]
    fn test_part_two() {
        let reindeers = ReindeersParser::try_from_aoc_reader(DATA.as_bytes()).unwrap();
        let res = part_two(reindeers, 1000).unwrap();
        assert_eq!(res, 689);
    }
}
