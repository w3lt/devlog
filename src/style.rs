use anstyle::{AnsiColor, Style};
use clap::ValueEnum;

use crate::data::status::DevLogEntryStatus;

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
