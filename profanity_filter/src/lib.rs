// src/lib.rs
#[derive(Debug)]
pub struct Message {
    pub content: String,
    pub user: String,
}

impl Message {
    pub fn new(content: String, user: String) -> Self {
        Self { content, user }
    }

    pub fn send_ms(&self) -> Option<&str> {
        if self.content.is_empty() || self.content.to_lowercase().contains("stupid") {
            None
        } else {
            Some(&self.content)
        }
    }
}

pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.is_empty() || message.to_lowercase().contains("stupid") {
        Err("ERROR: illegal")
    } else {
        Ok(message)
    }
}