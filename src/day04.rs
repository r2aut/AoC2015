//! Day 4: The Ideal Stocking Stuffer
//!
//! <https://adventofcode.com/2015/day/4>

use anyhow::Result;
use std::io::BufRead;

struct AdventCoin {
    secret_key: String,
    zero_num: usize,

    zero_mask_: String,
    next_num_: u32,
}

impl AdventCoin {
    fn new_coin(secret_key_str: &str, zero_num: usize) -> Self {
        Self {
            secret_key: secret_key_str.to_string(),
            zero_num,
            zero_mask_: (vec!['0'; zero_num]).iter().collect::<String>(),
            next_num_: 0,
        }
    }
    fn digest(&self, number: u32) -> String {
        let number_str = number.to_string();
        let key_str = self.secret_key.clone() + &number_str;
        let digest = md5::compute(key_str.as_bytes());
        format!("{:x}", digest)
    }
    fn is_coin(&self, number: u32) -> bool {
        self.digest(number)[0..self.zero_num] == self.zero_mask_
    }
}

impl Iterator for AdventCoin {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let mut key = self.next_num_;
        while !self.is_coin(key) {
            key += 1;
        }
        self.next_num_ = key + 1;
        Some(key)
    }
}

pub fn solve(mut reader: impl BufRead) -> Result<(Option<String>, Option<String>)> {
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    let secret_key = buf.trim();

    let coin = AdventCoin::new_coin(secret_key, 5);
    let mut it = coin.into_iter();
    let res1 = it.next().map(|v| v.to_string());

    let coin = AdventCoin::new_coin(secret_key, 6);
    let mut it = coin.into_iter();
    let res2 = it.next().map(|v| v.to_string());

    Ok((res1, res2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5_zero_coin() {
        let coin = AdventCoin::new_coin("abcdef", 5);
        let mut it = coin.into_iter();
        assert_eq!(it.next().unwrap(), 609043);
        assert_ne!(it.next().unwrap(), 609043);

        let coin = AdventCoin::new_coin("pqrstuv", 5);
        let mut it = coin.into_iter();
        assert_eq!(it.next().unwrap(), 1048970);
        assert_ne!(it.next().unwrap(), 1048970);
    }
}
