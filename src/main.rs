use clap::Parser;

mod cli;
use cli::{Cli, Commands};
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
