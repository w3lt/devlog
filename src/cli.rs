use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "devlog",
    version,
    about = "A tiny developer journal for the terminal"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Add a new journal entry.
    Add {
        /// The note you want to remember.
        message: String,
    },

    /// List journal entries.
    List,
}
