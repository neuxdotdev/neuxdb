# Changelog

## [0.1.0] - 2026-04-27

### Added

- Initial release of NeuxDb embedded database
- SQL-like parser for CREATE, INSERT, SELECT, UPDATE, DELETE
- CSV-based table storage with pipe delimiter
- Schema validation with ColumnType (Text, Int)
- File locking for concurrent read/write safety
- Environment variable config support (NEUXDB_DATA_DIR)
- Basic error types: TableNotFound, ValueCountMismatch, Parse, etc.

### Changed

- Project structure migrated from `apis` to `core` module namespace
- Config system refactored to support runtime overrides
- Parser switched to token-based iterator architecture

### Fixed

- Typo corrections: `tabel` → `table`, `scema` → `schema`

### Removed

- CLI binary focus shifted to library-first design
- Legacy encryption dependencies (age, chrono, clap) from initial scope
