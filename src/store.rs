use std::{env, io, path::PathBuf};

use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension};

use crate::{
    data::{entry::DevLogEntry, status::DevLogEntryStatus},
    store::result::SetStatusResult,
};

pub mod result;

pub struct Store {
    connection: Connection,
}

const LATEST_SCHEMA_VERSION: u32 = 2;

impl Store {
    pub fn open() -> io::Result<Self> {
        let path = devlog_db_path()?;
        let opt_c = Connection::open(path);
        match opt_c {
            Ok(c) => {
                let mut s = Self { connection: c };
                s.prepare().map_err(io::Error::other)?;
                Ok(s)
            }
            Err(e) => Err(io::Error::other(e)),
        }
    }

    fn prepare(&mut self) -> rusqlite::Result<()> {
        let tx = self.connection.transaction()?;

        let mut version: u32 = tx.query_row("PRAGMA user_version", [], |row| row.get(0))?;

        if version < 1 {
            tx.execute_batch(
                "
                    CREATE TABLE IF NOT EXISTS devlog_entries (
                        id          TEXT PRIMARY KEY NOT NULL,
                        created_at  TEXT NOT NULL CHECK (datetime(created_at) IS NOT NULL),
                        message     TEXT NOT NULL CHECK (length(trim(message)) > 0)
                    );

                    PRAGMA user_version = 1;
                ",
            )?;

            version = 1;
        }

        if version < 2 {
            tx.execute_batch(
                "
                    ALTER TABLE devlog_entries
                    ADD COLUMN status TEXT NOT NULL DEFAULT 'in_progress'
                    CHECK (status IN ('in_progress', 'done', 'cancelled'));

                    PRAGMA user_version = 2;
                ",
            )?;

            version = 2;
        }

        debug_assert_eq!(version, LATEST_SCHEMA_VERSION);

        tx.commit()?;

        Ok(())
    }

    pub fn insert_devlog_entry(&self, entry: DevLogEntry) -> rusqlite::Result<()> {
        self.connection.execute(
            "
                INSERT INTO devlog_entries VALUES (
                    ?1, ?2, ?3, ?4
                )
            ",
            (
                entry.id,
                entry.created_at.to_rfc3339(),
                entry.message,
                entry.status.to_db_value(),
            ),
        )?;

        Ok(())
    }

    pub fn get_entries(&self) -> rusqlite::Result<Vec<DevLogEntry>> {
        let mut stmt = self.connection.prepare(
            "
                SELECT * FROM devlog_entries
                ORDER BY created_at ASC
            ",
        )?;

        let entries = stmt.query_map([], |row| {
            let id: String = row.get("id")?;
            let created_at_text: String = row.get("created_at")?;
            let message: String = row.get("message")?;
            let status_text: String = row.get("status")?;

            let created_at = DateTime::parse_from_rfc3339(&created_at_text)
                .map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        1,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?
                .with_timezone(&Utc);

            let status = DevLogEntryStatus::from_db_value(&status_text).ok_or_else(|| {
                rusqlite::Error::FromSqlConversionFailure(
                    3,
                    rusqlite::types::Type::Text,
                    Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("invalid devlog entry status: {status_text}"),
                    )),
                )
            })?;

            Ok(DevLogEntry {
                id,
                created_at,
                message,
                status,
            })
        })?;

        entries.collect()
    }

    pub fn set_status(
        &self,
        id: &str,
        status: &DevLogEntryStatus,
    ) -> rusqlite::Result<SetStatusResult> {
        let current_status: Option<String> = self
            .connection
            .query_row(
                "
                SELECT status
                FROM devlog_entries
                WHERE id = ?1
            ",
                [id],
                |row| row.get("status"),
            )
            .optional()?;

        let Some(current_status) = current_status else {
            return Ok(SetStatusResult::NotFound);
        };

        let new_status = status.to_db_value();

        if current_status == new_status {
            return Ok(SetStatusResult::NoChange);
        }

        self.connection.execute(
            "
                UPDATE devlog_entries
                SET status = ?1
                WHERE id = ?2
                    AND status <> ?1
            ",
            (status.to_db_value(), id),
        )?;

        Ok(SetStatusResult::Updated)
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
