# Changelog

All notable changes to Vikey will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2025-12-05

### Added

- **TelexMethodV2**: Complete rewrite using syllable-based architecture
- `Syllable` struct for Vietnamese syllable representation
- `Tone` enum with all Vietnamese tone marks
- `Modification` enum for letter modifications (circumflex, breve, horn, d-stroke)
- Comprehensive tone application logic (300+ character combinations)
- Demo UI with ratatui for real-time Vietnamese input testing

### Changed

- **BREAKING**: Recommended to use `telex_v2` instead of `telex` for better accuracy
- Demo UI now uses TelexMethodV2 by default
- Improved transformation pipeline (letter mods → tone marks)
- Always return complete syllable (fixes sync issues)

### Fixed

- Buffer desynchronization issues in old Telex implementation
- Unicode handling in character transformations
- Backspace count calculation
- Double key input bug in demo UI

## [0.2.0] - 2025-12-05

### Added

- New monorepo architecture with trait-based plugin system
- `vikey-vietnamese` crate with complete Vietnamese implementation
- `LanguagePlugin` trait for language support
- `InputMethodTrait` for input method implementations
- `LookupProvider` trait for character lookup
- `LanguageRules` trait for language-specific rules
- `Engine` as new main API
- `PluginRegistry` for managing language plugins
- Example: `basic_usage.rs` demonstrating new API

### Changed

- **BREAKING**: Refactored `vikey-core` to be 100% generic
- **BREAKING**: Removed `VikeyCore` struct (replaced by `Engine`)
- **BREAKING**: Removed `InputMethod` enum
- **BREAKING**: Removed Vietnamese-specific types from core (`ToneType`, `MarkType`, `Transformation`)
- Moved all Vietnamese logic to `vikey-vietnamese` crate

### Removed

- `processor.rs` - Vietnamese processing logic (moved to vikey-vietnamese)
- `lookup.rs` - Vietnamese lookup tables (moved to vikey-vietnamese)
- `spelling.rs` - Vietnamese spelling checker (moved to vikey-vietnamese)

## [0.1.0] - 2025-12-04

### Added

- Initial Vikey Core implementation
- Basic Telex, VNI, VIQR support
- Vietnamese input processing
- Buffer management
- Transformation tracking

[Unreleased]: https://github.com/HBCapital/Vikey/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/HBCapital/Vikey/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/HBCapital/Vikey/releases/tag/v0.1.0
