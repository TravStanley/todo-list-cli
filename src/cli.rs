use clap::{Parser, Subcommand};

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
        task: String,

        #[arg(short, long, default_value = "low")]
        priority: String,

        status: String,
    },

    Delete {
        task: String,
    },
}
