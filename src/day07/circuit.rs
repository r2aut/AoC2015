use super::gate::{AndGate, DirectGate, Gate, InputField, LShiftGate, NotGate, OrGate, RShiftGate};
use super::wire::Wires;
use crate::day07::wire::Signal;
use anyhow::Result;
use regex::Regex;
use std::io::BufRead;

/// Complete circuit
pub struct Circuit {
    pub wires: Wires,          // all wires
    gates: Vec<Box<dyn Gate>>, // all gates
}
impl Circuit {
    /// Constructor
    fn new() -> Self {
        Self {
            wires: Wires::new(),
            gates: Vec::new(),
        }
    }
    /// Calculate all signals until stable condition
    pub fn process(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            for g in &self.gates {
                if g.process() {
                    changed = true;
                }
            }
        }
    }
}

/// Read circuit from file
pub fn read_circuit(reader: impl BufRead) -> Result<Circuit> {
    let mut circuit = Circuit::new();

    let pattern_and_or_shift = Regex::new(r"(\w+) (\w+) (\w+) -> (\w+)")?;
    let pattern_direct = Regex::new(r"(\w+) -> (\w+)")?;
    let pattern_not = Regex::new(r"NOT (\w+) -> (\w+)")?;

    for line in reader.lines() {
        let line = line?;
        // commands AND, OR, LSHIFT and RSHIFT
        if let Some(cap) = pattern_and_or_shift.captures(&line) {
            // get input1
            let input1_str = cap[1].to_string();
            let input1;
            if let Ok(res) = input1_str.parse::<Signal>() {
                input1 = InputField::Value(res);
            } else {
                input1 = InputField::Wire(circuit.wires.get(&input1_str).clone());
            }

            // get command
            let command_str = cap[2].to_string();

            // get input2
            let input2_str = cap[3].to_string();
            let input2;
            if let Ok(res) = input2_str.parse::<Signal>() {
                input2 = InputField::Value(res);
            } else {
                input2 = InputField::Wire(circuit.wires.get(&input2_str).clone());
            }

            // get output
            let output_str = cap[4].to_string();
            let output = Some(circuit.wires.get(&output_str).clone());

            // construct Gate according to command
            let gate: Box<dyn Gate> = match command_str.as_str() {
                "AND" => Box::new(AndGate::new(input1, input2, output)),
                "OR" => Box::new(OrGate::new(input1, input2, output)),
                "LSHIFT" => Box::new(LShiftGate::new(input1, input2, output)),
                "RSHIFT" => Box::new(RShiftGate::new(input1, input2, output)),
                _ => panic!("Unknown input string: {}", line),
            };

            // save gate to circuit
            circuit.gates.push(gate);
        } else if let Some(cap) = pattern_not.captures(&line) {
            let input_str = cap[1].to_string();
            let input;
            if let Ok(res) = input_str.parse::<Signal>() {
                input = InputField::Value(res);
            } else {
                input = InputField::Wire(circuit.wires.get(&input_str).clone());
            }

            let output_str = cap[2].to_string();
            let output = Some(circuit.wires.get(&output_str).clone());

            let gate = Box::new(NotGate::new(input, output));
            circuit.gates.push(gate);
        } else if let Some(cap) = pattern_direct.captures(&line) {
            let input_str = cap[1].to_string();
            let input;
            if let Ok(res) = input_str.parse::<Signal>() {
                input = InputField::Value(res);
            } else {
                input = InputField::Wire(circuit.wires.get(&input_str).clone());
            }

            let output_str = cap[2].to_string();
            let output = circuit.wires.get(&output_str).clone();

            if let InputField::Wire(_) = input {
                let gate = Box::new(DirectGate::new(input, Some(output)));
                circuit.gates.push(gate);
            } else if let InputField::Value(v) = input {
                output.set(v);
            }
        } else {
            panic!("Unknown input string: {}", line)
        }
    }
    Ok(circuit)
}
