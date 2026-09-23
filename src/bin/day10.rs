//! Day 10: Elves Look, Elves Say
//!
//! <https://adventofcode.com/2015/day/10>

use anyhow::{Result, anyhow};
use aoc2015::{AOCPrint, P1, P2, get_reader};
use itertools::Itertools;
use std::io::BufRead;

/// Look and Say sequence (A005150)
struct LookAndSay {
    cur_item: String,
    first: bool,
}

impl Iterator for LookAndSay {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        // provide starting value as the first sequence item
        if self.first {
            self.first = false;
            return Some(self.cur_item.clone());
        }

        // calc next item
        let next_item = self
            .cur_item // take current value
            .chars() // split for chars
            .chunk_by(|c| *c) // group by the same chars
            .into_iter()
            .map(|(key, group)| {
                // prepare next string
                let count = group.count();
                let mut tmp = count.to_string();
                tmp.push(key);
                tmp // count + char
            })
            .join(""); // join into one string
        self.cur_item = next_item;
        Some(self.cur_item.clone())
    }
}

/// Look and Say sequence genegator
#[allow(unused)]
fn look_and_say() -> LookAndSay {
    LookAndSay { cur_item: "1".to_string(), first: true }
}

/// Look and Say generator with starting value
#[allow(unused)]
fn look_and_say_with_starting_item(value: &str) -> LookAndSay {
    LookAndSay { cur_item: value.to_string(), first: true }
}

/// Read starting value from file
fn read_starting_value(mut reader: impl BufRead) -> Result<String> {
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    Ok(buf.trim_end().to_string())
}

fn main() -> Result<()> {
    let reader = get_reader(r"puzzles/day10.txt")?;
    let starting_item = read_starting_value(reader)?;

    let res_1 = look_and_say_with_starting_item(&starting_item).nth(40).ok_or(anyhow!("Cannot fild"))?.len();
    res_1.aoc_print(P1);

    let res_2 = look_and_say_with_starting_item(&starting_item).nth(50).ok_or(anyhow!("Cannot fild"))?.len();
    res_2.aoc_print(P2);

    Ok(())
}
