mod err;

use std::error::Error;
use std::fs;
use serde::Deserialize;

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

#[derive(Deserialize)]
struct RawTodoList {
    title: String,
    tasks: Vec<RawTask>,
}

#[derive(Deserialize)]
struct RawTask {
    id: u32,
    description: String,
    level: u32,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        // Read file
        let content = fs::read_to_string(path)
            .map_err(|e| Box::new(err::ReadErr { child_err: Box::new(e) }))?;

        // Parse JSON
        let raw_todo: RawTodoList = serde_json::from_str(&content)
            .map_err(|e| Box::new(err::ParseErr::Malformed(Box::new(e))))?;

        // Check if tasks is empty
        if raw_todo.tasks.is_empty() {
            return Err(Box::new(err::ParseErr::Empty));
        }

        // Convert RawTask to Task
        let tasks = raw_todo.tasks
            .into_iter()
            .map(|t| Task {
                id: t.id,
                description: t.description,
                level: t.level,
            })
            .collect();

        Ok(TodoList {
            title: raw_todo.title,
            tasks,
        })
    }
}