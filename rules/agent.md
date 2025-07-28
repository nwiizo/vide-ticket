# vibe-ticket AI Assistant Rules

ALWAYS use `vibe-ticket --help` when unsure. NEVER edit `.vibe-ticket/` files directly.
Before ANY code changes: `vibe-ticket new <slug> --title "<desc>"` then `vibe-ticket start <slug>`
The start command creates a Git worktree at `./<project>-vibeticket-<slug>/` in the current directory. ALWAYS work in that worktree.
Track progress: `vibe-ticket task add "<task>"` and `vibe-ticket task complete <n>`
When done: `vibe-ticket close <slug> --message "<summary>"` and reference ticket ID in commits.
IMPORTANT: Worktrees are created in the current directory (not parent), making them visible to Claude Code.