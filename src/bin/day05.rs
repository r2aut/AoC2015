//! Day 5: Doesn't He Have Intern-Elves For This?
//!
//! <https://adventofcode.com/2015/day/5>

use anyhow::Result;
use aoc2015::{AOCPrint, P1, P2, get_reader};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

// Rule 1: "contains at least three vowels"
fn rule1(string: &str) -> bool {
    let vowels: HashSet<char> = HashSet::from_iter("aeiou".chars());
    string.chars().filter(|ch| vowels.contains(ch)).count() >= 3
}

// Rule 2: "contains at least one letter that appears twice in a row"
fn rule2(string: &str) -> bool {
    string.chars().tuple_windows::<(_, _)>().any(|(a, b)| a == b)
}

// Rule 3: "does not contain some strings"
fn rule3(string: &str) -> bool {
    let letters = ["ab", "cd", "pq", "xy"];
    let chanks: HashSet<String> = HashSet::from_iter(letters.iter().map(|s| s.to_string()));
    !string.chars().tuple_windows::<(_, _)>().map(|(a, b)| a.to_string() + &b.to_string()).any(|a| chanks.contains(&a))
}

// Rule 4: "contains a pair of any two letters that appears at least twice"
fn rule4(string: &str) -> bool {
    let mut pairs: HashMap<&str, u32> = HashMap::new();

    let mut prev_pair = "";
    for i in 0..string.len() - 1 {
        // make 2-char slice of str
        let cc = &string[i..i + 2];
        if cc != prev_pair {
            // count it in the HashMap
            pairs.entry(cc).and_modify(|e| *e += 1).or_insert(1);
            prev_pair = cc;
        } else {
            // 3 the same chars in a row
            prev_pair = "";
        }
    }

    // was at least one pair that apeared 2 or more times?
    let mut result = false;
    for (_pair, cnt) in pairs {
        if cnt >= 2 {
            result = true;
            break;
        }
    }
    result
}

// Rule 5: "contains at least one letter which repeats with exactly one letter between"
fn rule5(string: &str) -> bool {
    string.chars().tuple_windows::<(_, _, _)>().any(|(a, _b, c)| a == c)
}

// Niceness of string in Part One
fn is_nice_1(string: &str) -> bool {
    rule1(string) && rule2(string) && rule3(string)
}

// Niceness of string in Part Two
fn is_nice_2(string: &str) -> bool {
    rule4(string) && rule5(string)
}

// Read strings from the file
fn read_data(reader: impl BufRead) -> Result<Vec<String>> {
    Ok(reader.lines().map(|r| if let Ok(res) = r { res } else { panic!() }).collect())
}

// Solution for Part One
fn part_one(data: &[String]) -> u32 {
    data.iter().filter(|s| is_nice_1(s)).collect::<Vec<&String>>().len() as u32
}

// Solution for Part Two
fn part_two(data: &[String]) -> u32 {
    data.iter().filter(|s| is_nice_2(s)).collect::<Vec<&String>>().len() as u32
}

fn main() -> Result<()> {
    let reader = get_reader("puzzles/day05.txt")?;
    let data = &read_data(reader)?;

    let res_1 = part_one(data);
    res_1.aoc_print(P1);
    let res_2 = part_two(data);
    res_2.aoc_print(P2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule1() {
        assert!(rule1("aei"));
        assert!(rule1("xazegov"));
        assert!(rule1("aeiouaeiouaeiou"));
        assert!(!rule1("hdsjfjks"));
        assert!(!rule1("hdasjfijks"));
    }

    #[test]
    fn test_rule2() {
        assert!(rule2("xx"));
        assert!(rule2("abcdde"));
        assert!(rule2("aabbccdd"));
        assert!(!rule2("abcd"));
    }

    #[test]
    fn test_rule3() {
        assert!(rule3("aeiouaeiouaeiou"));
        assert!(!rule3("aabbccdd"));
        assert!(!rule3("fjdkjabffds"));
        assert!(!rule3("cdfjgkjkgj"));
        assert!(!rule3("rrngjghhsgfpq"));
        assert!(!rule3("thhgsxygs"));
    }

    #[test]
    fn test_rule4() {
        assert!(rule4("xyxy"));
        assert!(rule4("aabcdefgaa"));
        assert!(!rule4("aaa"));
    }

    #[test]
    fn test_rule5() {
        assert!(rule5("xyx"));
        assert!(rule5("abcdefeghi"));
        assert!(rule5("aaa"));
    }

    #[test]
    fn test_nice_1() {
        assert!(is_nice_1("ugknbfddgicrmopn"));
        assert!(is_nice_1("aaa"));
        assert!(!is_nice_1("jchzalrnumimnmhp"));
        assert!(!is_nice_1("haegwjzuvuyypxyu"));
        assert!(!is_nice_1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_nice_2() {
        assert!(is_nice_2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_2("xxyxx"));
        assert!(!is_nice_2("uurcxstgmygtbstg"));
        assert!(!is_nice_2("ieodomkazucvgmuy"));
    }
}
