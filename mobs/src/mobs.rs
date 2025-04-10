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
        self.members.push(Member::new(&name, Role::Associate, age));
    }

    fn power_score(&self) -> u32 {
        self.members.iter().map(|m| m.role.score()).sum()
    }

    pub fn attack(&mut self, other: &mut Mob) {
        let self_score = self.power_score();
        let other_score = other.power_score();

        let (loser, winner) = if self_score > other_score {
            (other, self)
        } else if self_score < other_score {
            (self, other)
        } else {
            (self, other) // Attacker loses in draw
        };

        if !loser.members.is_empty() {
            loser.members.pop();
        }

        if loser.members.is_empty() {
            winner.wealth += loser.wealth;
            loser.wealth = 0;

            for city in loser.cities.drain(..) {
                winner.cities.push(city);
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
