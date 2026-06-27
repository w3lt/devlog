use crate::{
    data::status::DevLogEntryStatus,
    store::{Store, result::SetStatusResult},
    style::{self, ID_STYLE},
};
use std::io::{self, Write};

pub fn set_status(store: &Store, id: &str, status: DevLogEntryStatus) -> io::Result<()> {
    let status_style = style::status_style(&status);
    let mut out = anstream::stdout().lock();
    let bold_id_style = ID_STYLE.bold();

    match store.set_status(id, &status) {
        Ok(result) => {
            match result {
                SetStatusResult::Updated => {
                    writeln!(
                        out,
                        "Set status of item {bold_id_style}{}{bold_id_style:#} to be {status_style}{}{status_style:#}",
                        id, status
                    )?;
                }
                SetStatusResult::NoChange => {
                    writeln!(
                        out,
                        "Item {bold_id_style}{}{bold_id_style:#} is already {status_style}{}{status_style:#}",
                        id, status
                    )?;
                }
                SetStatusResult::NotFound => {
                    writeln!(
                        out,
                        "Item {bold_id_style}{}{bold_id_style:#} not found!",
                        id
                    )?;
                }
            }
            Ok(())
        }
        Err(e) => Err(io::Error::other(e)),
    }
}
