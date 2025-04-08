// src/lib.rs
#[derive(Debug)]
pub struct Message {
    pub content: String,
    pub user: String,
}

impl Message {
    pub fn new(content: String, user: String) -> Message {
        Message { content, user }
    }

    pub fn send_ms(&self) -> Option<&str> {
        if self.content.is_empty() || self.content.to_lowercase().contains("stupid") {
            None
        } else {
            Some(&self.content)
        }
    }
}

pub fn check_ms(message: &Message) -> (bool, &str) {
    match message.send_ms() {
        Some(content) => (true, content),
        None => (false, "ERROR: illegal"),
    }
}