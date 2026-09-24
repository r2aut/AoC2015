//! Day 23: Opening the Turing Lock
//!
//! <https://adventofcode.com/2015/day/23>

use anyhow::{Ok, Result, bail};
use std::{io::BufRead, ops::Index};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Register {
    A,
    B,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
enum Command {
    HLF(Register),
    TPL(Register),
    INC(Register),
    JMP(isize),
    JIE(Register, isize),
    JIO(Register, isize),
}

#[derive(Debug, Clone)]
struct Program(Vec<Command>);
impl Program {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn add(&mut self, cmd: Command) {
        self.0.push(cmd);
    }
}
impl Index<usize> for Program {
    type Output = Command;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

struct ProgramParser;
impl ProgramParser {
    /// Parser for Program from AoC format
    fn try_from_aoc_reader(reader: impl BufRead) -> Result<Program> {
        let mut prog = Program::new();
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(' ').collect();
            let cmd = match parts[0].trim() {
                "hlf" => Command::HLF(ProgramParser::reg_by_name(parts[1].trim())),
                "tpl" => Command::TPL(ProgramParser::reg_by_name(parts[1].trim())),
                "inc" => Command::INC(ProgramParser::reg_by_name(parts[1].trim())),
                "jmp" => {
                    let shift = parts[1].trim().parse()?;
                    Command::JMP(shift)
                }
                "jie" => {
                    let shift: isize = parts[2].trim().parse()?;
                    let reg = ProgramParser::reg_by_name(&parts[1].trim()[0..1]);
                    Command::JIE(reg, shift)
                }
                "jio" => {
                    let shift: isize = parts[2].trim().parse()?;
                    let reg = ProgramParser::reg_by_name(&parts[1].trim()[0..1]);
                    Command::JIO(reg, shift)
                }
                _ => bail!("Unknown command {}", line),
            };
            prog.add(cmd);
        }
        Ok(prog)
    }

    /// Get register by its name
    fn reg_by_name(reg_name: &str) -> Register {
        match reg_name.trim() {
            "a" => Register::A,
            "b" => Register::B,
            _ => panic!("Wrong register name {}", reg_name.trim()),
        }
    }
}

struct Computer {
    reg_a: u32,
    reg_b: u32,
    prog: Program,
    addr: usize,
}
impl Computer {
    fn new(prog: &Program) -> Self {
        Self { reg_a: 0, reg_b: 0, prog: (*prog).clone(), addr: 0 }
    }

    /// Get register value for reading
    fn reg(&self, reg: &Register) -> &u32 {
        match reg {
            Register::A => &self.reg_a,
            Register::B => &self.reg_b,
        }
    }

    /// Get register value for mutation
    fn reg_mut(&mut self, reg: &Register) -> &mut u32 {
        match reg {
            Register::A => &mut self.reg_a,
            Register::B => &mut self.reg_b,
        }
    }

    /// Execute one operation
    fn exec_op(&mut self) {
        let cmd = self.prog[self.addr];
        match cmd {
            Command::HLF(reg) => {
                *self.reg_mut(&reg) = *self.reg_mut(&reg) / 2;
                self.addr += 1;
            }
            Command::TPL(reg) => {
                *self.reg_mut(&reg) = *self.reg_mut(&reg) * 3;
                self.addr += 1;
            }
            Command::INC(reg) => {
                *self.reg_mut(&reg) = *self.reg_mut(&reg) + 1;
                self.addr += 1;
            }
            Command::JMP(shift) => {
                let new_adr = self.addr as isize + shift;
                self.addr = new_adr as usize;
            }
            Command::JIE(reg, shift) => {
                if self.reg(&reg).is_multiple_of(2) {
                    let new_adr = self.addr as isize + shift;
                    self.addr = new_adr as usize;
                } else {
                    self.addr += 1;
                }
            }
            Command::JIO(reg, shift) => {
                if *self.reg(&reg) == 1 {
                    let new_adr = self.addr as isize + shift;
                    self.addr = new_adr as usize;
                } else {
                    self.addr += 1;
                }
            }
        }
    }

    /// Execute the whole program
    fn exec_prog(&mut self) {
        while self.addr < self.prog.len() {
            self.exec_op();
        }
    }
}

fn part_one(prog: &Program) -> u32 {
    let mut comp = Computer::new(prog);
    comp.exec_prog();
    *comp.reg(&Register::B)
}
fn part_two(prog: &Program) -> u32 {
    let mut comp = Computer::new(prog);
    *comp.reg_mut(&Register::A) = 1;
    comp.exec_prog();
    *comp.reg(&Register::B)
}

pub fn solve(reader: impl BufRead) -> Result<(Option<String>, Option<String>)> {
    let prog = ProgramParser::try_from_aoc_reader(reader)?;
    let res1 = Some(part_one(&prog).to_string());
    let res2 = Some(part_two(&prog).to_string());
    Ok((res1, res2))
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "\
inc a
jio a, +2
tpl a
inc a
";

    #[test]
    fn test_day23() {
        let reader = DATA.as_bytes();
        let prog = ProgramParser::try_from_aoc_reader(reader).unwrap();
        let mut comp = Computer::new(&prog);
        comp.exec_prog();
        let res = comp.reg(&Register::A);
        assert_eq!(*res, 2);
    }
}
