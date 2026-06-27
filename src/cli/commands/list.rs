use std::io::{self, Write};

use anstyle::{AnsiColor, Style};
use chrono::{Local, NaiveDate};

use crate::{
    data::{entry::DevLogEntry, status::DevLogEntryStatus},
    store::Store,
};

const DATE_STYLE: Style = Style::new().bold();
const ENTRY_COUNT_STYLE: Style = Style::new().dimmed();
const TIME_STYLE: Style = Style::new().dimmed().italic();
const PROJECT_STYLE: Style = AnsiColor::Cyan.on_default().bold();
const SEPARATOR_STYLE: Style = Style::new().dimmed();
const ID_STYLE: Style = Style::new().dimmed();

pub fn list_entries(store: &Store, project: Option<String>) -> io::Result<()> {
    match store.get_entries(project.as_deref()) {
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

            let mut out = anstream::stdout().lock();

            for (day, day_entries) in groups.iter().rev() {
                print_day(&mut out, day, day_entries)?;
            }

            Ok(())
        }
        Err(e) => Err(io::Error::other(e)),
    }
}

fn print_day(
    out: &mut impl Write,
    day: &NaiveDate,
    day_entries: &Vec<DevLogEntry>,
) -> io::Result<()> {
    let entry_count = day_entries.len();
    let entry_count_label = if entry_count == 1 { "entry" } else { "entries" };

    writeln!(
        out,
        "{DATE_STYLE}{}{DATE_STYLE:#} {ENTRY_COUNT_STYLE}· {} {}{ENTRY_COUNT_STYLE:#}",
        day.format("%A, %Y-%m-%d"),
        entry_count,
        entry_count_label
    )?;

    writeln!(out)?;

    for entry in day_entries {
        let local_time = entry.created_at.with_timezone(&Local);
        let status_style = status_style(&entry.status);
        let message_style = message_style(&entry.status);

        write!(
            out,
            "  {status_style}{}{status_style:#} {TIME_STYLE}{}{TIME_STYLE:#}  {message_style}{}{message_style:#}",
            entry.status.to_ascii(),
            local_time.format("%H:%M"),
            entry.message,
        )?;

        if let Some(project_name) = &entry.project_name {
            write!(
                out,
                " {SEPARATOR_STYLE}·{SEPARATOR_STYLE:#} {PROJECT_STYLE}{}{PROJECT_STYLE:#}",
                project_name,
            )?;
        }

        writeln!(out)?;

        writeln!(out, "      {ID_STYLE}id: {}{ID_STYLE:#}", entry.id,)?;

        writeln!(out)?;
    }

    Ok(())
}

fn status_style(status: &DevLogEntryStatus) -> Style {
    match status {
        DevLogEntryStatus::InProgress => AnsiColor::Yellow.on_default().bold(),
        DevLogEntryStatus::Done => AnsiColor::Green.on_default().bold(),
        DevLogEntryStatus::Cancelled => AnsiColor::Red.on_default().bold(),
    }
}

fn message_style(status: &DevLogEntryStatus) -> Style {
    match status {
        DevLogEntryStatus::Cancelled => Style::new().strikethrough(),
        _ => Style::new(),
    }
}
