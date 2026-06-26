---
title: Data storage
description: Where devlog stores data, the SQLite schema, and how migrations work.
---

Everything lives in one SQLite database, created on first run:

```text
~/.devlog/entries.sqlite
```

## Schema

The schema is two tables: entries, and the projects they can be tagged with.

```sql
CREATE TABLE IF NOT EXISTS devlog_entries (
    id            TEXT PRIMARY KEY NOT NULL,
    created_at    TEXT NOT NULL CHECK (datetime(created_at) IS NOT NULL),
    message       TEXT NOT NULL CHECK (length(trim(message)) > 0),
    status        TEXT NOT NULL DEFAULT 'in_progress'
                  CHECK (status IN ('in_progress', 'done', 'cancelled')),
    last_updated  TEXT NOT NULL DEFAULT '1970-01-01T00:00:00Z'
                  CHECK (datetime(last_updated) IS NOT NULL),
    project_name  TEXT REFERENCES devlog_local_projects(name) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS devlog_local_projects (
    id            TEXT PRIMARY KEY NOT NULL,
    name          TEXT NOT NULL UNIQUE CHECK (length(trim(name)) > 0),
    created_at    TEXT NOT NULL CHECK (datetime(created_at) IS NOT NULL),
    last_updated  TEXT NOT NULL CHECK (datetime(last_updated) IS NOT NULL)
);
```

## `devlog_entries`

| Column | Type | Notes |
|---|---|---|
| `id` | `TEXT` | UUID v7, time-ordered and generated per entry. |
| `created_at` | `TEXT` | UTC timestamp in RFC 3339; the `CHECK` rejects anything SQLite cannot parse as a datetime. |
| `message` | `TEXT` | The note. Must be non-empty after trimming whitespace. |
| `status` | `TEXT` | Lifecycle state: one of `in_progress`, `done`, or `cancelled`. |
| `last_updated` | `TEXT` | UTC timestamp of the last status change; equals `created_at` for fresh entries. The `1970` default exists only to backfill rows created before this column. |
| `project_name` | `TEXT` | Optional project tag, a foreign key into `devlog_local_projects(name)`, or `NULL` for untagged entries. |

## `devlog_local_projects`

| Column | Type | Notes |
|---|---|---|
| `id` | `TEXT` | UUID v7, generated per project. |
| `name` | `TEXT` | Unique project name. Must be non-empty after trimming whitespace. |
| `created_at` | `TEXT` | UTC timestamp in RFC 3339. |
| `last_updated` | `TEXT` | UTC timestamp in RFC 3339. |

`devlog_local_projects` holds one row per project, created on demand the first
time you tag an entry with `--project`.

## Versioning and migrations

The schema is versioned with SQLite's `PRAGMA user_version`, currently `3`, so
existing journals are migrated in place when you upgrade `devlog`.

The `status` column was added through a migration. Later migrations added the
projects table, `project_name`, and `last_updated`.

## Inspecting your database

Because it is plain SQLite, you can inspect or back up your journal with
ordinary tools:

```bash
sqlite3 ~/.devlog/entries.sqlite "SELECT created_at, status, project_name, message FROM devlog_entries;"
```
