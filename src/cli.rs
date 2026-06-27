use clap::{Parser, Subcommand};

use crate::{data::status::DevLogEntryStatus, style::ColorChoice};

pub mod commands;

#[derive(Debug, Parser)]
#[command(
    name = "devlog",
    version = version_text(),
    long_version = version_text(),
    about = "A tiny developer journal for the terminal"
)]
pub struct Cli {
    #[arg(long, value_enum, global = true, default_value_t = ColorChoice::Auto)]
    pub color: ColorChoice,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Add a new journal entry.
    Add {
        /// The note you want to remember.
        message: String,

        #[arg(short, long, value_name = "PROJECT", value_parser = parse_project_value)]
        project: Option<String>,
    },

    /// List journal entries.
    List {
        #[arg(short, long, value_name = "PROJECT", value_parser = parse_project_value)]
        project: Option<String>,
    },

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

fn parse_project_value(value: &str) -> Result<String, String> {
    let value = value.trim();

    if value.is_empty() {
        Err("project cannot be empty".to_string())
    } else {
        Ok(value.to_string())
    }
}
