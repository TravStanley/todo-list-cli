mod cli;
mod db;
mod models;
mod repository;
mod service;

use clap::Parser;
use cli::{Cli, Commands};
use db::init_db;
use models::Task;
use repository::SqliteTaskRepository;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let pool = init_db().await;
    let repo = SqliteTaskRepository::new(pool);

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
            match service::add_task(&repo, task).await {
                Ok(_) => println!(
                    "{:?} has been added at priority {:?} and status {:?}!\n",
                    save_for_print.name, save_for_print.priority, save_for_print.status
                ),
                Err(e) => println!("{e:?}"),
            }
        }
        Commands::Delete { task_name } => match service::delete_task(&repo, &task_name).await {
            Ok(_) => println!("{task_name:?} deleted!\n"),
            Err(e) => println!("{e:?}"),
        },
        Commands::Status { task_name, status } => {
            match service::update_task_status(&repo, &task_name, status).await {
                Ok(_) => println!("{task_name:?} status updated to {status:?}!\n"),
                Err(e) => println!("{e:?}"),
            }
        }
        Commands::Showall {} => match service::show_all(&repo).await {
            Ok(_) => println!("\n"),
            Err(e) => println!("{e:?}"),
        },
    }
}
