# vibe-ticket AI Assistant Rules

ALWAYS use `vibe-ticket --help` when unsure. NEVER edit `.vibe-ticket/` files directly.
Before ANY code changes: `vibe-ticket new <slug> --title "<desc>"` then `vibe-ticket start <slug>`
The start command creates a Git worktree at `../<project>-ticket-<slug>/` - work there.
Track progress: `vibe-ticket task add "<task>"` and `vibe-ticket task complete <n>`
When done: `vibe-ticket close <slug> --message "<summary>"` and reference ticket ID in commits.