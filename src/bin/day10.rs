//! Day 10: Elves Look, Elves Say
//!
//! <https://adventofcode.com/2015/day/10>

use std::io::BufRead;

use aoc2015::{P1, P2};
use colored::Colorize;
use itertools::Itertools;

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
    LookAndSay {
        cur_item: "1".to_string(),
        first: true,
    }
}

/// Look and Say generator with starting value
#[allow(unused)]
fn look_and_say_with_starting_item(value: &str) -> LookAndSay {
    LookAndSay {
        cur_item: value.to_string(),
        first: true,
    }
}

/// Read starting value from file
fn read_starting_value(file_name: &str) -> String {
    let file = std::fs::File::open(file_name).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    buf.trim_end().to_string()
}

fn main() {
    let file_name = r"puzzles/day10.txt";
    let starting_item = read_starting_value(file_name);

    let res_1 = look_and_say_with_starting_item(&starting_item).nth(40).unwrap();
    println!("{} The length of the result is {}", P1.green(), res_1.len().to_string().green());

    let res_2 = look_and_say_with_starting_item(&starting_item).nth(50).unwrap();
    println!("{} The length of the result is {}", P2.green(), res_2.len().to_string().green());
}
