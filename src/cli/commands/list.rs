use std::io;

use chrono::{Local, NaiveDate};

use crate::{data::entry::DevLogEntry, store::Store};

pub fn list_entries(store: &Store) -> io::Result<()> {
    match store.get_entries() {
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
    }
}
