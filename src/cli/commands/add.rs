use std::io::{self, Write};

use crate::{data::entry::DevLogEntry, store::Store, style::ITEM_STYLE};

pub fn add_entry(store: &mut Store, message: &str, project_name: Option<String>) -> io::Result<()> {
    let new_entry = DevLogEntry::new(message, project_name);
    let mut out = anstream::stdout().lock();
    match store.insert_devlog_entry(new_entry) {
        Ok(_) => {
            writeln!(out, "Added item {ITEM_STYLE}\"{}\"{ITEM_STYLE:#}!", message)?;
            Ok(())
        }
        Err(e) => Err(io::Error::other(e)),
    }
}
