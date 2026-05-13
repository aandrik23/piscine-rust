pub mod boss;
pub mod member;

use std::collections::{HashMap, HashSet};

pub use boss::*;
pub use member::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn recruit(&mut self, (name, age): (&str, u32)) {
        self.members.insert(
            name.to_owned(),
            Member {
                role: Role::Associate,
                age,
            },
        );
    }

    fn calculate_power(&self) -> usize {
        self.members
            .values()
            .map(|m| match m.role {
                Role::Underboss => 4,
                Role::Caporegime => 3,
                Role::Soldier => 2,
                Role::Associate => 1,
            })
            .sum()
    }

    pub fn attack(&mut self, target: &mut Mob) {
        let self_power = self.calculate_power();
        let target_power = target.calculate_power();

        if self_power > target_power {
            let youngest = target.members.values().map(|m| m.age).min().unwrap();
            target.members.retain(|_, m| m.age > youngest);
            if target.members.is_empty() {
                self.cities.extend(target.cities.drain());
                self.wealth += target.wealth;
                target.wealth = 0;
            }
        } else {
            let youngest = self.members.values().map(|m| m.age).min().unwrap();
            self.members.retain(|_, m| m.age > youngest);
            if self.members.is_empty() {
                target.cities.extend(self.cities.drain());
                target.wealth += self.wealth;
                self.wealth = 0;
            }
        }
    }

    pub fn steal(&mut self, target: &mut Mob, value: u64) {
        let clamped = value.min(target.wealth);
        self.wealth += clamped;
        target.wealth -= clamped;
    }

    pub fn conquer_city(&mut self, mobs: &[&Mob], wanted_city: String) {
        if !mobs
            .iter()
            .flat_map(|m| &m.cities)
            .any(|c| *c == wanted_city)
        {
            self.cities.insert(wanted_city);
        }
    }
}