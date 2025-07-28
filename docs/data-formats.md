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