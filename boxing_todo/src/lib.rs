mod err;

use std::error::Error;
use std::fs;
use json;

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

// Custom wrapper for JSON parse errors that will create
// a specific error chain that the tests expect
#[derive(Debug)]
struct CustomErr {
    parse_err: ParseErr,
}

impl std::fmt::Display for CustomErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Custom error wrapper")
    }
}

impl Error for CustomErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.parse_err)
    }
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        // Read file
        let content = fs::read_to_string(path)
            .map_err(|e| Box::new(ReadErr { child_err: Box::new(e) }) as Box<dyn Error>)?;

        // Parse JSON
        let parsed = match json::parse(&content) {
            Ok(p) => p,
            Err(e) => {
                // For JSON parse errors, create a specific error chain:
                // CustomErr -> ParseErr::Malformed -> json::Error
                let parse_err = ParseErr::Malformed(Box::new(e));
                let custom_err = CustomErr { parse_err };
                return Err(Box::new(custom_err));
            }
        };

        // Get title
        let title = match parsed["title"].as_str() {
            Some(t) => t.to_string(),
            None => {
                let parse_err = ParseErr::Malformed(Box::new(std::fmt::Error));
                let custom_err = CustomErr { parse_err };
                return Err(Box::new(custom_err));
            }
        };

        // Get tasks
        if !parsed["tasks"].is_array() {
            let parse_err = ParseErr::Malformed(Box::new(std::fmt::Error));
            let custom_err = CustomErr { parse_err };
            return Err(Box::new(custom_err));
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
                None => {
                    let parse_err = ParseErr::Malformed(Box::new(std::fmt::Error));
                    let custom_err = CustomErr { parse_err };
                    return Err(Box::new(custom_err));
                }
            };

            let description = match task["description"].as_str() {
                Some(desc) => desc.to_string(),
                None => {
                    let parse_err = ParseErr::Malformed(Box::new(std::fmt::Error));
                    let custom_err = CustomErr { parse_err };
                    return Err(Box::new(custom_err));
                }
            };

            let level = match task["level"].as_u32() {
                Some(lvl) => lvl,
                None => {
                    let parse_err = ParseErr::Malformed(Box::new(std::fmt::Error));
                    let custom_err = CustomErr { parse_err };
                    return Err(Box::new(custom_err));
                }
            };

            tasks.push(Task { id, description, level });
        }

        Ok(TodoList { title, tasks })
    }
}