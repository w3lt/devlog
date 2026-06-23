use clap::{ArgAction, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "devlog",
    disable_version_flag = true,
    version = version_text(),
    about = "A tiny developer journal for the terminal"
)]
pub struct Cli {
    #[arg(
        short = 'v',
        long = "version",
        action = ArgAction::Version,
        help = "Print version information"
    )]
    pub version: bool,

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
