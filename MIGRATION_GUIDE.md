# Migration Guide: vide-ticket to vibe-ticket

This guide helps existing users migrate from the incorrectly named `vide-ticket` to the correctly named `vibe-ticket`.

## Background

The project was initially named `vide-ticket` but should have been `vibe-ticket` to match the Vibe Coding environment. This has been corrected in version 0.1.1.

## Migration Steps

### 1. Update Your Installation

If you installed from source:
```bash
# Pull the latest changes
git pull origin main

# Rebuild and install
cargo install --path . --force
```

If you installed from crates.io (when available):
```bash
cargo install vibe-ticket --force
```

### 2. Update Your Shell Configuration

If you have any aliases or scripts referencing `vide-ticket`, update them to use `vibe-ticket`:

```bash
# Old
alias vt='vide-ticket'

# New
alias vt='vibe-ticket'
```

### 3. Update Environment Variables (If Used)

The following environment variables have been renamed:
- `VIDE_TICKET_PROJECT` → `VIBE_TICKET_PROJECT`
- `VIDE_TICKET_NO_COLOR` → `VIBE_TICKET_NO_COLOR`
- `VIDE_TICKET_JSON` → `VIBE_TICKET_JSON`

Note: These environment variables are documented but not yet implemented in the current version.

### 4. Update Git Hooks

If you have Git hooks that reference `vide-ticket`, update them:

```bash
# Example: Update .git/hooks/prepare-commit-msg
sed -i 's/vide-ticket/vibe-ticket/g' .git/hooks/*
```

### 5. Update CI/CD Pipelines

Update any GitHub Actions, GitLab CI, or other CI/CD configurations:

```yaml
# Old
- run: vide-ticket check

# New
- run: vibe-ticket check
```

## What Hasn't Changed

- All command-line arguments and options remain the same
- Configuration file formats are unchanged
- Ticket data format is unchanged
- Project structure (`.vibe-ticket/` directory) remains the same

## Compatibility

- The binary name has changed from `vide-ticket` to `vibe-ticket`
- All internal references have been updated
- No changes to ticket data or project files are required

## Getting Help

If you encounter any issues during migration:
1. Check the project issues: https://github.com/nwiizo/vibe-ticket/issues
2. Review the documentation: `vibe-ticket --help`
3. Create a new issue with the migration tag

## Timeline

- v0.1.0: Original release as `vide-ticket`
- v0.1.1: Renamed to `vibe-ticket` (current)