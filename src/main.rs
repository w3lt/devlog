use std::io;

use crate::{
    cli::{Cli, Command, commands},
    store::Store,
};
use clap::Parser;

mod cli;
mod data;
mod store;
mod style;

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let mut store = Store::open()?;

    cli.color.into_color_choice().write_global();

    match cli.command {
        Command::Add { message, project } => {
            commands::add::add_entry(&mut store, &message, project)
        }
        Command::List { project } => commands::list::list_entries(&store, project),
        Command::SetStatus { id, status } => commands::set_status::set_status(&store, &id, status),
    }
}
