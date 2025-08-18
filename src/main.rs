mod cli;
mod enums;
mod file_io;
mod structs;

use clap::Parser;
use cli::{Cli, Commands};
use structs::Task;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add {
            name,
            description,
            priority,
            status,
            due_date,
        } => {
            let task = Task::new(name, description, priority, status, due_date);

            let save_for_print = task.clone();
            match file_io::add_todo_object(task) {
                Ok(_) => println!(
                    "{:?} has been added at priority {:?} and status {:?}!\n",
                    save_for_print.name, save_for_print.priority, save_for_print.status
                ),
                Err(e) => println!("{e:?}"),
            }
        }
        Commands::Delete { task } => println!("{task:?} deleted!\n"),
    }
}
