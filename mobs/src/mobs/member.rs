#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}

impl Role {
    pub fn combat_score(&self) -> u32 {
        match self {
            Role::Underboss => 4,
            Role::Caporegime => 3,
            Role::Soldier => 2,
            Role::Associate => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub name: String,
    pub role: Role,
    pub age: u8,
}

impl Member {
    pub fn new(name: &str, role: Role, age: u8) -> Self {
        Member {
            name: name.to_string(),
            role,
            age,
        }
    }

    pub fn get_promotion(&mut self) {
        self.role = match self.role {
            Role::Associate => Role::Soldier,
            Role::Soldier => Role::Caporegime,
            Role::Caporegime => Role::Underboss,
            r => r,
        };
    }
}