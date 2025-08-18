use std::fs;

use super::structs::{Task, TaskList};

const TASK_TOML: &str = "todo-data.toml";

pub fn add_todo_object(task: Task) -> Result<(), Box<dyn std::error::Error>> {
    // let serialized = toml::to_string(task)?;

    let mut task_list: TaskList = match fs::read_to_string(TASK_TOML) {
        Ok(contents) if !contents.trim().is_empty() => toml::from_str(&contents)?,
        _ => TaskList { tasks: Vec::new() },
    };

    task_list.tasks.push(task);
    fs::write(TASK_TOML, toml::to_string(&task_list)?)?;
    Ok(())
}
