//! Day 7: Some Assembly Required
//!
//! Gates implementation

use super::wire::{RefWire, Signal};

pub trait Gate {
    /// Calculate output from input
    fn calculate(&self) -> Signal;
    /// Get reference to output wire
    fn get_output(&self) -> Option<RefWire>;
    /// Put signal to output wire if cinnected
    /// If change happend returns true
    fn process(&self) -> bool {
        if let Some(outp) = &self.get_output() {
            let res_sig = self.calculate();
            if outp.get() != res_sig {
                outp.set(res_sig);
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

/// Possible options of input connection for Gates
pub enum InputField {
    Wire(RefWire), // connected to wire
    Value(Signal), // has direct signal number
    None,          // disconnected
}

/// Gate with AND operation
pub struct AndGate {
    input1: InputField,
    input2: InputField,
    output: Option<RefWire>,
}
impl AndGate {
    pub fn new(input1: InputField, input2: InputField, output: Option<RefWire>) -> Self {
        Self { input1, input2, output }
    }
}
impl Gate for AndGate {
    fn calculate(&self) -> Signal {
        let inp_signal_1 = match &self.input1 {
            InputField::Wire(rw) => rw.get(),
            InputField::Value(s) => *s,
            InputField::None => Signal::default(),
        };
        let inp_signal_2 = match &self.input2 {
            InputField::Wire(rw) => rw.get(),
            InputField::Value(s) => *s,
            InputField::None => Signal::default(),
        };

        inp_signal_1 & inp_signal_2
    }
    fn get_output(&self) -> Option<RefWire> {
        self.output.clone()
    }
}

/// Gate with OR operation
pub struct OrGate {
    input1: InputField,
    input2: InputField,
    output: Option<RefWire>,
}
impl OrGate {
    pub fn new(input1: InputField, input2: InputField, output: Option<RefWire>) -> Self {
        Self { input1, input2, output }
    }
}
impl Gate for OrGate {
    fn calculate(&self) -> Signal {
        let inp_signal_1 = match &self.input1 {
            InputField::Wire(rw) => rw.get(),
            InputField::Value(s) => *s,
            InputField::None => Signal::default(),
        };
        let inp_signal_2 = match &self.input2 {
            InputField::Wire(rw) => rw.get(),
            InputField::Value(s) => *s,
            InputField::None => Signal::default(),
        };

        inp_signal_1 | inp_signal_2
    }
    fn get_output(&self) -> Option<RefWire> {
        self.output.clone()
    }
}

/// Gate with LSHIFT operation
pub struct LShiftGate {
    input: InputField,
    shift: InputField,
    output: Option<RefWire>,
}
impl LShiftGate {
    pub fn new(input: InputField, shift: InputField, output: Option<RefWire>) -> Self {
        Self { input, shift, output }
    }
}
impl Gate for LShiftGate {
    fn calculate(&self) -> Signal {
        let inp_signal = match &self.input {
            InputField::Wire(rw) => rw.get(),
            InputField::Value(s) => *s,
            InputField::None => Signal::default(),
        };

        let shift = match &self.shift {
            InputField::Value(s) => *s,
            InputField::None => Signal::default(),
            _ => panic!("Wrong type of field"),
        };

        inp_signal << shift
    }
    fn get_output(&self) -> Option<RefWire> {
        self.output.clone()
    }
}

/// Gate with RSHIFT operation
pub struct RShiftGate {
    input: InputField,
    shift: InputField,
    output: Option<RefWire>,
}
impl RShiftGate {
    pub fn new(input: InputField, shift: InputField, output: Option<RefWire>) -> Self {
        Self { input, shift, output }
    }
}
impl Gate for RShiftGate {
    fn calculate(&self) -> Signal {
        let inp_signal = match &self.input {
            InputField::Wire(rw) => rw.get(),
            InputField::Value(s) => *s,
            InputField::None => Signal::default(),
        };

        let shift = match &self.shift {
            InputField::Value(s) => *s,
            InputField::None => Signal::default(),
            _ => panic!("Wrong type of field"),
        };

        inp_signal >> shift
    }
    fn get_output(&self) -> Option<RefWire> {
        self.output.clone()
    }
}

/// Gate with NOT operation
pub struct NotGate {
    input: InputField,
    output: Option<RefWire>,
}
impl NotGate {
    pub fn new(input: InputField, output: Option<RefWire>) -> Self {
        Self { input, output }
    }
}
impl Gate for NotGate {
    fn calculate(&self) -> Signal {
        let inp_signal = match &self.input {
            InputField::Wire(rw) => rw.get(),
            InputField::Value(s) => *s,
            InputField::None => Signal::default(),
        };
        !inp_signal
    }
    fn get_output(&self) -> Option<RefWire> {
        self.output.clone()
    }
}

/// Gate without operations
pub struct DirectGate {
    input: InputField,
    output: Option<RefWire>,
}
impl DirectGate {
    pub fn new(input: InputField, output: Option<RefWire>) -> Self {
        Self { input, output }
    }
}
impl Gate for DirectGate {
    fn calculate(&self) -> Signal {
        match &self.input {
            InputField::Wire(rw) => rw.get(),
            InputField::Value(s) => *s,
            InputField::None => Signal::default(),
        }
    }
    fn get_output(&self) -> Option<RefWire> {
        self.output.clone()
    }
}
