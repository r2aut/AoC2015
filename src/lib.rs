use anyhow::Result;
use colored::Colorize;
use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub const AOC: &str = "AoC2015";
pub const AOC_DESC: &str = "Advent of Code 2015";

// result print prefixes
pub const P1: &str = "Part One = ";
pub const P2: &str = "Part Two = ";

pub mod day07;
pub mod day22;

pub fn get_reader<P: AsRef<Path>>(path: P) -> Result<impl BufRead> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

pub trait AOCPrint {
    fn aoc_print(&self, part: &str);
}

impl<T: Display> AOCPrint for Option<T> {
    fn aoc_print(&self, part: &str) {
        match self {
            Some(value) => {
                let value_str = value.to_string();
                println!("{} {}", part.blue(), value_str.green())
            }
            None => {
                println!("{} {}", part.blue(), "no result".red())
            }
        }
    }
}

// Macro for printing results
macro_rules! impl_aoc_print {
    ($($t:ty),*) => {
        $(
            impl AOCPrint for $t {
                fn aoc_print(&self, part: &str) {
                    let value_str = self.to_string();
                    println!("{} {}", part.blue(), value_str.green());
                }
            }
        )*
    };
}

// Types for print macros
impl_aoc_print!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, &str, String);
