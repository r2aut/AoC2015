//! Day 24: It Hangs in the Balance
//!
//! <https://adventofcode.com/2015/day/24>

use anyhow::Result;
use aoc2015::{AOCPrint, P1, P2, get_reader, print_day};
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    io::BufRead,
};

/// Get all packet sets using backtraking
fn get_all_packet_sets(
    items: &[i32],
    start_index: usize,
    used: &mut Vec<i32>,
    cmp: &dyn Fn(&Vec<i32>) -> std::cmp::Ordering,
    results: &mut Vec<Vec<i32>>,
) {
    for next_index in start_index..items.len() {
        // process current node
        used.push(items[next_index]);

        match cmp(used) {
            Less => get_all_packet_sets(items, next_index + 1, used, cmp, results),
            Equal => results.push(used.clone()),
            Greater => {}
        };
        // step back
        used.pop();
    }
}

/// Get packet set with the best values
fn get_best_group_eq(packets: &Vec<Vec<i32>>) -> Option<i128> {
    let mut res: Vec<(Vec<i32>, i32, i128)> = Vec::new();
    for p in packets {
        // calc the packet set size
        let num = p.len() as i32;
        // calc the quantum entanglement of the packet set
        let prod = p.iter().map(|n| *n as i128).product();
        // get all packet sets with size and qe
        res.push((p.clone(), num, prod));
        // get min packet set
    }
    // get min value by size and after by eq
    let min_packet = res.iter().min_by_key(|i| (i.1, i.2))?;
    Some(min_packet.2)
}

/// Read packages from stream
fn read_packets(reader: impl BufRead) -> Result<Vec<i32>> {
    reader
        .lines()
        .map(|s| {
            let parsed = s?.parse::<i32>()?;
            Ok(parsed)
        })
        .collect()
}

/// Calculate the best eq for the packages (part one and two)
fn calc_best_eq(packets: &[i32], group_num: i32) -> Option<i128> {
    let total_weght: i32 = packets.iter().sum::<i32>() / group_num;

    let mut used: Vec<i32> = Vec::new();
    let mut result: Vec<Vec<i32>> = Vec::new();
    get_all_packet_sets(
        packets,
        0,
        &mut used,
        &move |v: &Vec<i32>| -> std::cmp::Ordering {
            let s = v.iter().sum::<i32>();
            s.cmp(&total_weght)
        },
        &mut result,
    );
    let qe = get_best_group_eq(&result)?;
    Some(qe)
}

fn main() -> Result<()> {
    let reader = get_reader("puzzles/day24.txt")?;
    let mut packets = read_packets(reader)?;
    packets.sort_by(|a, b| b.cmp(a));

    print_day();
    let res1 = calc_best_eq(&packets, 3);
    res1.aoc_print(P1);
    let res2 = calc_best_eq(&packets, 4);
    res2.aoc_print(P2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "\
1
2
3
4
5
7
8
9
10
11
";

    #[test]
    fn test_part_1() {
        let packets = read_packets(DATA.as_bytes()).unwrap();
        let res = calc_best_eq(&packets, 3).unwrap();
        assert_eq!(res, 99);
    }
    #[test]
    fn test_part_2() {
        let packets = read_packets(DATA.as_bytes()).unwrap();
        let res = calc_best_eq(&packets, 4).unwrap();
        assert_eq!(res, 44);
    }
}
