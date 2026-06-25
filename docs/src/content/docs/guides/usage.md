---
title: Usage
description: CLI commands, typical sessions, status markers, and status updates.
---

## Command overview

```console
$ devlog --help
A tiny developer journal for the terminal

Usage: devlog <COMMAND>

Commands:
  add         Add a new journal entry
  list        List journal entries
  set-status  Set status of entry
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

| Command | What it does | Example |
|---|---|---|
| `devlog add <message> [-p <project>]` | Append a new entry, stamped with the current UTC time. Optionally tag it with a project. | `devlog add "Cut the v0.2 release" -p devlog` |
| `devlog list [-p <project>]` | Print every entry, grouped by day, newest day first. Pass `-p` to show one project only. | `devlog list -p devlog` |
| `devlog set-status <id> <status>` | Set an entry's status to `in_progress`, `done`, or `cancelled`. | `devlog set-status <id> done` |
| `devlog --version` | Show the installed version. | `devlog --version` |
| `devlog --help` | Show help, including for subcommands. | `devlog add --help` |

## A typical session

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

Entries are grouped under a day header, `<weekday>, <YYYY-MM-DD> · <count>`,
with the most recent day first. Within a day, each entry shows a status marker,
its local `HH:MM` time, the message, and, if the entry is tagged, a
`· <project>` suffix, followed by the full UUID on an indented `id:` line.

## Status markers

- `[~]` — in progress, the state every new entry starts in
- `[✓]` — done
- `[x]` — cancelled

## Updating status

Move an entry between states with:

```bash
devlog set-status <id> <status>
```

Pass the full id from the `list` output and one of:

- `in_progress`
- `done`
- `cancelled`

If the entry is already in that state, `devlog` says so and changes nothing. An
unknown id reports `Item <id> not found!`.
