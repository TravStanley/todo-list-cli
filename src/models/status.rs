use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, ValueEnum, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "PascalCase")]
pub enum Status {
    ToDo,
    InProgress,
    Completed,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::ToDo => "todo",
            Status::InProgress => "inprogress",
            Status::Completed => "completed",
        }
    }
}
