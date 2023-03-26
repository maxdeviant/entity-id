# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.4] - 2023-03-26

### Added

- Added `EntityId` trait

## [0.0.3] - 2023-03-26

### Added

- Added a separate `derive` feature (enabled by default)

### Changed

- Moved `uuid` interop behind the `uuid` feature

### Fixed

- Removed need for consumers to depend on `entity_id_core` directly

## [0.0.2] - 2023-03-26

### Added

- Added `PREFIX` associated constant to `EntityId`s

### Fixed

- Fully qualified identifiers in generated `impl`

## [0.0.1] - 2023-03-25

### Added

- Initial release

[unreleased]: https://github.com/maxdeviant/entity-id/compare/v0.0.4...HEAD
[0.0.4]: https://github.com/maxdeviant/entity-id/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/maxdeviant/entity-id/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/maxdeviant/entity-id/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/maxdeviant/entity-id/compare/1140d8f...v0.0.1
