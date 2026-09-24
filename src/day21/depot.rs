use rand::{RngExt, rng, rngs::ThreadRng};
use std::fmt::Display;

#[allow(dead_code)]
pub trait FightItem {}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Weapon {
    pub name: String,
    pub cost: i32,
    pub damage: i32,
    pub armor: i32,
}
impl FightItem for Weapon {}
impl Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"Weapon "{}" (cost={}, damage={})"#, self.name, self.cost, self.damage)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Armor {
    pub name: String,
    pub cost: i32,
    pub damage: i32,
    pub armor: i32,
}
impl FightItem for Armor {}
impl Display for Armor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"Armor "{}" (cost={}, armor={})"#, self.name, self.cost, self.armor)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Ring {
    pub name: String,
    pub cost: i32,
    pub damage: i32,
    pub armor: i32,
}
impl FightItem for Ring {}
impl Display for Ring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"Ring "{}" (cost={}, damage={}, armor={})"#, self.name, self.cost, self.damage, self.armor)
    }
}

#[derive(Debug, Default)]
pub struct Depot {
    weapons: Vec<Weapon>,
    armors: Vec<Armor>,
    rings: Vec<Ring>,
    rng: ThreadRng,
}
impl Depot {
    pub fn new() -> Self {
        Self { weapons: Vec::new(), armors: Vec::new(), rings: Vec::new(), rng: rng() }
    }

    fn get_weapon(&mut self, index: usize) -> Option<Weapon> {
        self.weapons.get(index).cloned()
    }
    fn get_armor(&mut self, index: usize) -> Option<Armor> {
        self.armors.get(index).cloned()
    }
    fn get_ring(&mut self, index: usize) -> Option<Ring> {
        self.rings.get(index).cloned()
    }
    pub fn get_random_weapon(&mut self) -> Weapon {
        let size = self.weapons.len();
        let index = self.rng.random_range(0..size);
        #[allow(clippy::expect_used)]
        self.get_weapon(index).expect("Weapon should alwais be on depot")
    }
    pub fn get_random_armor(&mut self) -> Armor {
        let size = self.armors.len();
        let index = self.rng.random_range(0..size); // -1 means no armor
        #[allow(clippy::expect_used)]
        self.get_armor(index).expect("Armor should alwais be on depot")
    }
    pub fn get_random_rings(&mut self) -> Vec<Ring> {
        let mut res = Vec::new();
        let size = self.rings.len();
        let mut index1 = self.rng.random_range(0..size);
        let mut index2 = self.rng.random_range(0..size);
        while index1 == index2 {
            index1 = self.rng.random_range(0..size);
            index2 = self.rng.random_range(0..size);
        }
        #[allow(clippy::expect_used)]
        res.push(self.get_ring(index1).expect("Ring should alwais be on depot"));
        #[allow(clippy::expect_used)]
        res.push(self.get_ring(index2).expect("Ring should alwais be on depot"));
        res
    }
}
impl Display for Depot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        res += format!("{:12} {:>6} {:>8} {:>8}\n", "Weapons:", "Cost", "Damage", "Armor").as_str();
        for w in &self.weapons {
            res += format!("{:12} {:6} {:8} {:8}\n", w.name, w.cost, w.damage, 0).as_str();
        }
        res += "\n";
        res += format!("{:12} {:>6} {:>8} {:>8}\n", "Armor:", "Cost", "Damage", "Armor").as_str();
        for a in &self.armors {
            res += format!("{:12} {:6} {:8} {:8}\n", a.name, a.cost, 0, a.armor).as_str();
        }
        res += "\n";
        res += format!("{:12} {:>6} {:>8} {:>8}\n", "Ring:", "Cost", "Damage", "Armor").as_str();
        for r in &self.rings {
            res += format!("{:12} {:6} {:8} {:8}\n", r.name, r.cost, r.damage, r.armor).as_str();
        }

        write!(f, "{res}")
    }
}

pub struct ItemFactory {}
impl ItemFactory {
    const WEAPONS: [(&str, i32, i32, i32); 5] = [
        ("Dagger", 8, 4, 0),
        ("Shortsword", 10, 5, 0),
        ("Warhammer", 25, 6, 0),
        ("Longsword", 40, 7, 0),
        ("Greataxe", 74, 8, 0),
    ];
    const ARMORS: [(&str, i32, i32, i32); 6] = [
        ("No armor", 0, 0, 0),
        ("Leather", 13, 0, 1),
        ("Chainmail", 31, 0, 2),
        ("Splintmail", 53, 0, 3),
        ("Bandedmail", 75, 0, 4),
        ("Platemail", 102, 0, 5),
    ];
    const RINGS: [(&str, i32, i32, i32); 8] = [
        ("No ring 1", 0, 0, 0),
        ("No ring 2", 0, 0, 0),
        ("Damage +1", 25, 1, 0),
        ("Damage +2", 50, 2, 0),
        ("Damage +3", 100, 3, 0),
        ("Defense +1", 20, 0, 1),
        ("Defense +2", 40, 0, 2),
        ("Defense +3", 80, 0, 3),
    ];
    pub fn fill_depot(depot: &mut Depot) {
        for (name, cost, damage, armor) in ItemFactory::WEAPONS {
            depot.weapons.push(Weapon { name: name.to_string(), cost, damage, armor });
        }
        for (name, cost, damage, armor) in ItemFactory::ARMORS {
            depot.armors.push(Armor { name: name.to_string(), cost, damage, armor });
        }
        for (name, cost, damage, armor) in ItemFactory::RINGS {
            depot.rings.push(Ring { name: name.to_string(), cost, damage, armor });
        }
    }
}
