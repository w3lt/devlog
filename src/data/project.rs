use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct LocalProject {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl LocalProject {
    pub fn new(name: &str) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7().to_string(),
            name: String::from(name),
            created_at: now,
            last_updated: now,
        }
    }
}
