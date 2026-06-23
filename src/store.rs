use std::{env, io, path::PathBuf};

use chrono::{DateTime, Utc};
use rusqlite::Connection;

use crate::data::entry::DevLogEntry;

pub struct Store {
    connection: Connection,
}

impl Store {
    pub fn open() -> io::Result<Self> {
        let path = devlog_db_path()?;
        let opt_c = Connection::open(path);
        match opt_c {
            Ok(c) => {
                let s = Self { connection: c };
                s.prepare().map_err(io::Error::other)?;
                Ok(s)
            }
            Err(e) => Err(io::Error::other(e)),
        }
    }

    fn prepare(&self) -> rusqlite::Result<()> {
        self.connection.execute(
            "
                CREATE TABLE IF NOT EXISTS devlog_entries (
                    id          TEXT PRIMARY KEY NOT NULL,
                    created_at  TEXT NOT NULL CHECK (datetime(created_at) IS NOT NULL),
                    message     TEXT NOT NULL CHECK (length(trim(message)) > 0)
                )
            ",
            [],
        )?;

        Ok(())
    }

    pub fn insert_devlog_entry(&self, entry: DevLogEntry) -> rusqlite::Result<()> {
        self.connection.execute(
            "
                INSERT INTO devlog_entries VALUES (
                    ?1, ?2, ?3
                )
            ",
            (entry.id, entry.created_at.to_rfc3339(), entry.message),
        )?;

        Ok(())
    }

    pub fn get_entries(&self) -> rusqlite::Result<Vec<DevLogEntry>> {
        let mut stmt = self.connection.prepare(
            "
                SELECT * FROM devlog_entries
            ",
        )?;

        let entries = stmt.query_map([], |row| {
            let id: String = row.get("id")?;
            let created_at_text: String = row.get("created_at")?;
            let message: String = row.get("message")?;

            let created_at = DateTime::parse_from_rfc3339(&created_at_text)
                .map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        1,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?
                .with_timezone(&Utc);

            Ok(DevLogEntry {
                id,
                created_at,
                message,
            })
        })?;

        entries.collect()
    }
}

fn devlog_db_path() -> io::Result<PathBuf> {
    let home = env::var_os("HOME")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "HOME is not set"))?;

    let mut path = PathBuf::from(home);
    path.push(".devlog");

    std::fs::create_dir_all(&path)?;

    path.push("entries.sqlite");

    Ok(path)
}
