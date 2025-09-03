use crate::models::{Status, Task};
use crate::repository::TaskRepository;
use sqlx::{Result, Row};

pub async fn add_task<R: TaskRepository>(repo: &R, task: Task) -> Result<()> {
    repo.add(task).await
}

pub async fn delete_task<R: TaskRepository>(repo: &R, name: &str) -> Result<()> {
    repo.delete(name).await
}

pub async fn update_task_status<R: TaskRepository>(
    repo: &R,
    name: &str,
    status: Status,
) -> Result<()> {
    repo.update_status(name, status).await
}

pub async fn show_all<R: TaskRepository>(repo: &R) -> Result<()> {
    let row = repo.all().await?;

    let tasks: Vec<Task> = row
        .iter()
        .map(|row| {
            Task::new(
                row.get("name"),
                row.get("description"),
                row.get("priority"),
                row.get("status"),
                row.get("due_date"),
            )
        })
        .collect();
    for task in tasks {
        println!("{:?}", task);
    }
    Ok(())
}
