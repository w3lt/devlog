---
title: Introduction
description: What devlog does and what is planned next.
---

`devlog` is a tiny developer journal for the terminal. It gives you a local,
timestamped trail of your work without requiring a web app, account, or
background process.

## Features

| | |
|---|---|
| 📓 **Frictionless capture** | One short command — `devlog add "…"` — and the thought is saved. |
| 📜 **Full history at a glance** | `devlog list` groups every entry by day, most recent day first. |
| ✅ **Track each entry's state** | Move items between `in_progress`, `done`, and `cancelled` with `devlog set-status`. |
| 🏷️ **Group by project** | Tag an entry with `--project` and filter your history down to one project at a time. |
| 🗃️ **Local-first SQLite** | Your journal lives in `~/.devlog/entries.sqlite`. No cloud, no account. |
| 🆔 **Time-ordered UUID v7** | IDs encode creation time, so entries sort naturally by when they happened. |
| 🕓 **Honest timestamps** | Stored in UTC (RFC 3339), shown in your local time when you `list`. |
| 🦀 **One small binary** | SQLite is bundled at build time — nothing to install alongside it. |
| 🔒 **Quiet by design** | No telemetry, no background process, no network. |

## Roadmap

Ideas under consideration, not yet implemented:

- `devlog search <term>` — filter entries by text
- `devlog rm` / `devlog edit` — remove or amend entries
- Date filters (`--since`, `--today`, `--week`)
- Tags per entry
- Export to Markdown or JSON
- A scrollable TUI view

Have a different itch? Open an issue.
