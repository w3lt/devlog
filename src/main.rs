use std::io;

use crate::{
    cli::{Cli, Command},
    data::entry::DevLogEntry,
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
        Command::Add { message } => {
            let new_entry = DevLogEntry::new(&message);
            match store.insert_devlog_entry(new_entry) {
                Ok(_) => {
                    println!("Added item \"{}\"!", message);
                    Ok(())
                }
                Err(e) => Err(io::Error::other(e)),
            }
        }
        Command::List => match store.get_entries() {
            Ok(entries) => {
                for e in entries {
                    println!("{}", e);
                }
                Ok(())
            }
            Err(e) => Err(io::Error::other(e)),
        },
    }
}
