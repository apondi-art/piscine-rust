#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;
    
    fn next(&mut self) -> Option<Self::Item> {
        // Gravity in m/s²
        const GRAVITY: f32 = 9.8;
        
        // Increase time by 1 second
        self.time += 1.0;
        
        // Calculate new position and velocity
        // Horizontal motion: constant velocity
        self.actual_position.x = self.init_position.x + self.init_velocity.x * self.time;
        
        // Vertical motion: affected by gravity
        // Position formula: y = y₀ + v₀t - ½gt²
        self.actual_position.y = self.init_position.y + 
                               self.init_velocity.y * self.time - 
                               0.5 * GRAVITY * self.time * self.time;
        
        // Velocity formula: v = v₀ - gt
        self.actual_velocity.x = self.init_velocity.x;
        self.actual_velocity.y = self.init_velocity.y - GRAVITY * self.time;
        
        // Check if the object has reached the ground or below (y ≤ 0)
        if self.actual_position.y <= 0.0 {
            None
        } else {
            Some(self.clone())
        }
    }
}