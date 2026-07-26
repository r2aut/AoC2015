//! Day 8: Matchsticks
//!
//! <https://adventofcode.com/2015/day/8>

use aoc2015::{P1, P2};
use colored::Colorize;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

/// Convert string from code representation to memory representation
fn squize_string(inp: &[u8]) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    let mut pos: usize = 0;

    while pos < inp.len() {
        match inp[pos] {
            b'\"' => {
                // begin or end of the string, just skip
                if (pos == 0) || (pos == inp.len() - 1) {
                    pos += 1;
                } else {
                    panic!("Wrong situation, met quotation mark, str {:?}, pos = {}", inp, pos)
                }
            }
            b'\\' => match inp[pos + 1] {
                b'\\' => {
                    res.push(b'\\');
                    pos += 2;
                }
                b'\"' => {
                    res.push(b'\"');
                    pos += 2;
                }
                b'x' => {
                    let d0 = (inp[pos + 2] as char).to_digit(16).unwrap();
                    let d1 = (inp[pos + 3] as char).to_digit(16).unwrap();
                    let dr = d0 * 16 + d1;
                    res.push(dr as u8);
                    pos += 4;
                }
                _ => {
                    panic!("Wrong situation, met quotation mark, str {:?}, pos = {}", inp, pos)
                }
            },
            _ => {
                res.push(inp[pos]);
                pos += 1;
            }
        }
    }
    res
}

/// Convert string from memory representation to code representation
fn expand_string(inp: &Vec<u8>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    res.push(b'\"');
    for ch in inp {
        match ch {
            b'\"' => {
                // let mut bv: Vec<u8> = "\\\"".as_bytes().into_iter().map(|x| *x).collect();
                let mut bv: Vec<u8> = "\\\"".bytes().collect();
                res.append(&mut bv);
            }
            b'\\' => {
                let mut bv: Vec<u8> = "\\\\".bytes().collect();
                res.append(&mut bv);
            }
            _ => res.push(*ch),
        }
    }
    res.push(b'\"');
    res
}

/// Solve part one
fn part_one(svec: &Vec<Vec<u8>>) -> u32 {
    let mut counter = 0;
    for s in svec {
        let new_s = squize_string(s);
        let string_size = s.len();
        let memory_size = new_s.len();
        counter += (string_size - memory_size) as u32;
    }
    counter
}

/// Solve part two
fn part_two(svec: &Vec<Vec<u8>>) -> u32 {
    let mut counter = 0;
    for s in svec {
        let new_s = expand_string(s);
        let string_size = new_s.len();
        let memory_size = s.len();
        counter += (string_size - memory_size) as u32;
    }
    counter
}

/// Read lines from file
fn read_lines(reader: impl BufRead) -> Vec<Vec<u8>> {
    let mut res: Vec<Vec<u8>> = Vec::new();
    for line in reader.lines() {
        let bytes_line = line.unwrap().into_bytes();
        res.push(bytes_line);
    }
    res
}

fn main() {
    let path = Path::new("puzzles/day08.txt");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sss = read_lines(reader);

    let res_1 = part_one(&sss);
    println!("{} {}", P1.blue(), res_1.to_string().green());

    let res_2 = part_two(&sss);
    println!("{} {}", P2.blue(), res_2.to_string().green());
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = r###"""
"abc"
"aaa\"aaa"
"\x27"
"###;

    const TEST_DATA1: &str = r###""q\xb7oh\"p\xce\"n"
"###;

    #[test]
    fn test_part_one() {
        let sss = read_lines(TEST_DATA.as_bytes());
        assert_eq!(part_one(&sss), 12)
    }
    #[test]
    fn test_part_one_1() {
        let sss = read_lines(TEST_DATA1.as_bytes());
        assert_eq!(part_one(&sss), 10)
    }
    #[test]
    fn test_part_two() {
        let sss = read_lines(TEST_DATA.as_bytes());
        assert_eq!(part_two(&sss), 19)
    }
}
