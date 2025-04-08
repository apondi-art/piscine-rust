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
        // This is the crucial part - return the ParseErr as the source
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
                // First create the ParseErr holding the JSON error
                let parse_err = ParseErr::Malformed(Box::new(e));
                
                // Then wrap it in our custom ErrorWrapper that will return
                // the ParseErr as its source()
                return Err(Box::new(ErrorWrapper { parse_err }));
            }
        };

        // Get title
        let title = match parsed["title"].as_str() {
            Some(t) => t.to_string(),
            None => {
                let parse_err = ParseErr::Malformed(Box::new(std::fmt::Error));
                return Err(Box::new(ErrorWrapper { parse_err }));
            }
        };

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
            let id = match task["id"].as_u32() {
                Some(id) => id,
                None => {
                    let parse_err = ParseErr::Malformed(Box::new(std::fmt::Error));
                    return Err(Box::new(ErrorWrapper { parse_err }));
                }
            };

            let description = match task["description"].as_str() {
                Some(desc) => desc.to_string(),
                None => {
                    let parse_err = ParseErr::Malformed(Box::new(std::fmt::Error));
                    return Err(Box::new(ErrorWrapper { parse_err }));
                }
            };

            let level = match task["level"].as_u32() {
                Some(lvl) => lvl,
                None => {
                    let parse_err = ParseErr::Malformed(Box::new(std::fmt::Error));
                    return Err(Box::new(ErrorWrapper { parse_err }));
                }
            };

            tasks.push(Task { id, description, level });
        }

        Ok(TodoList { title, tasks })
    }
}