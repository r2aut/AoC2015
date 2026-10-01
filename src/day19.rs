//! Day 19: Medicine for Rudolph
//!
//! <https://adventofcode.com/2015/day/19>

use anyhow::Result;
use log::{debug, trace};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{Debug, Display},
    io::BufRead,
    ops::Index,
};

type Molecule = String;

/// Medicine is an ordered collection of Molecules
#[derive(PartialEq, Eq, Clone, Hash)]
struct Medicine(Vec<Molecule>);
impl Medicine {
    // replace item in medicine by index using map
    fn replace_by_index(&self, ind: usize, rep: &RepForward) -> Vec<Medicine> {
        let mut res = Vec::new();
        if let Some(molecule) = &self.0.get(ind) {
            for repl in rep.get_iter(molecule) {
                let mut new_medicine = Medicine(Vec::new());
                new_medicine.0.extend_from_slice(&self.0[0..ind]);
                new_medicine.0.extend_from_slice(&repl.0[..]);
                new_medicine.0.extend_from_slice(&self.0[ind + 1..]);
                res.push(new_medicine);
            }
        }
        res
    }
    fn len(&self) -> usize {
        self.0.len()
    }

    // replace back (simplify) medicine using map with backward translation
    fn replace_back(&self, bw_rep: &RepBackward) -> Vec<Medicine> {
        let mut res = Vec::new();
        for ind in (0..self.len()).rev() {
            let sl = &self.0[ind..];
            for rep in bw_rep.0.keys() {
                if sl.starts_with(&rep.0) {
                    let llen = rep.0.len();
                    let new_v = bw_rep.get(rep).expect("Shoiuld work");
                    let mut new_medicine = Medicine(Vec::new());
                    new_medicine.0.extend_from_slice(&self.0[0..ind]);
                    new_medicine.0.extend_from_slice(&new_v[..]);
                    new_medicine.0.extend_from_slice(&self.0[ind + llen..]);
                    res.push(new_medicine);
                }
            }
        }
        res
    }
}
impl From<&str> for Medicine {
    fn from(value_str: &str) -> Self {
        let mut parts = Vec::new();
        let mut last_index = 0;

        // Итерируемся по символам и их байтовым индексам
        for (idx, ch) in value_str.char_indices() {
            // Если нашли заглавную букву и это не самое начало строки
            if ch.is_uppercase() && idx > 0 {
                parts.push(value_str[last_index..idx].to_string());
                last_index = idx;
            }
        }

        // Добавляем оставшуюся часть строки
        if !value_str.is_empty() {
            parts.push(value_str[last_index..].to_string());
        }

        Self(parts)
    }
}
impl Index<usize> for Medicine {
    type Output = Molecule;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Display for Medicine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join(""))
    }
}
impl Debug for Medicine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join(""))
    }
}

/// Map for converting one Molecule into several (Medicine)
#[derive(Debug)]
struct RepForward(HashMap<Molecule, Vec<Medicine>>);
impl RepForward {
    fn new() -> Self {
        Self(HashMap::new())
    }
    // fn get(&self, molecule: &Molecule) -> Option<&Vec<Medicine>> {
    //     self.0.get(molecule)
    // }
    fn get_iter<'a>(&'a self, molecule: &Molecule) -> impl Iterator<Item = &'a Medicine> {
        self.0.get(molecule).into_iter().flatten()
    }
    fn add(&mut self, sorce: Molecule, dest: Medicine) {
        self.0.entry(sorce).or_default().push(dest);
    }
}

struct RepForwardParser;
impl RepForwardParser {
    fn try_from_aoc_reader(reader: &mut impl BufRead) -> Result<RepForward> {
        let mut rep = RepForward::new();
        for line in reader.by_ref().lines() {
            let line = line?;
            let parts: Vec<&str> = line.split("=>").map(|s| s.trim()).collect();
            if parts.len() == 2 {
                let molecule = parts[0].to_string();
                let value = parts[1].to_string();
                let med = Medicine::from(value.as_str());
                rep.add(molecule, med);
            } else {
                break;
            }
        }
        Ok(rep)
    }
}

/// Solve Part One
fn part_one(med: &Medicine, rep: &RepForward) -> usize {
    let mut res: HashSet<Medicine> = HashSet::new();
    for (idx, _mol) in med.0.iter().enumerate() {
        let bbb = med.replace_by_index(idx, rep);
        for b in bbb {
            res.insert(b);
        }
    }
    res.len()
}

#[derive(Debug)]
struct RepBackward(HashMap<Medicine, Vec<Molecule>>);
impl RepBackward {
    fn new() -> Self {
        Self(HashMap::new())
    }
    fn get(&self, med: &Medicine) -> Option<&Vec<Molecule>> {
        self.0.get(med)
    }
    fn add(&mut self, sorce: Medicine, dest: Molecule) {
        let souce_ref = self.0.entry(sorce).or_default();
        souce_ref.push(dest);
    }
}
impl From<&RepForward> for RepBackward {
    fn from(rep: &RepForward) -> Self {
        let mut res = RepBackward::new();
        for (k, v_vec) in rep.0.iter() {
            for v in v_vec {
                res.add(v.clone(), k.clone());
            }
        }
        res
    }
}

/// Status state for medicine
struct Status {
    med: Medicine,
    counter: usize,
}

/// Solve Part Two
fn part_two(med: &Medicine, rep: &RepForward) -> Option<usize> {
    debug!("Part two called for medicine: {}", med);
    let bw_rep = RepBackward::from(rep);
    trace!("RepBackward map created {:?}", bw_rep);

    let mut queue: VecDeque<Status> = VecDeque::new();
    let status = Status { med: med.clone(), counter: 0 };
    trace!("Medicine {} queued. Counter: {}", status.med, status.counter);
    queue.push_back(status);

    while let Some(status) = queue.pop_front() {
        trace!("Medicine {} processed. Counter: {}", status.med, status.counter);
        let new_med_vec = status.med.replace_back(&bw_rep);

        if !new_med_vec.is_empty() {
            // find min length medicine (max decreasing)
            let best_next_step =
                new_med_vec.iter().min_by_key(|molecule| molecule.len()).expect("Vector should not be not empty");
            // check for finish condition
            if *best_next_step == Medicine::from("e") {
                debug!("Medicine {} acheived. Counter: {}", best_next_step, status.counter + 1);
                return Some(status.counter + 1);
            } else {
                // process next step
                let new_status = Status { med: best_next_step.clone(), counter: status.counter + 1 };
                queue.push_back(new_status);
                trace!("Medicine {} put for next step. Counter: {}", best_next_step, status.counter + 1);
            }
        }
    }
    None
}

pub fn solve(mut reader: impl BufRead) -> Result<(Option<String>, Option<String>)> {
    let rep = RepForwardParser::try_from_aoc_reader(&mut reader)?;
    let mut m_str = Molecule::new();
    reader.read_line(&mut m_str)?;
    let med = Medicine::from(m_str.trim());

    let res1 = Some(part_one(&med, &rep).to_string());
    let res2 = part_two(&med, &rep).map(|n| n.to_string());

    Ok((res1, res2))
}

#[cfg(test)]
mod test {

    use super::*;

    const REPR: &str = "\
e => H
e => O
H => HO
H => OH
O => HH
";
    #[test]
    fn test_part_one() {
        let mut reader = REPR.as_bytes();
        let repr = RepForwardParser::try_from_aoc_reader(&mut reader).unwrap();
        assert_eq!(part_one(&Medicine::from("HOH"), &repr), 4);
        assert_eq!(part_one(&Medicine::from("HOHOHO"), &repr), 7);
    }
    #[test]
    fn test_part_two() {
        let mut reader = REPR.as_bytes();
        let repr = RepForwardParser::try_from_aoc_reader(&mut reader).unwrap();
        assert_eq!(part_two(&Medicine::from("HOH"), &repr), Some(3));
        assert_eq!(part_two(&Medicine::from("HOHOHO"), &repr), Some(6));
    }
}
