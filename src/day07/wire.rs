//! Day 7: Some Assembly Required
//!
//! Wires implementation

use std::{
    cell::Cell,
    collections::HashMap,
    fmt::{Debug, Display},
    rc::Rc,
};

// Signal should be Option, but according to task it's a flat type
// because in part two we need to reset signals on all wires
// and the answer required to set all to 0
pub type Signal = u16;

/// Reference to Signal with Internal Mutability
pub type RefWire = Rc<Cell<Signal>>;

/// All wires mentioned in the input file
pub struct Wires {
    wires: HashMap<String, RefWire>,
}
impl Wires {
    /// Constructor
    pub fn new() -> Self {
        Self { wires: HashMap::new() }
    }
    /// Get referens to the wire
    pub fn get(&mut self, id: &str) -> RefWire {
        // self.wires.get(k);
        self.wires.entry(id.to_string()).or_default().clone()
    }
    /// Reset all wires to 0
    pub fn reset_all(&mut self) {
        for (_, v) in &mut self.wires {
            v.set(Signal::default());
        }
    }
}
impl Debug for Wires {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut wire_strings = Vec::new();
        for (k, v) in &self.wires {
            wire_strings.push(format!("{:2}:{}", k, v.get()));
        }
        let res = wire_strings.join(", ");
        write!(f, "{{{}}}", res)
    }
}
impl Display for Wires {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut wire_strings = Vec::new();
        for (k, v) in &self.wires {
            wire_strings.push(format!("{:2}:{}", k, v.get()));
        }
        let res = wire_strings.join(", ");
        write!(f, "{{{}}}", res)
    }
}
