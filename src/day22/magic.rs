//! Day 22: Wizard Simulator 20XX
//! Magic lib

use tracing::debug;

/// Varians of magic
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Magic {
    Missile {
        cost: i32,
        damage: i32,
    },
    Drain {
        cost: i32,
        damage: i32,
        healing: i32,
    },
    Shield {
        cost: i32,
        rearmoring: i32,
        duration: i32,
    },
    Poison {
        cost: i32,
        damage: i32,
        duration: i32,
    },
    Recharge {
        cost: i32,
        mana: i32,
        duration: i32,
    },
}
impl Magic {
    /// Get cost of exact magic
    pub fn cost(&self) -> i32 {
        match self {
            Magic::Missile { cost, .. }
            | Magic::Drain { cost, .. }
            | Magic::Shield { cost, .. }
            | Magic::Poison { cost, .. }
            | Magic::Recharge { cost, .. } => *cost,
        }
    }
}

impl Drop for Magic {
    // just for logging when something wears off
    fn drop(&mut self) {
        match self {
            Magic::Recharge { .. } => debug!("Recharge weres off"),
            Magic::Poison { .. } => debug!("Poison weres off"),
            Magic::Shield { rearmoring, .. } => {
                debug!("Shields weres off, decreasing armor by {}", rearmoring)
            }
            _ => (),
        }
    }
}

/// Characteristics of The Magic Missile
pub const MISSILE: Magic = Magic::Missile {
    cost: 53,
    damage: 4,
};

/// Characteristics of The Drain
pub const DRAIN: Magic = Magic::Drain {
    cost: 73,
    damage: 2,
    healing: 2,
};

/// Characteristics of The Shield
pub const SHIELD: Magic = Magic::Shield {
    cost: 113,
    rearmoring: 7,
    duration: 6,
};

/// Characteristics of The Poison
pub const POISON: Magic = Magic::Poison {
    cost: 173,
    damage: 3,
    duration: 6,
};

/// Characteristics of The Recharge
pub const RECHARGE: Magic = Magic::Recharge {
    cost: 229,
    mana: 101,
    duration: 5,
};

/// Full set of magic
pub const ALL_MAGIC: [Magic; 5] = [MISSILE, DRAIN, SHIELD, POISON, RECHARGE];
