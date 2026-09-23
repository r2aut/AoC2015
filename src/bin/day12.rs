//! Day 12: JSAbacusFramework.io
//!
//! <https://adventofcode.com/2015/day/12>

use anyhow::Result;
use aoc2015::{AOCPrint, P1, P2, get_reader, print_day};
use serde_json::Value;

/// Visit all nodes of the json
fn visit_node(value: &Value, counter: &mut i64, check_for_red: bool) {
    if value.is_array() {
        for v in value.as_array().expect("It is already checked that it should be array") {
            visit_node(v, counter, check_for_red);
        }
    } else if value.is_object() {
        let mut bad = false;
        if check_for_red {
            for v in value.as_object().expect("It is already checked that it should be object").values() {
                if v.is_string() && v == "red" {
                    bad = true;
                    break;
                }
            }
        }
        if !bad {
            for v in value.as_object().expect("It is already checked that it should be object").values() {
                visit_node(v, counter, check_for_red);
            }
        }
    } else if value.is_i64() {
        *counter += value.as_i64().expect("It is already checked that it should be i64");
    }
}

fn part_one(json: &Value) -> i64 {
    let mut counter = 0;
    visit_node(json, &mut counter, false);
    counter
}

fn part_two(json: &Value) -> i64 {
    let mut counter = 0;
    visit_node(json, &mut counter, true);
    counter
}

fn main() -> Result<()> {
    let reader = get_reader("puzzles/day12.txt")?;
    let json: Value = serde_json::from_reader(reader)?;
    print_day();
    let res1 = part_one(&json);
    res1.aoc_print(P1);
    let res2 = part_two(&json);
    res2.aoc_print(P2);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: [(&str, i64, &str); 8] = [
        (r##"[1,2,3]"##, 6, "Test11"),
        (r##"{"a":2,"b":4}"##, 6, "Test12"),
        (r##"[[[3]]]"##, 3, "Test13"),
        (r##"{"a":{"b":4},"c":-1}"##, 3, "Test14"),
        (r##"{"a":[-1,1]}"##, 0, "Test15"),
        (r##"[-1,{"a":1}]"##, 0, "Test16"),
        (r##"[]"##, 0, "Test17"),
        (r##"{}"##, 0, "Test18"),
    ];

    #[test]
    fn test_part_one() {
        for (data, res, text) in DATA1 {
            let json: Value = serde_json::from_reader(data.as_bytes()).unwrap();
            assert_eq!(part_one(&json), res, "{}", text);
        }
    }

    const DATA2: [(&str, i64, &str); 4] = [
        (r##"[1,2,3]"##, 6, "Test21"),
        (r##"[1,{"c":"red","b":2},3]"##, 4, "Test22"),
        (r##"{"d":"red","e":[1,2,3,4],"f":5}"##, 0, "Test23"),
        (r##"[1,"red",5]"##, 6, "Test24"),
    ];

    #[test]
    fn test_part_two() {
        for (data, res, text) in DATA2 {
            let json: Value = serde_json::from_reader(data.as_bytes()).unwrap();
            assert_eq!(part_two(&json), res, "{}", text);
        }
    }
}
