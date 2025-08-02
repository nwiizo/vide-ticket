# vibe-ticket v0.2.2 Release Notes

## Overview

This patch release focuses on code quality improvements and CI reliability enhancements. We've resolved all clippy warnings, standardized code formatting, and fixed various CI configuration issues.

## What's Changed

### 🐛 Bug Fixes
- **Removed unnecessary async functions**: Fixed 61 clippy warnings about async functions that don't use await in MCP handlers
- **MSRV consistency**: Aligned Minimum Supported Rust Version between Cargo.toml and clippy.toml to 1.85.0
- **CI workflow fix**: Removed dependency on non-existent 'license' job that was causing workflow failures
- **License checking**: Updated cargo-deny configuration to properly allow Unicode-3.0 license used by dependencies

### 🎨 Code Quality
- **Code formatting**: Applied `cargo fmt` to entire codebase (34 files formatted) for consistent code style
- **Rust 2024 Edition**: Updated project to use Rust 2024 Edition
- **CI improvements**: Enhanced CI configuration for better reliability and faster feedback

### 📝 Documentation
- Updated CHANGELOG.md with all recent changes
- Improved deny.toml configuration documentation

## Technical Details

### Async Function Cleanup
We identified and fixed 61 instances where functions were marked as `async` but didn't actually use `.await`. This cleanup improves compilation efficiency and code clarity:
- `src/mcp/handlers/config.rs`: 2 functions
- `src/mcp/handlers/search.rs`: 2 functions  
- `src/mcp/handlers/tickets.rs`: 3 functions
- `src/mcp/handlers/worktree.rs`: 2 functions
- Related call sites in `src/mcp/service.rs` were updated

### Rust Version Update
- MSRV updated from 1.70.0 to 1.85.0 across all configurations
- Project now uses Rust 2024 Edition
- All CI jobs updated to test against the new MSRV

### Dependency Management
- cargo-deny now properly configured to handle all project dependencies
- Unicode-3.0 license added to allowed licenses list
- Duplicate dependency warnings are tracked but don't block CI

## Upgrade Guide

This is a patch release with no breaking changes. Simply update your dependency:

```toml
[dependencies]
vibe-ticket = "0.2.2"
```

Or update using cargo:

```bash
cargo update -p vibe-ticket
```

## Requirements

- Rust 1.85.0 or later (MSRV updated)
- All other requirements remain the same as v0.2.1

## Contributors

Thanks to all contributors who helped improve the code quality and CI reliability!

## Full Changelog

For a complete list of changes, see the [CHANGELOG.md](https://github.com/nwiizo/vibe-ticket/blob/main/CHANGELOG.md)

---

*Released on: August 2, 2025*