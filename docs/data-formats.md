# Data Formats

## Export/Import JSON Format

```json
{
  "tickets": [
    {
      "id": "uuid-here",
      "slug": "202507201345-feature-name",
      "title": "Feature Title",
      "description": "Detailed description",
      "priority": "high",
      "status": "todo",
      "tags": ["backend", "api"],
      "created_at": "2025-07-20T13:45:00Z",
      "started_at": null,
      "closed_at": null,
      "assignee": "username",
      "tasks": [
        {
          "title": "Task description",
          "completed": false
        }
      ],
      "metadata": {}
    }
  ]
}
```

## CSV Format

CSV exports include the following columns:
- ID, Slug, Title, Status, Priority, Assignee, Tags, Created At, Started At, Closed At, Tasks Total, Tasks Completed, Description

## File Structure

```
.vibe-ticket/
├── config.yaml          # Project configuration
├── state.yaml          # Project state and metadata
├── active_ticket       # Currently active ticket ID
├── tickets/            # Ticket YAML files
│   ├── <ticket-id>.yaml
│   ├── <ticket-id>.yaml.lock  # Lock file (temporary, auto-cleaned)
│   └── ...
├── specs/              # Specification files
│   ├── <spec-id>.yaml
│   └── ...
├── templates/          # Custom ticket templates
│   ├── bug.yaml
│   └── feature.yaml
├── plugins/            # Plugin extensions
└── backups/            # Backup files
```

## Concurrent Access Protection

vibe-ticket uses file-based locking to ensure data integrity when multiple processes or users access tickets simultaneously:

### Lock Files
- Lock files (`*.lock`) are created automatically when modifying tickets
- They contain metadata about the lock holder and operation
- Locks are automatically released when operations complete
- Stale locks (older than 30 seconds) are cleaned up automatically

### Lock File Format
```json
{
  "holder_id": "uuid-v4",
  "pid": 12345,
  "acquired_at": 1234567890,
  "operation": "save_ticket"
}
```

### Retry Mechanism
- Operations retry up to 10 times with 100ms delays if a file is locked
- This ensures smooth operation even under concurrent access
- Users experience no interruption during normal usage