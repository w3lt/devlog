<div align="center">

<img src="assets/hero.svg" width="820" alt="devlog — a tiny developer journal that lives in your terminal" />

<p>
  <img src="https://img.shields.io/badge/version-0.9.0-3b82f6?style=flat-square" alt="version" />
  <img src="https://img.shields.io/badge/license-Apache--2.0-22c55e?style=flat-square" alt="license" />
  <img src="https://img.shields.io/badge/Rust-2024_edition-f59e0b?style=flat-square&logo=rust&logoColor=white" alt="Rust 2024 edition" />
  <img src="https://img.shields.io/badge/storage-SQLite-003B57?style=flat-square&logo=sqlite&logoColor=white" alt="SQLite storage" />
  <img src="https://img.shields.io/badge/platform-macOS_·_Linux-64748b?style=flat-square" alt="platform" />
</p>

<em>Capture what you did, when you did it — without ever leaving the shell.</em>

</div>

<img src="assets/divider.svg" width="100%" alt="" />

## Contents

- [Why devlog?](#why-devlog)
- [Features](#features)
- [Install](#install)
- [Usage](#usage)
- [How your data is stored](#how-your-data-is-stored)
- [Project layout](#project-layout)
- [Building from source](#building-from-source)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)

## Why devlog?

Standups, retros, performance reviews, and "wait, what did I actually ship last
Tuesday?" all want the same thing: a timestamped trail of your work. `devlog`
gives you that with two commands and zero ceremony. It is **local-first**, has
**no network calls**, and keeps everything in a single SQLite file you own.

```console
$ devlog add "Refactored the store layer"
Added item "Refactored the store layer"!
```

That is the whole ritual. Type the note, hit enter, get back to work.

## Features

| | |
|---|---|
| 📓 **Frictionless capture** | One short command — `devlog add "…"` — and the thought is saved. |
| 📜 **Full history at a glance** | `devlog list` groups every entry by day, most recent day first. |
| ✅ **Track each entry's state** | Move items between `in_progress`, `done`, and `cancelled` with `devlog set-status`. |
| 🎨 **Color where it helps** | `list`, `add`, and `set-status` color status markers, project tags, and ids; the global `--color` flag (`auto`/`always`/`never`) keeps output script- and pipe-friendly. |
| 🏷️ **Group by project** | Tag an entry with `--project` and filter your history down to one project at a time. |
| 🗃️ **Local-first SQLite** | Your journal lives in `~/.devlog/entries.sqlite`. No cloud, no account. |
| 🆔 **Time-ordered UUID v7** | IDs encode creation time, so entries sort naturally by when they happened. |
| 🕓 **Honest timestamps** | Stored in UTC (RFC 3339), shown in your local time when you `list`. |
| 🦀 **One small binary** | SQLite is bundled at build time — nothing to install alongside it. |
| 🔒 **Quiet by design** | No telemetry, no background process, no network. |

## Install

### From crates.io

```bash
cargo install d3vlog --locked
```

> The crate is published as **`d3vlog`**, but it installs a binary named **`devlog`**.

### From source

```bash
git clone https://github.com/w3lt/devlog
cd devlog
cargo install --path .
```

**Requirements:** a Rust toolchain new enough for the 2024 edition (Rust **1.85+**)
and a C compiler — [`rusqlite`](https://crates.io/crates/rusqlite) compiles a
bundled copy of SQLite, so you do **not** need SQLite installed on your system.

## Usage

```console
$ devlog --help
A tiny developer journal for the terminal

Usage: devlog [OPTIONS] <COMMAND>

Commands:
  add         Add a new journal entry
  list        List journal entries
  set-status  Set status of entry
  help        Print this message or the help of the given subcommand(s)

Options:
      --color <COLOR>  [default: auto] [possible values: auto, always, never]
  -h, --help           Print help
  -V, --version        Print version
```

| Command | What it does | Example |
|---|---|---|
| `devlog add <message> [-p <project>]` | Append a new entry, stamped with the current UTC time. Optionally tag it with a project. | `devlog add "Cut the v0.2 release" -p devlog` |
| `devlog list [-p <project>]` | Print every entry, grouped by day (newest day first). Pass `-p` to show one project only. | `devlog list -p devlog` |
| `devlog set-status <id> <status>` | Set an entry's status to `in_progress`, `done`, or `cancelled`. | `devlog set-status <id> done` |
| `devlog --color <when>` | Global flag controlling colored output: `auto` (default), `always`, or `never`. | `devlog list --color never` |
| `devlog --version` | Show the installed version. | `devlog --version` |
| `devlog --help` | Show help (works on subcommands too). | `devlog add --help` |

### A typical session

```console
$ devlog add "Ship the new auth flow"
Added item "Ship the new auth flow"!

$ devlog add "Fix flaky test in store.rs" --project devlog
Added item "Fix flaky test in store.rs"!

$ devlog list
Wednesday, 2026-06-24 · 2 entries

  [~] 09:14  Ship the new auth flow
      id: 019efa5e-5f23-70b3-b4d3-f5f1643764a3

  [~] 11:02  Fix flaky test in store.rs · devlog
      id: 019efa5e-5f2a-7eb0-9ed7-9980495715a5

$ devlog set-status 019efa5e-5f23-70b3-b4d3-f5f1643764a3 done
Set status of item 019efa5e-5f23-70b3-b4d3-f5f1643764a3 to be Done

$ devlog list --project devlog
Wednesday, 2026-06-24 · 1 entry

  [~] 11:02  Fix flaky test in store.rs · devlog
      id: 019efa5e-5f2a-7eb0-9ed7-9980495715a5
```

Entries are grouped under a day header — `<weekday>, <YYYY-MM-DD> · <count>` —
with the most recent day first. Within a day, each entry shows a status marker,
its local `HH:MM` time, the message, and — if the entry is tagged — a
`· <project>` suffix, followed by the full UUID on an indented `id:` line:

- `[~]` — in progress, shown in **yellow** (the state every new entry starts in)
- `[✓]` — done, shown in **green**
- `[x]` — cancelled, shown in **red** with the message struck through

Move an entry between states with `devlog set-status <id> <status>`, passing the
full id from the `list` output and one of `in_progress`, `done`, or `cancelled`.
If the entry is already in that state, `devlog` says so and changes nothing; an
unknown id reports `Item <id> not found!`.

### Color output

`devlog` colorizes its output so a long history is easy to scan: status markers
and labels are color-coded (`[~]` yellow, `[✓]` green, `[x]` red), project tags
and freshly added messages are cyan, and timestamps and ids are dimmed.

Color is enabled only when writing to a terminal — piping or redirecting drops
the escape codes automatically. Override that detection with the global
`--color` flag, which works on any command:

```bash
devlog list --color always   # force color, e.g. when paging
devlog list --color never    # plain text, no escape codes
devlog list --color auto     # default: color only on a TTY
```

### Projects

Group related entries by tagging them with `-p`/`--project` when you add them:

```console
$ devlog add "Bump rusqlite to 0.40" --project devlog
Added item "Bump rusqlite to 0.40"!
```

The project is created automatically the first time you mention it. Names are
trimmed of surrounding whitespace, and a blank `--project` is rejected with
`project cannot be empty`. Narrow your history to a single project with
`devlog list --project devlog`; a plain `devlog list` still shows everything,
tagged or not.

## How your data is stored

Everything lives in one SQLite database, created on first run:

```
~/.devlog/entries.sqlite
```

The schema is two tables — entries, and the projects they can be tagged with:

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

`devlog_entries`:

| Column | Type | Notes |
|---|---|---|
| `id` | `TEXT` | UUID v7 — time-ordered, generated per entry. |
| `created_at` | `TEXT` | UTC timestamp in RFC 3339; the `CHECK` rejects anything SQLite can't parse as a datetime. |
| `message` | `TEXT` | The note. Must be non-empty after trimming whitespace. |
| `status` | `TEXT` | Lifecycle state — one of `in_progress` (the default for new entries), `done`, or `cancelled`. |
| `last_updated` | `TEXT` | UTC timestamp of the last status change; equals `created_at` for fresh entries. The `1970` default exists only to backfill rows created before this column. |
| `project_name` | `TEXT` | Optional project tag — a foreign key into `devlog_local_projects(name)`, or `NULL` for untagged entries. |

`devlog_local_projects` holds one row per project, created on demand the first
time you tag an entry with `--project`.

The schema is versioned (via SQLite's `PRAGMA user_version`, currently `3`), so
existing journals are migrated in place when you upgrade `devlog` — the `status`
column was added this way, and later the projects table, `project_name`, and
`last_updated`.

Because it is plain SQLite, you can always inspect or back up your journal with
ordinary tools:

```bash
sqlite3 ~/.devlog/entries.sqlite "SELECT created_at, status, project_name, message FROM devlog_entries;"
```

## Project layout

```
devlog/
├── Cargo.toml              # crate: d3vlog · binary: devlog
└── src/
    ├── main.rs             # entry point — open the store, dispatch commands
    ├── cli.rs              # clap definitions + the global `--color` flag
    ├── style.rs            # `--color` choice and per-status color styles (anstyle)
    ├── cli/
    │   ├── commands.rs     # commands module root
    │   └── commands/
    │       ├── add.rs      # `add`        — insert a new entry
    │       ├── list.rs     # `list`       — group by day and print
    │       └── set_status.rs # `set-status` — update an entry's status
    ├── store.rs            # SQLite connection, schema migrations, reads & writes
    ├── store/
    │   └── result.rs       # outcome enum for a status update
    ├── data.rs             # data module root
    └── data/
        ├── entry.rs        # DevLogEntry model + display formatting
        ├── project.rs      # LocalProject model
        └── status.rs       # DevLogEntryStatus enum + parsing & rendering
```

The dependencies are intentionally few:

- [`clap`](https://crates.io/crates/clap) — argument parsing (derive API)
- [`rusqlite`](https://crates.io/crates/rusqlite) — SQLite access (bundled)
- [`chrono`](https://crates.io/crates/chrono) — UTC timestamps
- [`uuid`](https://crates.io/crates/uuid) — UUID v7 identifiers
- [`anstyle`](https://crates.io/crates/anstyle) — terminal text styling
- [`anstream`](https://crates.io/crates/anstream) — color-aware stdout (auto-strips on non-TTY)

## Building from source

```bash
# Run without installing
cargo run -- add "Trying devlog from a checkout"
cargo run -- list

# Optimized build
cargo build --release   # binary at target/release/devlog
```

## Roadmap

Ideas under consideration — **not yet implemented**:

- [ ] `devlog search <term>` — filter entries by text
- [ ] `devlog rm` / `devlog edit` — remove or amend entries
- [ ] Date filters (`--since`, `--today`, `--week`)
- [ ] Tags per entry
- [ ] Export to Markdown or JSON
- [ ] A scrollable TUI view

Have a different itch? Open an issue.

## Contributing

Issues and pull requests are welcome.

```bash
git clone https://github.com/w3lt/devlog
cd devlog
cargo build
cargo run -- list
```

Please keep changes small and focused, and run `cargo fmt` and `cargo clippy`
before opening a PR.

## License

Licensed under the **Apache License, Version 2.0**. See [`LICENSE`](LICENSE) or
<https://www.apache.org/licenses/LICENSE-2.0> for the full text.

<img src="assets/divider.svg" width="100%" alt="" />

<div align="center">
  <sub>Built with 🦀 Rust and SQLite · <a href="https://github.com/w3lt/devlog">github.com/w3lt/devlog</a></sub>
</div>
