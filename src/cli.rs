use chrono::NaiveDate;
use clap::{Parser, Subcommand};

use crate::enums::{Priority, Status};

#[derive(Parser, Debug)]
#[command(name = "todo", about = "A simple to-do list CLI")]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Add a new task to the to-do list")]
    Add {
        name: String,

        description: Option<String>,

        #[arg(short, long, default_value = "low")]
        priority: Priority,

        #[arg(short, long, default_value = "to-do")]
        status: Status,

        #[arg(short, long, value_parser = parse_date)]
        due_date: Option<NaiveDate>,
    },

    Delete {
        task_name: String,
    },

    Status {
        task_name: String,

        status: Status,
    },
}

fn parse_date(date: &str) -> Result<NaiveDate, String> {
    let normalized = date.replace('/', "-").replace('.', "-").replace(',', "-");

    NaiveDate::parse_from_str(&normalized, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date '{}': {}", date, e))
}
