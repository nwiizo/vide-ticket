# Claude Code Integration

vibe-ticket includes built-in support for [Claude Code](https://claude.ai/code) through automatic CLAUDE.md generation.

## Overview

CLAUDE.md files help Claude Code understand your project structure, commands, and workflows. vibe-ticket can automatically generate and maintain these files with project-specific information.

## Generating CLAUDE.md

### During Project Initialization

The simplest way to get started is during project initialization:

```bash
# Initialize project with CLAUDE.md
vibe-ticket init --claude-md

# Alternative syntax
vibe-ticket init --claude
```

This creates a comprehensive CLAUDE.md file that includes:
- Project name and description
- Common vibe-ticket commands with examples
- Current project configuration
- Workflow guidelines
- Best practices for ticket management
- The initialization command itself (for reproducibility)

### For Existing Projects

Add CLAUDE.md to existing projects using the config command:

```bash
# Generate basic CLAUDE.md
vibe-ticket config claude

# Generate with advanced template
vibe-ticket config claude --template advanced

# Append to existing CLAUDE.md (preserves custom content)
vibe-ticket config claude --append

# Custom output location
vibe-ticket config claude --output ./docs/CLAUDE.md

# Combine options
vibe-ticket config claude --template advanced --append --output ./CLAUDE.md
```

## Template Options

### Basic Template (default)

The basic template includes:
- **Project Overview**: Name, description, and purpose
- **Essential Commands**: Common vibe-ticket operations with examples
- **Configuration**: Current project settings (Git integration, default priority, etc.)
- **Statistics**: Real-time ticket counts (total, active, completed)
- **Workflow Guidelines**: Standard ticket management practices
- **Best Practices**: Naming conventions and organizational tips

### Advanced Template

The advanced template includes everything from basic plus:
- **Git Worktree Support**: Examples for parallel development
- **Advanced Search**: Complex filtering and regex patterns
- **Export/Import**: Data migration and backup commands
- **Environment Variables**: `VIDE_TICKET_PROJECT`, `VIDE_TICKET_NO_COLOR`, etc.
- **Git Hooks Integration**: Pre-commit and post-checkout examples
- **Troubleshooting**: Common issues and solutions
- **Performance Tips**: Optimization strategies

## Dynamic Content

Generated CLAUDE.md files include dynamically populated information:
- Current date of generation
- Active project configuration values
- Real-time ticket statistics
- Git integration status
- Project-specific settings

## Adding AI Agent Rules

To ensure AI assistants follow proper vibe-ticket workflows, append the agent rules to your CLAUDE.md:

```bash
# Append agent rules from GitHub (when published)
curl https://raw.githubusercontent.com/nwiizo/vibe-ticket/main/rules/agent.md >> CLAUDE.md

# Or append from local file
cat rules/agent.md >> CLAUDE.md
```

This adds strict guidelines that help AI agents:
- Always use vibe-ticket commands (never edit `.vibe-ticket/` files directly)
- Create tickets before starting any work
- Properly manage Git worktrees
- Follow correct status transitions
- Track all development work with tickets

## Workflow Examples

### New Project with AI Assistance

```bash
# Create and initialize project
mkdir my-project && cd my-project
vibe-ticket init --claude-md

# Open with Claude Code
claude .

# Claude now understands your project structure
```

### Adding to Existing Project

```bash
# Generate initial CLAUDE.md
vibe-ticket config claude

# Later, after configuration changes
vibe-ticket config set git.auto_branch true
vibe-ticket config claude --append  # Updates CLAUDE.md
```

### Team Onboarding

```bash
# Generate comprehensive documentation
vibe-ticket config claude --template advanced

# Add team-specific instructions
echo "## Team Conventions" >> CLAUDE.md
echo "- Use 'feat/' prefix for feature branches" >> CLAUDE.md
```

## Claude Code Benefits

With a properly configured CLAUDE.md, Claude Code can:

1. **Understand Project Context**
   - Knows available commands and their usage
   - Understands project structure and conventions
   - Aware of current configuration and settings

2. **Provide Better Assistance**
   - Suggests appropriate vibe-ticket commands
   - Follows established workflows
   - Respects project-specific conventions

3. **Automate Common Tasks**
   - Generate tickets from error logs
   - Create task breakdowns
   - Suggest implementation approaches

4. **Maintain Consistency**
   - Uses correct naming conventions
   - Follows team practices
   - Applies project standards

## Best Practices

1. **Initial Setup**: Always use `--claude-md` during initialization for new projects
2. **Regular Updates**: Run `config claude --append` after major configuration changes
3. **Customization**: Add project-specific sections after generation
4. **Version Control**: Commit CLAUDE.md to track project evolution
5. **Team Alignment**: Review and update CLAUDE.md during team meetings

## Advanced Usage

### Custom Sections

After generation, add custom sections for your team:

```markdown
## Custom Commands
- `make deploy`: Deploy to production
- `npm run e2e`: Run end-to-end tests

## Architecture Decisions
- We use feature flags for gradual rollouts
- All API endpoints require authentication
```

### Integration with CI/CD

```yaml
# .github/workflows/update-claude.yml
on:
  push:
    paths:
      - '.vibe-ticket/config.yaml'
jobs:
  update-claude:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: vibe-ticket config claude --append
      - uses: EndBug/add-and-commit@v9
        with:
          message: 'chore: update CLAUDE.md'
```