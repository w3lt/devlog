<div align="center">

<img src="assets/hero.svg" width="820" alt="devlog — a tiny developer journal that lives in your terminal" />

<p>
  <img src="https://img.shields.io/badge/version-0.2.0-3b82f6?style=flat-square" alt="version" />
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
| 📜 **Full history at a glance** | `devlog list` replays every entry, oldest to newest. |
| 🗃️ **Local-first SQLite** | Your journal lives in `~/.devlog/entries.sqlite`. No cloud, no account. |
| 🆔 **Time-ordered UUID v7** | IDs encode creation time, so entries sort naturally by when they happened. |
| 🕓 **Honest timestamps** | Stored in UTC (RFC 3339) and shown as `YYYY-MM-DD HH:MM UTC`. |
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

Usage: devlog <COMMAND>

Commands:
  add   Add a new journal entry
  list  List journal entries
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

| Command | What it does | Example |
|---|---|---|
| `devlog add <message>` | Append a new entry, stamped with the current UTC time. | `devlog add "Cut the v0.2 release"` |
| `devlog list` | Print every entry in creation order. | `devlog list` |
| `devlog --version` | Show the installed version. | `devlog --version` |
| `devlog --help` | Show help (works on subcommands too). | `devlog add --help` |

### A typical session

```console
$ devlog add "Ship the new auth flow"
Added item "Ship the new auth flow"!

$ devlog add "Fix flaky test in store.rs"
Added item "Fix flaky test in store.rs"!

$ devlog list
[2026-06-23 09:14 UTC] 0190a1b2 Ship the new auth flow
[2026-06-23 11:02 UTC] 0190a3c4 Fix flaky test in store.rs
```

Each `list` line is `[<created_at> UTC] <short-id> <message>`, where the short
id is the first 8 characters of the entry's UUID v7.

## How your data is stored

Everything lives in one SQLite database, created on first run:

```
~/.devlog/entries.sqlite
```

The schema is a single table:

```sql
CREATE TABLE IF NOT EXISTS devlog_entries (
    id          TEXT PRIMARY KEY NOT NULL,
    created_at  TEXT NOT NULL CHECK (datetime(created_at) IS NOT NULL),
    message     TEXT NOT NULL CHECK (length(trim(message)) > 0)
);
```

| Column | Type | Notes |
|---|---|---|
| `id` | `TEXT` | UUID v7 — time-ordered, generated per entry. |
| `created_at` | `TEXT` | UTC timestamp in RFC 3339; the `CHECK` rejects anything SQLite can't parse as a datetime. |
| `message` | `TEXT` | The note. Must be non-empty after trimming whitespace. |

Because it is plain SQLite, you can always inspect or back up your journal with
ordinary tools:

```bash
sqlite3 ~/.devlog/entries.sqlite "SELECT created_at, message FROM devlog_entries;"
```

## Project layout

```
devlog/
├── Cargo.toml          # crate: d3vlog · binary: devlog
└── src/
    ├── main.rs         # entry point — parse args, dispatch commands
    ├── cli.rs          # clap definitions for `add` and `list`
    ├── store.rs        # SQLite connection, schema, reads & writes
    ├── data.rs         # data module root
    └── data/
        └── entry.rs    # DevLogEntry model + display formatting
```

The dependencies are intentionally few:

- [`clap`](https://crates.io/crates/clap) — argument parsing (derive API)
- [`rusqlite`](https://crates.io/crates/rusqlite) — SQLite access (bundled)
- [`chrono`](https://crates.io/crates/chrono) — UTC timestamps
- [`uuid`](https://crates.io/crates/uuid) — UUID v7 identifiers

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
- [ ] Tags / projects per entry
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
