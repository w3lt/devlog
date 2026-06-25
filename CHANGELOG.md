# Changelog

All notable changes to this project are documented here. Entries are grouped by
crate version and based on closed GitHub issues for `w3lt/devlog`.

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
