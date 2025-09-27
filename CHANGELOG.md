# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1] - 2025-09-26
### Fixed
- The visibility on internal methods

## [0.2.0] - 2025-09-26

### Added
  - Implemented helper methods `get_full_name()` and `get_profile_image()` on the `Profile` struct for more convenient data access.

### Changed
  - Updated the authentication mechanism to rely on session tokens (`li_at` and `JSESSIONID`) instead of username and password credentials, improving security and stability.

### Fixed
  - Corrected the SPDX license expression in `Cargo.toml` to `AGPL-3.0-or-later`.
  - Updated the `basic.rs` example to be functional and align with the new authentication method.
  - Refactored the test suite for clarity and correctness.

## [0.1.0] - 2025-09-22

### Added 
- Implemented `PartialEq` for profile-related data structures to allow for direct comparison.

[Unreleased]: https://github.com/philocalyst/infat/compare/v0.2.1…HEAD
[0.2.1]: https://github.com/philocalyst/infat/compare/v0.2.0...v0.2.1 
[0.2.0]: https://github.com/philocalyst/infat/compare/v0.1.0...v0.2.0 
[0.1.0]: https://github.com/philocalyst/infat/compare/v0.1.0... (Comparing against the start of the project)
