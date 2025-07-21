# Changelog

All notable changes to vide-ticket will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2025-07-21

### Fixed
- Critical: Unicode boundary error in output formatting that caused panic when displaying multi-byte characters (e.g., Japanese text)
- String truncation now properly handles character boundaries instead of byte boundaries

### Added
- Comprehensive tests for multi-byte character handling
- Test coverage for emoji and mixed ASCII/Unicode strings

### Developer Experience
- Successfully dogfooded vide-ticket for its own development
- Identified and documented multiple improvement opportunities

## [0.1.0] - 2025-07-21

### Added
- Initial release of vide-ticket
- Core ticket management functionality
- Git integration for branch creation
- CLAUDE.md generation for AI-assisted development
- Comprehensive CLI with multiple subcommands
- Export/Import functionality (JSON, CSV, Markdown)
- Task management within tickets
- Specification-driven development support
- Search and filtering capabilities
- Archive functionality
- Configuration management

### Features
- High-performance ticket operations (10x faster than shell-based alternatives)
- Multiple output formats (table, JSON)
- Rich priority and status management
- Time tracking for tickets
- Tag-based organization
- Flexible date filtering

[0.1.1]: https://github.com/nwiizo/vide-ticket/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/nwiizo/vide-ticket/releases/tag/v0.1.0
