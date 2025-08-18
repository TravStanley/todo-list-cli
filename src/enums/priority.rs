use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, ValueEnum, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}
