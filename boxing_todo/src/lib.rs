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

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        // Read file
        let content = fs::read_to_string(path)
            .map_err(|e| Box::new(ReadErr { child_err: Box::new(e) }) as Box<dyn Error>)?;

        // Parse JSON with nested error structure
        let parsed = match json::parse(&content) {
            Ok(p) => p,
            Err(e) => {
                // Create inner ParseErr with json::Error
                let inner_err = ParseErr::Malformed(Box::new(e));
                // Create outer ParseErr with inner_err as source
                let outer_err = ParseErr::Malformed(Box::new(inner_err));
                return Err(Box::new(outer_err));
            }
        };

        // Get title
        let title = parsed["title"].as_str()
            .ok_or_else(|| {
                Box::new(ParseErr::Malformed(Box::new(std::fmt::Error))) as Box<dyn Error>
            })?
            .to_string();

        // Get tasks
        if !parsed["tasks"].is_array() {
            return Err(Box::new(ParseErr::Malformed(Box::new(std::fmt::Error))));
        }

        let tasks_value = &parsed["tasks"];
        
        // Check if tasks is empty
        if tasks_value.len() == 0 {
            return Err(Box::new(ParseErr::Empty));
        }

        // Parse tasks
        let mut tasks = Vec::new();
        for task in tasks_value.members() {
            let id = task["id"].as_u32()
                .ok_or_else(|| {
                    Box::new(ParseErr::Malformed(Box::new(std::fmt::Error))) as Box<dyn Error>
                })?;

            let description = task["description"].as_str()
                .ok_or_else(|| {
                    Box::new(ParseErr::Malformed(Box::new(std::fmt::Error))) as Box<dyn Error>
                })?
                .to_string();

            let level = task["level"].as_u32()
                .ok_or_else(|| {
                    Box::new(ParseErr::Malformed(Box::new(std::fmt::Error))) as Box<dyn Error>
                })?;

            tasks.push(Task { id, description, level });
        }

        Ok(TodoList { title, tasks })
    }
}