use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum Status {
    ToDo,
    InProgress,
    Completed,
}
