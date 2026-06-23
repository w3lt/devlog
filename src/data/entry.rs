use std::fmt::Display;

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DevLogEntry {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
}

impl DevLogEntry {
    pub fn new(message: &str) -> Self {
        Self {
            id: Uuid::now_v7().to_string(),
            created_at: Utc::now(),
            message: String::from(message),
        }
    }
}

impl Display for DevLogEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let short_id = self.id.get(..8).unwrap_or(&self.id);

        write!(
            f,
            "[{}] {} {}",
            self.created_at.format("%Y-%m-%d %H:%M UTC"),
            short_id,
            self.message
        )
    }
}
