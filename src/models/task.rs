use crate::models::{Priority, Status};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub name: String,
    pub description: Option<String>,
    pub priority: Priority,
    pub status: Status,
    pub due_date: Option<NaiveDate>,
}

impl Task {
    pub fn new(
        name: String,
        description: Option<String>,
        priority: Priority,
        status: Status,
        due_date: Option<NaiveDate>,
    ) -> Self {
        Self {
            name,
            description,
            priority,
            status,
            due_date,
        }
    }
}
