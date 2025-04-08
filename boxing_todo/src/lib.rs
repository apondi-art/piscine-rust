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

        // Simple JSON parsing (very basic implementation)
        let mut title = None;
        let mut tasks = Vec::new();
        
        for line in content.lines() {
            let line = line.trim();
            
            // Parse title
            if line.starts_with("\"title\"") {
                if let Some(start) = line.find(':') {
                    let value = line[start+1..].trim().trim_matches('"').trim().to_string();
                    title = Some(value);
                }
            }
            
            // Parse tasks
            if line.starts_with("\"id\"") {
                let mut id = 0;
                let mut description = String::new();
                let mut level = 0;
                
                if let Some(start) = line.find(':') {
                    if let Some(end) = line.find(',') {
                        if let Ok(num) = line[start+1..end].trim().parse::<u32>() {
                            id = num;
                        }
                    }
                }
                
                // This is very simplified parsing - would need more robust implementation
                // for real-world use
                tasks.push(Task {
                    id,
                    description: "placeholder".to_string(), // Simplified
                    level: 0, // Simplified
                });
            }
        }

        if tasks.is_empty() {
            return Err(Box::new(err::ParseErr::Empty));
        }

        Ok(TodoList {
            title: title.unwrap_or_default(),
            tasks,
        })
    }
}