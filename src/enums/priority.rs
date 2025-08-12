use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum Priority {
    Low,
    Medium,
    High,
}
