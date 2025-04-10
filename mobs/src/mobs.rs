pub mod boss;
pub mod member;

use boss::Boss;
use member::{Member, Role};

#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: Vec<Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32,
}

impl Mob {
    pub fn recruit(&mut self, name: String, age: u8) {
        self.members.push(Member::new(name, Role::Associate, age));
    }

    fn power_score(&self) -> u32 {
        self.members.iter().map(|m| m.role.score()).sum()
    }

    pub fn attack(&mut self, other: &mut Mob) {
        let self_score = self.power_score();
        let other_score = other.power_score();

        let loser = if self_score > other_score {
            other
        } else if self_score < other_score {
            self
        } else {
            self
        };

        if !loser.members.is_empty() {
            loser.members.pop();
        }

        if self_score == other_score || self.members.is_empty() || other.members.is_empty() {
            if other.members.is_empty() && !self.members.is_empty() {
                self.wealth += other.wealth;
                other.wealth = 0;
                for city in &other.cities {
                    self.cities.push(city.clone());
                }
                other.cities.clear();
            } else if self.members.is_empty() && !other.members.is_empty() {
                other.wealth += self.wealth;
                self.wealth = 0;
                for city in &self.cities {
                    other.cities.push(city.clone());
                }
                self.cities.clear();
            }
        }
    }

    pub fn steal(&mut self, target: &mut Mob, amount: u32) {
        let actual_amount = amount.min(target.wealth);
        self.wealth += actual_amount;
        target.wealth -= actual_amount;
    }

    pub fn conquer_city(&mut self, mobs: &[Mob], city_name: String, level: u8) {
        if mobs.iter().all(|m| !m.cities.iter().any(|(c, _)| c == &city_name)) {
            self.cities.push((city_name, level));
        }
    }
}
