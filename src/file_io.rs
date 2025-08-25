use chrono::NaiveDate;
use std::fs;

use crate::enums::{Priority, Status};

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

pub fn update_todo_object(
    task_name: &str,
    description: Option<String>,
    priority: Option<Priority>,
    status: Option<Status>,
    due_date: Option<NaiveDate>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut task_list = get_todo_list()?;

    if let Some(pos) = task_list
        .tasks
        .iter()
        .position(|task| task.name == task_name)
    {
        let task = &mut task_list.tasks[pos];

        if let Some(desc) = description {
            task.description = Some(desc);
        }
        if let Some(p) = priority {
            task.priority = p;
        }
        if let Some(s) = status {
            task.status = s;
        }
        if let Some(date) = due_date {
            task.due_date = Some(date);
        }

        fs::write(TASK_TOML, toml::to_string(&task_list)?)?;
    } else {
        return Err(format!("Task '{}' not found", task_name).into());
    }

    Ok(())
}

pub fn print_tasks_to_cli() -> Result<(), Box<dyn std::error::Error>> {
    let task_list = get_todo_list()?;

    task_list
        .tasks
        .iter()
        .for_each(|task_obj| println!("{:?}", task_obj));

    Ok(())
}
