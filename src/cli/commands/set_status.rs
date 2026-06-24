use std::io;

use crate::{
    data::status::DevLogEntryStatus,
    store::{Store, result::SetStatusResult},
};

pub fn set_status(store: &Store, id: &str, status: DevLogEntryStatus) -> io::Result<()> {
    match store.set_status(id, &status) {
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
    }
}
