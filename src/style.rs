use anstyle::{AnsiColor, Style};
use clap::ValueEnum;

use crate::data::status::DevLogEntryStatus;

pub const DATE_STYLE: Style = Style::new().bold();
pub const ENTRY_COUNT_STYLE: Style = Style::new().dimmed();
pub const TIME_STYLE: Style = Style::new().dimmed().italic();
pub const PROJECT_STYLE: Style = AnsiColor::Cyan.on_default().bold();
pub const SEPARATOR_STYLE: Style = Style::new().dimmed();
pub const ID_STYLE: Style = AnsiColor::Blue.on_default();
pub const ID_LABEL_STYLE: Style = Style::new().dimmed();
pub const ITEM_STYLE: Style = AnsiColor::Cyan.on_default().bold();

#[derive(Debug, Clone, ValueEnum)]
pub enum ColorChoice {
    Auto,
    Always,
    Never,
}

impl ColorChoice {
    pub fn into_color_choice(self) -> anstream::ColorChoice {
        match self {
            ColorChoice::Auto => anstream::ColorChoice::Auto,
            ColorChoice::Always => anstream::ColorChoice::Always,
            ColorChoice::Never => anstream::ColorChoice::Never,
        }
    }
}

pub fn status_style(status: &DevLogEntryStatus) -> Style {
    match status {
        DevLogEntryStatus::InProgress => AnsiColor::Yellow.on_default().bold(),
        DevLogEntryStatus::Done => AnsiColor::Green.on_default().bold(),
        DevLogEntryStatus::Cancelled => AnsiColor::Red.on_default().bold(),
    }
}
