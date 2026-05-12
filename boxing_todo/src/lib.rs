pub mod err;
pub use err::{ParseErr, ReadErr};

use std::error::Error;

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
        let content = std::fs::read_to_string(path).map_err(|e| {
            Box::new(ReadErr {
                child_err: Box::new(e),
            }) as Box<dyn Error>
        })?;

        let parsed = json::parse(&content).map_err(|e| {
            Box::new(ParseErr::Malformed(Box::new(e))) as Box<dyn Error>
        })?;

        let title = parsed["title"].as_str().unwrap_or("").to_string();
        let tasks_json = &parsed["tasks"];

        if tasks_json.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        let mut tasks = Vec::new();

        for task in tasks_json.members() {
            tasks.push(Task {
                id: task["id"].as_u32().unwrap(),
                description: task["description"].as_str().unwrap().to_string(),
                level: task["level"].as_u32().unwrap(),
            });
        }

        Ok(TodoList { title, tasks })
    }
}