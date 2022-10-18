# CHANGELOG

## [2.0.0](https://github.com/vihu/gogo/releases/tag/v2.0.0) - 2022-10-18

### Breaking Changes

- **NOTE** `v2.0.0` switches backend from rocksdb to sqlite. It is backward
  incompatible with any `v1.0.*`.

### Migrating from v1.0.* -> v2.0.0

- Ensure you are using `v1.0.1`.
- Run `gogo export`, take note of the `output_ts.csv` file.
- Update to `v2.0.0`.
- Ensure you've correctly set `GOGODB_PATH` env var.
- Run `gogo import /path/to/output_ts.csv`.

## [1.0.1](https://github.com/vihu/gogo/releases/tag/v1.0.1) - 2022-10-17

### Added

- Added export command, to allow transition from rocksdb backend to sqlite
  backend in the next release.
