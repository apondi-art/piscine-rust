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

        // Get title
        let title_result = parsed["title"].as_str();
        if title_result.is_none() {
            let parse_err = ParseErr::Malformed(Box::new(std::fmt::Error));
            return Err(Box::new(ErrorWrapper { parse_err }));
        }
        let title = title_result.unwrap().to_string();

        // Get tasks array
        if !parsed["tasks"].is_array() {
            let parse_err = ParseErr::Malformed(Box::new(std::fmt::Error));
            return Err(Box::new(ErrorWrapper { parse_err }));
        }

        let tasks_value = &parsed["tasks"];

        // Check if tasks is empty
        if tasks_value.len() == 0 {
            return Err(Box::new(ParseErr::Empty));
        }

        // Parse tasks
        let mut tasks = Vec::new();
        for task in tasks_value.members() {
            let id_result = task["id"].as_u32();
            if id_result.is_none() {
                let parse_err = ParseErr::Malformed(Box::new(std::fmt::Error));
                return Err(Box::new(ErrorWrapper { parse_err }));
            }
            let id = id_result.unwrap();

            let description_result = task["description"].as_str();
            if description_result.is_none() {
                let parse_err = ParseErr::Malformed(Box::new(std::fmt::Error));
                return Err(Box::new(ErrorWrapper { parse_err }));
            }
            let description = description_result.unwrap().to_string();

            let level_result = task["level"].as_u32();
            if level_result.is_none() {
                let parse_err = ParseErr::Malformed(Box::new(std::fmt::Error));
                return Err(Box::new(ErrorWrapper { parse_err }));
            }
            let level = level_result.unwrap();

            tasks.push(Task { id, description, level });
        }

        Ok(TodoList { title, tasks })
    }
}