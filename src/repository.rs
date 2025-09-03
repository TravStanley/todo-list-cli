use crate::models::{Status, Task};
use async_trait::async_trait;
use sqlx::{Result, SqlitePool, sqlite::SqliteRow};

#[async_trait]
pub trait TaskRepository {
    async fn add(&self, task: Task) -> Result<()>;
    async fn delete(&self, name: &str) -> Result<()>;
    async fn update_status(&self, name: &str, status: Status) -> Result<()>;
    async fn all(&self) -> Result<Vec<SqliteRow>>;
}

pub struct SqliteTaskRepository {
    pool: SqlitePool,
}

impl SqliteTaskRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskRepository for SqliteTaskRepository {
    async fn add(&self, task: Task) -> Result<()> {
        sqlx::query(
            "
            INSERT INTO tasks (name, description, priority, status, due_date)
            VALUES (?, ?, ?, ?, ?)
            ",
        )
        .bind(&task.name)
        .bind(&task.description)
        .bind(task.priority.as_str())
        .bind(task.status.as_str())
        .bind(task.due_date.map(|d| d.to_string()))
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, name: &str) -> Result<()> {
        sqlx::query("DELETE FROM tasks WHERE name = ?")
            .bind(name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn update_status(&self, name: &str, status: Status) -> Result<()> {
        sqlx::query("UPDATE tasks SET status = ? WHERE name = ?")
            .bind(status.as_str())
            .bind(name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn all(&self) -> Result<Vec<SqliteRow>> {
        let rows = sqlx::query("SELECT * FROM tasks")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows)
    }
}
