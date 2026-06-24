use clap::ValueEnum;
use std::fmt::Display;

#[derive(Debug, Clone, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum DevLogEntryStatus {
    InProgress,
    Done,
    Cancelled,
}

impl Display for DevLogEntryStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let displayed_character = match self {
            DevLogEntryStatus::InProgress => "In Progress",
            DevLogEntryStatus::Done => "Done",
            DevLogEntryStatus::Cancelled => "Cancelled",
        };
        write!(f, "{}", displayed_character)
    }
}

impl DevLogEntryStatus {
    pub fn to_db_value(&self) -> String {
        match self {
            DevLogEntryStatus::InProgress => "in_progress".to_string(),
            DevLogEntryStatus::Done => "done".to_string(),
            DevLogEntryStatus::Cancelled => "cancelled".to_string(),
        }
    }

    pub fn from_db_value(value: &str) -> Option<Self> {
        match value {
            "in_progress" => Some(DevLogEntryStatus::InProgress),
            "done" => Some(DevLogEntryStatus::Done),
            "cancelled" => Some(DevLogEntryStatus::Cancelled),
            _ => None,
        }
    }

    pub fn to_ascii(&self) -> String {
        match self {
            DevLogEntryStatus::InProgress => "[~]".to_string(),
            DevLogEntryStatus::Done => "[✓]".to_string(),
            DevLogEntryStatus::Cancelled => "[x]".to_string(),
        }
    }
}
