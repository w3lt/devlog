# Changelog

All notable changes to this project are documented here. Entries are grouped by
crate version and based on closed GitHub issues for `w3lt/devlog`.

## [0.9.0] - 2026-06-27

### Added

- Added colored, styled terminal output ([#14]): `devlog list`, `add`, and
  `set-status` color status markers and labels (yellow `in_progress`, green
  `done`, red `cancelled`), tint project tags and freshly added messages cyan,
  dim timestamps and ids, and strike through cancelled messages.
- Added a global `--color` flag (`auto`/`always`/`never`); color is detected
  from the terminal by default and stripped when output is piped or redirected.

## [0.8.1] - 2026-06-25

### Fixed

- Fixed `-p`/`--project` accepting empty or whitespace-only values ([#11]);
  `devlog add` and `devlog list` now reject blank projects with a clear error.

## [0.8.0] - 2026-06-25

### Added

- Added project support ([#7]): `devlog add` accepts `-p`/`--project`, creates
  local projects on first use, and `devlog list` can filter entries by project.
- Added SQLite schema support for local projects, entry `project_name`, and
  `last_updated` metadata.

## [0.7.0] - 2026-06-24

### Added

- Added entry status tracking ([#2]): new entries start as `in_progress`, can be
  moved to `done` or `cancelled` with `devlog set-status`, and `devlog list`
  shows status markers.
- Added a SQLite migration so existing journals receive the `status` column.

## [0.6.0] - 2026-06-24

### Fixed

- Fixed `devlog list` incorrectly requiring a `version` argument ([#1]); version
  details now come from the CLI's built-in `--version` output.

[#1]: https://github.com/w3lt/devlog/issues/1
[#2]: https://github.com/w3lt/devlog/issues/2
[#7]: https://github.com/w3lt/devlog/issues/7
[#11]: https://github.com/w3lt/devlog/issues/11
[#14]: https://github.com/w3lt/devlog/issues/14
