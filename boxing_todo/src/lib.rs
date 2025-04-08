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
            .map_err(|e| Box::new(ReadErr { child_err: Box::new(e) }))?;

        // Parse JSON
        let parsed = json::parse(&content)
            .map_err(|e| Box::new(ParseErr::Malformed(Box::new(e))))?;

        // Get title
        let title = if let Some(t) = parsed["title"].as_str() {
            t.to_string()
        } else {
            return Err(Box::new(ParseErr::Malformed(Box::new(std::fmt::Error))));
        };

        // Get tasks
        let tasks_value = &parsed["tasks"];
        if !tasks_value.is_array() {
            return Err(Box::new(ParseErr::Malformed(Box::new(std::fmt::Error))));
        }

        // Check if tasks is empty
        if tasks_value.len() == 0 {
            return Err(Box::new(ParseErr::Empty));
        }

        // Parse tasks
        let mut tasks = Vec::new();
        for task in tasks_value.members() {
            let id = if let Some(id) = task["id"].as_u32() {
                id
            } else {
                return Err(Box::new(ParseErr::Malformed(Box::new(std::fmt::Error))));
            };

            let description = if let Some(desc) = task["description"].as_str() {
                desc.to_string()
            } else {
                return Err(Box::new(ParseErr::Malformed(Box::new(std::fmt::Error))));
            };

            let level = if let Some(lvl) = task["level"].as_u32() {
                lvl
            } else {
                return Err(Box::new(ParseErr::Malformed(Box::new(std::fmt::Error))));
            };

            tasks.push(Task { id, description, level });
        }

        Ok(TodoList { title, tasks })
    }
}