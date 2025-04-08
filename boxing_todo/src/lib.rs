mod err;

use std::error::Error;
use std::fs;

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
            .map_err(|e| Box::new(err::ReadErr { child_err: Box::new(e) }))?;

        // Parse JSON
        let parsed: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| Box::new(err::ParseErr::Malformed(Box::new(e))))?;

        // Get title
        let title = parsed["title"]
            .as_str()
            .ok_or_else(|| Box::new(err::ParseErr::Malformed(Box::new(std::fmt::Error))))?
            .to_string();

        // Get tasks array
        let tasks_value = parsed["tasks"]
            .as_array()
            .ok_or_else(|| Box::new(err::ParseErr::Malformed(Box::new(std::fmt::Error))))?;

        // Check if tasks is empty
        if tasks_value.is_empty() {
            return Err(Box::new(err::ParseErr::Empty));
        }

        // Parse tasks
        let mut tasks = Vec::new();
        for task in tasks_value {
            let id = task["id"]
                .as_u64()
                .ok_or_else(|| Box::new(err::ParseErr::Malformed(Box::new(std::fmt::Error))))? as u32;

            let description = task["description"]
                .as_str()
                .ok_or_else(|| Box::new(err::ParseErr::Malformed(Box::new(std::fmt::Error))))?
                .to_string();

            let level = task["level"]
                .as_u64()
                .ok_or_else(|| Box::new(err::ParseErr::Malformed(Box::new(std::fmt::Error))))? as u32;

            tasks.push(Task {
                id,
                description,
                level,
            });
        }

        Ok(TodoList { title, tasks })
    }
}