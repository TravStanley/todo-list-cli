mod cli;
mod enums;

use clap::Parser;
use cli::{Cli, Commands};
use enums::{Priority, Status};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add {
            task,
            priority,
            status,
        } => {
            print!("{task:?} has been added at priority {priority:?} and {status:?}!")
        }
        Commands::Delete { task } => println!("{task:?} deleted!"),
    }
}
