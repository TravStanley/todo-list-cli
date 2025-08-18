use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, ValueEnum, Serialize, Deserialize)]
pub enum Status {
    ToDo,
    InProgress,
    Completed,
}
