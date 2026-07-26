//! Day 11: Corporate Policy
//!
//! <https://adventofcode.com/2015/day/11>

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use aoc2015::{P1, P2, errors::AoCError};
use colored::Colorize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Password(String);
impl Password {
    fn new(password: &str) -> Self {
        Self(password.to_string())
    }
    /// Rule 1
    /// Passwords must include one increasing straight of at least three letters
    fn rule1(&self) -> bool {
        let mut res = false;
        let v: Vec<u8> = self.0.bytes().collect();
        if v.len() >= 3 {
            for three in v.windows(3) {
                if three[0] == three[1] - 1 && three[1] == three[2] - 1 {
                    res = true;
                    break;
                }
            }
        }
        res
    }
    /// Rule 2
    /// Passwords may not contain the letters i, o, or l
    fn rule2(&self) -> bool {
        !(self.0.contains('i') || self.0.contains('o') || self.0.contains('l'))
    }
    /// Rule 3
    /// Passwords must contain at least two different, non-overlapping pairs of letters
    fn rule3(&self) -> bool {
        let v: Vec<u8> = self.0.bytes().collect();
        let tmp = v.windows(2).collect::<Vec<_>>();
        let mut prev_pair: Option<u8> = None;
        let mut pairs: Vec<u8> = Vec::new();
        for ss in tmp {
            if ss[0] == ss[1] && Some(ss[0]) != prev_pair {
                prev_pair = Some(ss[0]);
                pairs.push(ss[0]);
            } else {
                prev_pair = None;
            }
        }
        pairs.len() >= 2
    }
    /// Check is password valid
    fn is_valid(&self) -> bool {
        self.rule1() && self.rule2() && self.rule3()
    }
    /// Get value (str) of the password
    fn value(&self) -> &str {
        &self.0
    }
    /// Get next pretender to be the password
    fn next_pretender(&mut self) {
        let mut password_bytes: Vec<u8> = self.0.bytes().collect();
        password_bytes.reverse();
        let mut bit_to_add = 1;
        for n in &mut password_bytes {
            let new_n = *n + bit_to_add;
            // just get next ascii char
            *n = if new_n <= b'z' {
                bit_to_add = 0;
                new_n
            // wraps around
            } else {
                bit_to_add = 1;
                b'a'
            }
        }
        if bit_to_add == 1 {
            password_bytes.push(b'a');
        }
        password_bytes.reverse();
        self.0 = String::from_utf8(password_bytes).unwrap();
    }
    /// Get next valid password
    fn next_password(&mut self) -> Result<&str, AoCError> {
        self.next_pretender();
        while !self.is_valid() {
            self.next_pretender();
        }
        Ok(self.value())
    }
}

/// Read initial password from the file
fn read_init_password(reader: &mut impl BufRead) -> Result<Password, AoCError> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    Ok(Password::new(buf.trim()))
}

fn main() -> Result<(), AoCError> {
    let path = Path::new("puzzles/day11.txt");
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut password = read_init_password(&mut reader)?;

    let res_1 = password.next_password()?;
    println!("{}{}", P1.blue(), res_1.green());
    let res_2 = password.next_password()?;
    println!("{}{}", P2.blue(), res_2.green());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rules() {
        let test_str = "hijklmmn";
        assert!(Password::new(test_str).rule1());
        assert!(!Password::new(test_str).rule2());
        assert!(!Password::new(test_str).rule3());
        let test_str = "abbceffg";
        assert!(!Password::new(test_str).rule1());
        assert!(Password::new(test_str).rule2());
        assert!(Password::new(test_str).rule3());
        let test_str = "abbcegjk";
        assert!(!Password::new(test_str).rule1());
        assert!(Password::new(test_str).rule2());
        assert!(!Password::new(test_str).rule3());
    }

    #[test]
    fn test_next_password() {
        assert_eq!(Password::new("abcdefgh").next_password().unwrap(), "abcdffaa".to_string());
        assert_eq!(Password::new("ghijklmn").next_password().unwrap(), "ghjaabcc".to_string());
    }
}
