#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}

impl Role {
    pub fn score(&self) -> u32 {
        match self {
            Role::Underboss => 4,
            Role::Caporegime => 3,
            Role::Soldier => 2,
            Role::Associate => 1,
        }
    }

    pub fn promote(&self) -> Role {
        match self {
            Role::Associate => Role::Soldier,
            Role::Soldier => Role::Caporegime,
            Role::Caporegime => Role::Underboss,
            Role::Underboss => Role::Underboss,
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
        self.role = self.role.promote();
    }
}
