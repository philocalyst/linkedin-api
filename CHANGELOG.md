# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1] - 2025-09-27

### Added
- Added back wrapper types for NetworkInfo, School, Company, Conversation, ConversationDetails, and SearchPeopleParams.
- Implemented `FromStr` for `BirthDate`.
- Added important typing crates: `aho-corasick`, `anyhow`, `bincode`, `colored`, `csv`, `csv-core`, `either`, `email_address`, `heck`, `isolang`, `linked-hash-map`, `lru-cache`, `minimal-lexical`, `my_country`, `nom`, `oncemutex`, `phf`, `phf_shared`, `phonenumber`, `prettyplease`, `quick-xml`, `regex`, `regex-automata`, `regex-cache`, `regex-syntax`, `same-file`, `serde_yaml`, `siphasher`, `strum`, `strum_macros`, `thiserror`.
- Added `.cookies.json` for cookie management.

### Changed
- Made several fields in `Education` and `TimePeriod` optional.
- Made the core library type-aware by using strongly typed fields like `EmailAddress`, `BirthDate`, `Url`, and `PhoneNumber`.
- Fixed the naming scheme by removing `#[serde(rename = "entityUrn")]` and similar attributes from many fields.
- Introduced a new approach to typing, refactoring `types.rs` with new structs for `LinkedInProfileView`, `PersonName`, `Address`, `GeoLocation`, `BirthDate`, `Paging`, `CertificationView`, `Certification`, `CourseView`, `Course`, `HonorView`, `Honor`, `LanguageView`, `Language`, `LanguageProficiency`, `Experience`, `CompanyInfo`, `EmployeeCountRange`, `MiniCompany`, `Education`, `SchoolInfo`, `TestScoreView`, `TestScore`, `ContactInfo`, `Website`, `PositionGroupView`, `PositionGroup`, `PositionView`, `EducationView`, `SkillView`, `Skill`, `VolunteerExperienceView`, `VolunteerExperience`, `VolunteerCauseView`, `VolunteerCause`, `OrganizationView`, `PatentView`, `ProjectView`, `PublicationView`.
- Added back skipped fields and moved back towards the naming scheme in `Profile` and `ContactInfo`.

### Removed
- Removed all time period "convenience" methods (`duration_months`, `duration_string`).

### Fixed
- Fixed the language typing on `Locale` from `Language` to `isolang::Language`.

## [0.3.0] – 2025-09-26

### Added
  * **Profiles** now include **contact information** by default when fetching a profile.
      * This includes the new `ContactInfo` struct, which is added to the `Profile` struct.
      * Updated the internal `get_profile_contact_info` function to correctly handle both `public_id` and `UniformResourceName` as input.
      * The `get_profile_contact_info_by_urn` function now correctly accepts a `&UniformResourceName` as an argument.
  * **Experience** entries now include a `role` and detailed `time_period` structure.
      * This required adding the `TimePeriod` and `YearMonth` structs with custom serialization/deserialization logic to correctly handle the time data.
  * The **`time` crate** dependency was added with the `["serde"]` feature to support the new time-related data structures.
  * The `UniformResourceName` struct's visibility was changed from crate-private (`pub(crate)`) to **public** (`pub`).

### Changed
  * Minor refactoring for cleaner string formatting in internal API calls (removed verbose `&format!(...)` in a few places in `src/linkedin.rs`).

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

[Unreleased]: https://github.com/your-org/your-repo/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/your-org/your-repo/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/philocalyst/linkedin-api/compare/v0.2.1...v0.3.0 
[0.2.1]: https://github.com/philocalyst/linkedin-api/compare/v0.2.0...v0.2.1 
[0.2.0]: https://github.com/philocalyst/linkedin-api/compare/v0.1.0...v0.2.0 
[0.1.0]: https://github.com/philocalyst/linkedin-api/compare/v0.1.0... (Comparing against the start of the project)
