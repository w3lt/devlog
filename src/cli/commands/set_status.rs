use std::io::{self, Write};

use anstyle::{AnsiColor, Style};

use crate::{
    data::status::DevLogEntryStatus,
    store::{Store, result::SetStatusResult},
    style,
};

const ID_STYLE: Style = AnsiColor::Cyan.on_default().bold();

pub fn set_status(store: &Store, id: &str, status: DevLogEntryStatus) -> io::Result<()> {
    let status_style = style::status_style(&status);
    let mut out = anstream::stdout().lock();

    match store.set_status(id, &status) {
        Ok(result) => {
            match result {
                SetStatusResult::Updated => {
                    writeln!(
                        out,
                        "Set status of item {ID_STYLE}{}{ID_STYLE:#} to be {status_style}{}{status_style:#}",
                        id, status
                    )?;
                }
                SetStatusResult::NoChange => {
                    writeln!(
                        out,
                        "Item {ID_STYLE}{}{ID_STYLE:#} is already {status_style}{}{status_style:#}",
                        id, status
                    )?;
                }
                SetStatusResult::NotFound => {
                    writeln!(out, "Item {ID_STYLE}{}{ID_STYLE:#} not found!", id)?;
                }
            }
            Ok(())
        }
        Err(e) => Err(io::Error::other(e)),
    }
}
