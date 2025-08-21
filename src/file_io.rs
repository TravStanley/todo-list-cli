use std::fs;

use super::structs::{Task, TaskList};

const TASK_TOML: &str = "todo-data.toml";

fn get_todo_list() -> Result<TaskList, toml::de::Error> {
    let task_string = fs::read_to_string(TASK_TOML);
    match task_string {
        Ok(contents) if !contents.trim().is_empty() => toml::from_str(&contents),
        _ => Ok(TaskList { tasks: Vec::new() }),
    }
}

pub fn add_todo_object(task: Task) -> Result<(), Box<dyn std::error::Error>> {
    let mut task_list = get_todo_list()?;
    task_list.tasks.push(task);
    fs::write(TASK_TOML, toml::to_string(&task_list)?)?;
    Ok(())
}

pub fn delete_todo_object(task_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut task_list = get_todo_list()?;

    if let Some(pos) = task_list
        .tasks
        .iter()
        .position(|task| task.name == task_name)
    {
        task_list.tasks.remove(pos);
        fs::write(TASK_TOML, toml::to_string(&task_list)?)?;
    } else {
        return Err(format!("Task '{}' not found", task_name).into());
    }

    return Ok(());
}
