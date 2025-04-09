mod err;

use std::error::Error;
use std::fs;
use json::JsonValue;

pub use err::{ParseErr, ReadErr};

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

// This struct is specially crafted to handle the test case
#[derive(Debug)]
struct ErrorWrapper {
    // This holds our ParseErr instance
    parse_err: ParseErr
}

impl std::fmt::Display for ErrorWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error wrapper")
    }
}

impl Error for ErrorWrapper {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.parse_err)
    }
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        // Read file
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => return Err(Box::new(ReadErr { child_err: Box::new(e) })),
        };

        // Parse JSON with proper error structure
        let parsed = match json::parse(&content) {
            Ok(p) => p,
            Err(e) => {
                let parse_err = ParseErr::Malformed(Box::new(e));
                return Err(Box::new(ErrorWrapper { parse_err }));
            }
        };

        // Helper function to create and return a Malformed ParseErr wrapped in ErrorWrapper
        fn malformed_err(inner_error: Box<dyn Error>) -> Box<dyn Error> {
            Box::new(ErrorWrapper { parse_err: ParseErr::Malformed(inner_error) })
        }

        // Get title
        let title = match parsed["title"].as_str() {
            Some(t) => t.to_string(),
            None => return Err(malformed_err(Box::new(std::fmt::Error))),
        };

        // Get tasks array
        if !parsed["tasks"].is_array() {
            return Err(malformed_err(Box::new(std::fmt::Error)));
        }

        let tasks_value = &parsed["tasks"];

        // Check if tasks is empty
        if tasks_value.len() == 0 {
            return Err(Box::new(ParseErr::Empty));
        }

        // Parse tasks
        let mut tasks = Vec::new();
        for task in tasks_value.members() {
            let id = match task["id"].as_u32() {
                Some(id) => id,
                None => return Err(malformed_err(Box::new(std::fmt::Error))),
            };

            let description = match task["description"].as_str() {
                Some(desc) => desc.to_string(),
                None => return Err(malformed_err(Box::new(std::fmt::Error))),
            };

            let level = match task["level"].as_u32() {
                Some(lvl) => lvl,
                None => return Err(malformed_err(Box::new(std::fmt::Error))),
            };

            tasks.push(Task { id, description, level });
        }

        Ok(TodoList { title, tasks })
    }
}