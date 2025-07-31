# Spec-Driven Development with vibe-ticket

## Overview

vibe-ticket supports a specification-driven development workflow inspired by Kiro methodology. This approach emphasizes thorough planning through three distinct phases before implementation begins.

## Core Concepts

### Three-Phase Development

1. **Requirements Definition (要件定義書)**
   - Define what needs to be built
   - Capture user stories and success criteria
   - Document functional and non-functional requirements

2. **Technical Design (技術設計書)**
   - Design how it will be built
   - Define architecture and component interactions
   - Specify data models and interfaces

3. **Implementation Plan (実装計画書)**
   - Plan the execution steps
   - Break down work into actionable tasks
   - Estimate timelines and identify dependencies

### Specification Structure

Each specification is stored in `.vibe-ticket/specs/<spec-id>/` with:
- `spec.json` - Metadata and progress tracking
- `requirements.md` - Requirements definition document
- `design.md` - Technical design document
- `tasks.md` - Implementation plan document

## Getting Started

### 1. Initialize a Specification

```bash
vibe-ticket spec init "Feature Name" \
  --description "Brief description of the feature" \
  --ticket <ticket-id> \
  --tags "feature,backend"
```

### 2. Set Active Specification

```bash
# Set as active for convenience
vibe-ticket spec activate <spec-id>

# Or work with specific spec using -s flag
vibe-ticket spec requirements -s <spec-id>
```

### 3. Define Requirements

```bash
# Open requirements document in editor
vibe-ticket spec requirements --editor

# Or view current requirements
vibe-ticket spec requirements
```

The requirements template includes sections for:
- Background and Purpose
- Scope (In/Out of scope)
- Functional Requirements
- Non-functional Requirements
- Constraints and Assumptions
- Risks

### 4. Create Technical Design

```bash
# Open design document in editor
vibe-ticket spec design --editor

# Mark requirements as complete first
vibe-ticket spec requirements --complete
```

The design template covers:
- Architecture Overview
- Component Design
- Data Models
- Interface Definitions
- Security Design
- Performance Considerations

### 5. Plan Implementation

```bash
# Open tasks document in editor
vibe-ticket spec tasks --editor

# Mark design as complete
vibe-ticket spec design --complete
```

The tasks template includes:
- Milestones and Phases
- Detailed Task Breakdown
- Dependencies
- Time Estimates
- Risk Mitigation

### 6. Track Progress

```bash
# Check current status
vibe-ticket spec status --detailed

# List all specifications
vibe-ticket spec list

# Show complete specification
vibe-ticket spec show <spec-id> --all
```

## Workflow Integration

### Linking with Tickets

Specifications can be associated with vibe-tickets:

```bash
# Create spec linked to existing ticket
vibe-ticket spec init "Feature" --ticket <ticket-id>

# Export tasks to tickets (planned feature)
vibe-ticket spec tasks --export-tickets
```

### Phase Approval

Mark phases as approved for formal workflows:

```bash
# Approve requirements after review
vibe-ticket spec approve <spec-id> requirements \
  --message "Reviewed and approved by team"

# Approve design
vibe-ticket spec approve <spec-id> design \
  --message "Architecture approved"
```

### Version Control

Specifications are version-controlled:
- Each document edit increments patch version
- Major changes can be tracked through git history
- All changes update the `updated_at` timestamp

## Best Practices

### 1. Start with Clear Requirements
- Focus on the "what" not the "how"
- Include measurable success criteria
- Document edge cases and constraints

### 2. Design Before Coding
- Consider multiple approaches
- Document trade-offs and decisions
- Define clear interfaces between components

### 3. Break Down Work Effectively
- Create tasks that can be completed in 1-2 days
- Identify dependencies early
- Include time for testing and documentation

### 4. Keep Documents Updated
- Update specs as requirements change
- Document design decisions and changes
- Mark phases complete only when truly done

### 5. Use Templates Effectively
- Templates provide structure but aren't rigid
- Add sections as needed for your project
- Remove irrelevant sections to keep docs focused

## Advanced Usage

### Custom Templates

While vibe-ticket provides default templates, you can customize them:

1. Create a spec with default template
2. Edit to match your needs
3. Save as a reference for future specs

### Bulk Operations

```bash
# List specs in specific phase
vibe-ticket spec list --phase design

# Find specs by status
vibe-ticket spec list --status in_progress
```

### Integration with CI/CD

Specifications can be used in automation:
- Generate test cases from requirements
- Create API documentation from design specs
- Track implementation progress against tasks

## Example: Complete Feature Development

```bash
# 1. Create a new feature ticket
vibe-ticket new feature-auth --title "Add JWT authentication" -P high

# 2. Initialize specification
vibe-ticket spec init "JWT Authentication System" \
  --description "Implement JWT-based auth with refresh tokens" \
  --ticket feature-auth

# 3. Work through phases
vibe-ticket spec requirements -e  # Define what we need
vibe-ticket spec requirements -c   # Mark complete

vibe-ticket spec design -e         # Design the solution
vibe-ticket spec design -c         # Mark complete

vibe-ticket spec tasks -e          # Plan implementation
vibe-ticket spec tasks -c          # Mark complete

# 4. Start implementation
vibe-ticket start feature-auth

# 5. Track progress
vibe-ticket spec status -d
```

## Troubleshooting

### Lost Active Specification
```bash
# Find your spec
vibe-ticket spec list

# Reactivate it
vibe-ticket spec activate <spec-id>
```

### Orphaned Spec Directories
```bash
# Clean up invalid specs
rm -rf .vibe-ticket/specs/<invalid-id>
```

### Document Conflicts
- Specs are stored as plain Markdown files
- Use git to resolve conflicts in spec documents
- The `spec.json` metadata should be merged carefully

## See Also

- [Commands Reference](commands.md) - Full command documentation
- [Configuration](configuration.md) - Project configuration options
- [Git Worktree](git-worktree.md) - Working with multiple features