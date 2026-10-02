//! Day 17: No Such Thing as Too Much
//!
//! <https://adventofcode.com/2015/day/17>

use anyhow::Result;
use log::{debug, trace};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::BufRead;

type ContainerList = Vec<i32>;

/// Read all input data from stream
fn try_read_container_list(reader: impl BufRead) -> Result<ContainerList> {
    let mut res = ContainerList::new();
    for line in reader.lines() {
        let c = line?.parse()?;
        res.push(c);
    }
    debug!("Read items vector: {:?}", res);
    Ok(res)
}

/// Recurcive function to find results
fn backtrack(
    items: &ContainerList,
    start_index: usize,
    used: &mut ContainerList,
    cmp: &dyn Fn(&Vec<i32>) -> std::cmp::Ordering,
    results: &mut Vec<Vec<i32>>,
) {
    used.push(items[start_index]);
    trace!("Try set {:?}", used);
    match cmp(used) {
        Less => {
            for next_index in start_index + 1..items.len() {
                backtrack(items, next_index, used, cmp, results);
            }
        }
        Equal => {
            results.push(used.clone());
            debug!("{:?} \t=  {}", used, used.iter().sum::<i32>())
        }
        Greater => {}
    }
    used.pop();
}

/// Get all results that fulfil the target
fn get_results(items: &Vec<i32>, target: i32) -> Vec<ContainerList> {
    debug!("Got items vector: {:?}", items);
    debug!("Got target value: {}", target);

    let compare_closure = move |v: &Vec<i32>| -> std::cmp::Ordering {
        let s = v.iter().sum::<i32>();
        s.cmp(&target)
    };

    let mut used: Vec<i32> = Vec::new();
    let mut results: Vec<Vec<i32>> = Vec::new();

    for i in 0..items.len() {
        backtrack(items, i, &mut used, &compare_closure, &mut results);
    }
    debug!("Calculated results {}", results.len());
    results
}

/// Solve part one
fn part_one(results: &[ContainerList]) -> usize {
    results.len()
}
/// Solve part two
fn part_two(results: &[ContainerList]) -> Option<usize> {
    if let Some(min_length) = results.iter().map(|r| r.len()).min() {
        debug!("Min length of result {}", min_length);
        Some(results.iter().filter(|r| r.len() == min_length).count())
    } else {
        debug!("No results were provided, return None");
        None
    }
}

pub fn solve(reader: impl BufRead) -> Result<(Option<String>, Option<String>)> {
    let cont = try_read_container_list(reader)?;
    // items.sort_by(|a, b| b.cmp(a));
    let target = 150;

    let results = get_results(&cont, target);
    let res1 = Some(part_one(&results).to_string());
    let res2 = part_two(&results).map(|r| r.to_string());

    Ok((res1, res2))
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "\
20
15
5
10
5
";

    #[test]
    fn test_part_one() {
        let reader = DATA.as_bytes();
        let cont = try_read_container_list(reader).unwrap();
        let results = get_results(&cont, 25);
        let res = part_one(&results);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_part_two() {
        let reader = DATA.as_bytes();
        let cont = try_read_container_list(reader).unwrap();
        let results = get_results(&cont, 25);
        let res = part_two(&results);
        assert_eq!(res, Some(3));
    }
}
