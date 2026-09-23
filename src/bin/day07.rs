//! Day 7: Some Assembly Required
//!
//! <https://adventofcode.com/2015/day/7>

use anyhow::Result;
use aoc2015::{
    AOCPrint,
    day07::{
        circuit::{Circuit, read_circuit},
        wire::Signal,
    },
    get_reader, print_day,
};
use aoc2015::{P1, P2};

/// Solve part one
// Part two needs results from part one so it gets circuit by mut reference
fn part_one(circut: &mut Circuit) -> Signal {
    circut.process();
    circut.wires.get("a").get()
}

/// Solve part two
// It needs results from part one so it gets circuit processed by part one
fn part_two(circut: &mut Circuit, value: Signal) -> Signal {
    circut.wires.reset_all();
    circut.wires.get("b").set(value);
    circut.process();
    circut.wires.get("a").get()
}

fn main() -> Result<()> {
    let reader = get_reader("puzzles/day07.txt")?;
    let mut circut = read_circuit(reader)?;
    print_day();
    let res_1 = part_one(&mut circut);
    res_1.aoc_print(P1);
    let res_2 = part_two(&mut circut, res_1);
    res_2.aoc_print(P2);

    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    const TEST_DATA: &str = "\
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
";
    #[test]
    fn test_day07() {
        let mut circut = read_circuit(TEST_DATA.as_bytes()).unwrap();
        assert_eq!(part_one(&mut circut), 0);
        assert_eq!(circut.wires.get("d").get(), 72);
        assert_eq!(circut.wires.get("e").get(), 507);
        assert_eq!(circut.wires.get("f").get(), 492);
        assert_eq!(circut.wires.get("g").get(), 114);
        assert_eq!(circut.wires.get("h").get(), 65412);
        assert_eq!(circut.wires.get("i").get(), 65079);
        assert_eq!(circut.wires.get("x").get(), 123);
        assert_eq!(circut.wires.get("y").get(), 456);
    }
}
