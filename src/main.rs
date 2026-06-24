use std::io;

use crate::{
    cli::{Cli, Command},
    data::entry::DevLogEntry,
    store::{Store, result::SetStatusResult},
};
use chrono::{Local, NaiveDate};
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
                let mut groups: Vec<(NaiveDate, Vec<DevLogEntry>)> = Vec::new();

                for entry in entries {
                    let local_time = entry.created_at.with_timezone(&Local);
                    let day = local_time.date_naive();

                    match groups.last_mut() {
                        Some((current_day, day_entries)) if *current_day == day => {
                            day_entries.push(entry);
                        }
                        _ => {
                            groups.push((day, vec![entry]));
                        }
                    }
                }

                for (day, day_entries) in groups.iter().rev() {
                    let entry_count = day_entries.len();
                    println!(
                        "{} · {} {}",
                        day.format("%A, %Y-%m-%d"),
                        entry_count,
                        if entry_count == 1 { "entry" } else { "entries" }
                    );
                    println!();

                    for entry in day_entries {
                        let local_time = entry.created_at.with_timezone(&Local);

                        println!(
                            "  {} {}  {}",
                            entry.status.to_ascii(),
                            local_time.format("%H:%M"),
                            entry.message
                        );

                        println!("      id: {}", entry.id);
                        println!();
                    }
                }

                Ok(())
            }
            Err(e) => Err(io::Error::other(e)),
        },
        Command::SetStatus { id, status } => match store.set_status(&id, &status) {
            Ok(result) => {
                match result {
                    SetStatusResult::Updated => {
                        println!("Set status of item {} to be {}", id, status);
                    }
                    SetStatusResult::NoChange => {
                        println!("Item {} is already {}", id, status);
                    }
                    SetStatusResult::NotFound => {
                        println!("Item {} not found!", id);
                    }
                }
                Ok(())
            }
            Err(e) => Err(io::Error::other(e)),
        },
    }
}
