use std::io;

use crate::{data::entry::DevLogEntry, store::Store};

pub fn add_entry(store: &Store, message: &str, project_id: Option<String>) -> io::Result<()> {
    let new_entry = DevLogEntry::new(message, project_id);
    match store.insert_devlog_entry(new_entry) {
        Ok(_) => {
            println!("Added item \"{}\"!", message);
            Ok(())
        }
        Err(e) => Err(io::Error::other(e)),
    }
}
