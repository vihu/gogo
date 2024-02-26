# CHANGELOG

## [2.0.5](https://github.com/vihu/gogo/releases/tag/v2.0.5) - 2024-02-25

### Removed

- Failing windows binary

## [2.0.4](https://github.com/vihu/gogo/releases/tag/v2.0.4) - 2024-02-25

### Added

- Bump version
- Update `serde-trim` dependency
- Use [cargo-dist](https://github.com/axodotdev/cargo-dist) for release management

## [2.0.1](https://github.com/vihu/gogo/releases/tag/v2.0.1) - 2022-10-19

### Added

- Clarify README.
- Add a basic test.
- Add github workflow.
- Update import/export commands to use supplied CSV file path.
- Fix version for `gogo` executable.

### Fixes

- Fixes [#7](https://github.com/vihu/gogo/issues/7).

## [2.0.0](https://github.com/vihu/gogo/releases/tag/v2.0.0) - 2022-10-18

### Breaking Changes

- **NOTE** `v2.0.0` switches backend from rocksdb to sqlite. It is backward
  incompatible with any `v1.0.*`.

### Migrating from v1.0.\* -> v2.0.0

- Ensure you are using `v1.0.1`.
- Run `gogo export`, take note of the `output_ts.csv` file.
- Update to `v2.0.0`.
- Ensure you've correctly set `GOGODB_PATH` env var.
- Run `gogo import /path/to/output_ts.csv`.

## [1.0.1](https://github.com/vihu/gogo/releases/tag/v1.0.1) - 2022-10-17

### Added

- Added export command, to allow transition from rocksdb backend to sqlite
  backend in the next release.
