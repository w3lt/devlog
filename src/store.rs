use std::{env, io, path::PathBuf};

use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, Row};

use crate::{
    data::{entry::DevLogEntry, project::LocalProject, status::DevLogEntryStatus},
    store::result::SetStatusResult,
};

pub mod result;

pub struct Store {
    connection: Connection,
}

const LATEST_SCHEMA_VERSION: u32 = 3;
const ENTRY_TABLE_NAME: &str = "devlog_entries";
const LOCAL_PROJECT_TABLE_NAME: &str = "devlog_local_projects";

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
                format!(
                    "
                    CREATE TABLE IF NOT EXISTS {} (
                        id          TEXT PRIMARY KEY NOT NULL,
                        created_at  TEXT NOT NULL CHECK (datetime(created_at) IS NOT NULL),
                        message     TEXT NOT NULL CHECK (length(trim(message)) > 0)
                    );

                    PRAGMA user_version = 1;
                ",
                    ENTRY_TABLE_NAME
                )
                .as_str(),
            )?;

            version = 1;
        }

        if version < 2 {
            tx.execute_batch(
                format!(
                    "
                    ALTER TABLE {}
                    ADD COLUMN status TEXT NOT NULL DEFAULT 'in_progress'
                    CHECK (status IN ('in_progress', 'done', 'cancelled'));

                    PRAGMA user_version = 2;
                ",
                    ENTRY_TABLE_NAME
                )
                .as_str(),
            )?;

            version = 2;
        }

        if version < 3 {
            tx.execute_batch(
                format!(
                    "
                    PRAGMA foreign_keys = ON;

                    CREATE TABLE IF NOT EXISTS {} (
                        id           TEXT PRIMARY KEY NOT NULL,
                        name         TEXT NOT NULL UNIQUE,
                        created_at   TEXT NOT NULL CHECK (datetime(created_at) IS NOT NULL),
                        last_updated TEXT NOT NULL CHECK (datetime(last_updated) IS NOT NULL)
                    );

                    CREATE INDEX {}_name_index
                    ON {} (name);

                    ALTER TABLE {}
                    ADD COLUMN project_name TEXT REFERENCES {}(name) ON DELETE SET NULL;

                    ALTER TABLE {}
                    ADD COLUMN last_updated TEXT CHECK (
                        last_updated IS NULL OR datetime(last_updated) IS NOT NULL
                    );

                    UPDATE {}
                    SET last_updated = created_at
                    WHERE last_updated IS NULL;

                    ALTER TABLE {}
                    ALTER last_updated SET NOT NULL;

                    PRAGMA user_version = 3;
                ",
                    LOCAL_PROJECT_TABLE_NAME,
                    LOCAL_PROJECT_TABLE_NAME,
                    LOCAL_PROJECT_TABLE_NAME,
                    ENTRY_TABLE_NAME,
                    LOCAL_PROJECT_TABLE_NAME,
                    ENTRY_TABLE_NAME,
                    ENTRY_TABLE_NAME,
                    ENTRY_TABLE_NAME
                )
                .as_str(),
            )?;

            version = 3;
        }

        debug_assert_eq!(version, LATEST_SCHEMA_VERSION);

        tx.commit()?;

        Ok(())
    }

    pub fn insert_devlog_entry(&mut self, entry: DevLogEntry) -> rusqlite::Result<()> {
        let tx = self.connection.transaction()?;

        if let Some(project_name) = &entry.project_name {
            let project = LocalProject::new(project_name);

            tx.execute(
                format!(
                    "
                INSERT INTO {} (id, name, created_at, last_updated)
                VALUES (?1, ?2, ?3, ?4)
                ON CONFLICT(name) DO NOTHING;
            ",
                    LOCAL_PROJECT_TABLE_NAME
                )
                .as_str(),
                (
                    project.id,
                    project.name,
                    project.created_at.to_rfc3339(),
                    project.last_updated.to_rfc3339(),
                ),
            )?;
        }

        tx.execute(
            format!(
                "
                INSERT INTO {} (id, created_at, message, status, last_updated, project_name) VALUES (
                    ?1, ?2, ?3, ?4, ?5, ?6
                )
            ",
                ENTRY_TABLE_NAME
            )
            .as_str(),
            (
                entry.id,
                entry.created_at.to_rfc3339(),
                entry.message,
                entry.status.to_db_value(),
                entry.last_updated.to_rfc3339(),
                entry.project_name,
            ),
        )?;

        tx.commit()?;

        Ok(())
    }

    pub fn get_entries(&self, project: Option<String>) -> rusqlite::Result<Vec<DevLogEntry>> {
        match project {
            Some(real_project_name) => {
                let mut stmt = self.connection.prepare(
                    format!(
                        "
                            SELECT * FROM {}
                            WHERE project_name = ?1
                            ORDER BY created_at ASC
                        ",
                        ENTRY_TABLE_NAME
                    )
                    .as_str(),
                )?;

                stmt.query_map([real_project_name], Self::entry_from_row)?
                    .collect()
            }
            None => {
                let mut stmt = self.connection.prepare(
                    format!(
                        "
                            SELECT * FROM {}
                            ORDER BY created_at ASC
                        ",
                        ENTRY_TABLE_NAME
                    )
                    .as_str(),
                )?;

                stmt.query_map([], Self::entry_from_row)?.collect()
            }
        }
    }

    fn entry_from_row(row: &Row<'_>) -> rusqlite::Result<DevLogEntry> {
        let id: String = row.get("id")?;
        let created_at_text: String = row.get("created_at")?;
        let message: String = row.get("message")?;
        let status_text: String = row.get("status")?;
        let last_updated_text: String = row.get("last_updated")?;
        let project_name: Option<String> = row.get("project_name")?;

        let created_at = DateTime::parse_from_rfc3339(&created_at_text)
            .map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    1,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?
            .with_timezone(&Utc);

        let last_updated = DateTime::parse_from_rfc3339(&last_updated_text)
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
            last_updated,
            project_name,
        })
    }

    pub fn set_status(
        &self,
        id: &str,
        status: &DevLogEntryStatus,
    ) -> rusqlite::Result<SetStatusResult> {
        let current_status: Option<String> = self
            .connection
            .query_row(
                format!(
                    "
                    SELECT status
                    FROM {}
                    WHERE id = ?1
                ",
                    ENTRY_TABLE_NAME
                )
                .as_str(),
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

        let current_time_str = Utc::now().to_rfc3339();

        self.connection.execute(
            format!(
                "
                UPDATE {}
                SET status = ?1,
                    last_updated = ?2
                WHERE id = ?3
                    AND status <> ?1
            ",
                ENTRY_TABLE_NAME
            )
            .as_str(),
            (status.to_db_value(), current_time_str, id),
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
