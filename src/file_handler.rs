use serde::{Serialize, Deserialize};
use std::fmt;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use serde_json;

const FILE_PATH: &str = "todo.json"; 

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct TodoList {
    pub tasks: Vec<Task>,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Title: {}, Description: {}, Completed: {}",
            self.title,
            self.description,
            self.completed
        )
    }
}

impl Task {
    pub fn new(title: String, desc: String, completed: bool) -> Result<Task, &'static str> {
        if title.is_empty() || desc.is_empty() {
            Err("Title or description cannot be empty")
        } else {
            Ok(Task {
                title,
                description: desc,
                completed,
            })
        }
    }
}

impl TodoList {
    pub fn insert(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn print_data(&self) {
        for task in &self.tasks {
            println!("{}", task);
        }
    }

    pub fn remove_task(&mut self, title: &str) {
        self.tasks.retain(|task| task.title != title);
    }
}

pub fn load_todo_list() -> io::Result<TodoList> {
    if Path::new(FILE_PATH).exists() {
        let mut file = File::open(FILE_PATH)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        if data.trim().is_empty() {
            Ok(TodoList::default())
        } else {
            serde_json::from_str(&data)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        }
    } else {
        Ok(TodoList::default())
    }
}

pub fn save_todo_list(todo_list: &TodoList) -> io::Result<()> {
    let data = serde_json::to_string_pretty(todo_list)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let mut file = File::create(FILE_PATH)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
