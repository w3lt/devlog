---
title: Project layout
description: Repository layout, dependencies, and source-build commands.
---

## Repository layout

```text
devlog/
├── Cargo.toml              # crate: d3vlog · binary: devlog
└── src/
    ├── main.rs             # entry point: open the store, dispatch commands
    ├── cli.rs              # clap definitions for add, list, and set-status
    ├── cli/
    │   ├── commands.rs     # commands module root
    │   └── commands/
    │       ├── add.rs      # add: insert a new entry
    │       ├── list.rs     # list: group by day and print
    │       └── set_status.rs # set-status: update an entry's status
    ├── store.rs            # SQLite connection, schema migrations, reads, writes
    ├── store/
    │   └── result.rs       # outcome enum for a status update
    ├── data.rs             # data module root
    └── data/
        ├── entry.rs        # DevLogEntry model and display formatting
        ├── project.rs      # LocalProject model
        └── status.rs       # DevLogEntryStatus enum, parsing, rendering
```

## Dependencies

The dependencies are intentionally few:

- [`clap`](https://crates.io/crates/clap) — argument parsing with the derive API
- [`rusqlite`](https://crates.io/crates/rusqlite) — SQLite access, bundled
- [`chrono`](https://crates.io/crates/chrono) — UTC timestamps
- [`uuid`](https://crates.io/crates/uuid) — UUID v7 identifiers

## Building from source

Run without installing:

```bash
cargo run -- add "Trying devlog from a checkout"
cargo run -- list
```

Create an optimized build:

```bash
cargo build --release
```

The optimized binary is written to `target/release/devlog`.
