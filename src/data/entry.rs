use std::fmt::Display;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::data::status::DevLogEntryStatus;

#[derive(Debug, Clone)]
pub struct DevLogEntry {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub status: DevLogEntryStatus,
    pub last_updated: DateTime<Utc>,
    pub project_id: Option<String>,
}

impl DevLogEntry {
    pub fn new(message: &str, project_id: Option<String>) -> Self {
        Self {
            id: Uuid::now_v7().to_string(),
            created_at: Utc::now(),
            message: String::from(message),
            status: DevLogEntryStatus::InProgress,
            last_updated: Utc::now(),
            project_id,
        }
    }
}

impl Display for DevLogEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} {} {}",
            self.created_at.format("%Y-%m-%d %H:%M UTC"),
            self.status,
            self.id,
            self.message
        )
    }
}
