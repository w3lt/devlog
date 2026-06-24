use clap::{Parser, Subcommand};

use crate::data::status::DevLogEntryStatus;

#[derive(Debug, Parser)]
#[command(
    name = "devlog",
    version = version_text(),
    long_version = version_text(),
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

    /// Set status of entry
    SetStatus {
        /// Id of entry to set status
        id: String,

        /// Status to be set
        #[arg(value_enum)]
        status: DevLogEntryStatus,
    },
}

fn version_text() -> &'static str {
    concat!(
        "v",
        env!("CARGO_PKG_VERSION"),
        "\n",
        "Author: ",
        env!("CARGO_PKG_AUTHORS"),
        "\n",
        "Repository: ",
        env!("CARGO_PKG_REPOSITORY")
    )
}
