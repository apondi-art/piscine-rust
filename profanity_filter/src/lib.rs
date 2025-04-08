
#[derive(Debug)] 
pub struct Message {
    pub content: String,
    pub user: String,  // Mark as pub or remove if unused
}

impl Message {
    // Constructor to create a new Message
    pub fn new(content: String, user: String) -> Message {
        Message { content, user }
    }

    // Method to check if message should be sent
   pub  fn send_ms(&self) -> Option<&str> {
        // Check if content is empty or contains "stupid" (case insensitive)
        if self.content.is_empty() || self.content.to_lowercase().contains("stupid") {
            None  // Block the message
        } else {
            Some(&self.content)  // Allow the message
        }
    }
}

// Function to check a message and return the required result
pub fn check_ms(message: &Message) -> (bool, &str) {
    match message.send_ms() {
        Some(content) => (true, content),  // Message is allowed
        None => (false, "ERROR: illegal"), // Message is blocked
    }
}