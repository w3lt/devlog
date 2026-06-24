use std::io;

use crate::{
    cli::{Cli, Command, commands},
    store::Store,
};
use clap::Parser;

mod cli;
mod data;
mod store;

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let store = Store::open()?;

    match cli.command {
        Command::Add { message } => commands::add::add_entry(&store, &message),
        Command::List => commands::list::list_entries(&store),
        Command::SetStatus { id, status } => commands::set_status::set_status(&store, &id, status),
    }
}
