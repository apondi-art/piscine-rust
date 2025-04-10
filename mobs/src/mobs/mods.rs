pub mod boss;
pub mod member;

use std::vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: boss::Boss,
    pub members: Vec<member::Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32,
}

impl Mob {
    pub fn recruit(&mut self, name: &str, age: u8) {
        self.members.push(member::Member::new(name, member::Role::Associate, age));
    }

    pub fn attack(&mut self, other: &mut Mob) {
        let self_power: u32 = self.members.iter().map(|m| m.role.combat_score()).sum();
        let other_power: u32 = other.members.iter().map(|m| m.role.combat_score()).sum();

        if self_power <= other_power {
            self.members.pop();
        } else {
            other.members.pop();
        }

        if self.members.is_empty() && !other.members.is_empty() {
            other.wealth += self.wealth;
            other.cities.append(&mut self.cities);
            self.wealth = 0;
        } else if other.members.is_empty() && !self.members.is_empty() {
            self.wealth += other.wealth;
            self.cities.append(&mut other.cities);
            other.wealth = 0;
        }
    }

    pub fn steal(&mut self, target: &mut Mob, amount: u32) {
        let stolen = amount.min(target.wealth);
        target.wealth -= stolen;
        self.wealth += stolen;
    }

    pub fn conquer_city(&mut self, mobs: &[Mob], city_name: String, value: u8) {
        let exists = mobs.iter().any(|mob| 
            mob.cities.iter().any(|(name, _)| name == &city_name)
        );
        if !exists {
            self.cities.push((city_name, value));
        }
    }
}