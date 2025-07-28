# /ticket

Manage vibe-ticket tickets directly from Claude Code.

## Usage
```
/ticket                     # Show current ticket status
/ticket list                # List all tickets
/ticket start <slug>        # Start working on a ticket
/ticket new <slug> <title>  # Create a new ticket
/ticket close <slug>        # Close a ticket
/ticket show <slug>         # Show ticket details
/ticket todo                # Show TODO tickets
/ticket doing               # Show DOING tickets
```

## Description
This command provides quick access to vibe-ticket functionality without leaving Claude Code.

## Actions

```bash
! # Parse command arguments
! CMD="${ARGUMENTS%% *}"
! ARGS="${ARGUMENTS#* }"
! if [ "$CMD" = "$ARGUMENTS" ]; then
!   ARGS=""
! fi

! case "$CMD" in
!   "" | "status")
!     echo "📋 Current ticket status:"
!     vibe-ticket check
!     ;;
!   
!   "list")
!     echo "📊 All tickets:"
!     vibe-ticket list --limit 20
!     ;;
!   
!   "start")
!     if [ -z "$ARGS" ]; then
!       echo "❌ Usage: /ticket start <slug>"
!       exit 1
!     fi
!     echo "🚀 Starting ticket: $ARGS"
!     vibe-ticket start "$ARGS"
!     ;;
!   
!   "new")
!     SLUG="${ARGS%% *}"
!     TITLE="${ARGS#* }"
!     if [ -z "$SLUG" ] || [ "$SLUG" = "$TITLE" ]; then
!       echo "❌ Usage: /ticket new <slug> <title>"
!       exit 1
!     fi
!     echo "✨ Creating new ticket: $SLUG"
!     vibe-ticket new "$SLUG" --title "$TITLE"
!     ;;
!   
!   "close")
!     if [ -z "$ARGS" ]; then
!       echo "❌ Usage: /ticket close <slug>"
!       exit 1
!     fi
!     echo "✅ Closing ticket: $ARGS"
!     vibe-ticket close "$ARGS" --message "Completed via Claude Code"
!     ;;
!   
!   "show")
!     if [ -z "$ARGS" ]; then
!       echo "❌ Usage: /ticket show <slug>"
!       exit 1
!     fi
!     echo "🔍 Ticket details for: $ARGS"
!     vibe-ticket show "$ARGS"
!     ;;
!   
!   "task")
!     # Handle task subcommands
!     TASK_CMD="${ARGS%% *}"
!     TASK_ARGS="${ARGS#* }"
!     
!     case "$TASK_CMD" in
!       "add")
!         if [ -z "$TASK_ARGS" ] || [ "$TASK_ARGS" = "$ARGS" ]; then
!           echo "❌ Usage: /ticket task add <description>"
!           exit 1
!         fi
!         echo "➕ Adding task:"
!         vibe-ticket task add "$TASK_ARGS"
!         ;;
!       
!       "list")
!         echo "📝 Current tasks:"
!         vibe-ticket task list
!         ;;
!       
!       "complete")
!         if [ -z "$TASK_ARGS" ] || [ "$TASK_ARGS" = "$ARGS" ]; then
!           echo "❌ Usage: /ticket task complete <task-id>"
!           exit 1
!         fi
!         echo "✅ Completing task $TASK_ARGS"
!         vibe-ticket task complete "$TASK_ARGS"
!         ;;
!       
!       *)
!         echo "❌ Unknown task command: $TASK_CMD"
!         echo "Available: add, list, complete"
!         exit 1
!         ;;
!     esac
!     ;;
!   
!   "worktree")
!     # Handle worktree subcommands
!     WT_CMD="${ARGS%% *}"
!     
!     case "$WT_CMD" in
!       "list")
!         echo "🌳 Git worktrees for tickets:"
!         vibe-ticket worktree list
!         ;;
!       
!       "remove")
!         WT_ARGS="${ARGS#* }"
!         if [ -z "$WT_ARGS" ] || [ "$WT_ARGS" = "$ARGS" ]; then
!           echo "❌ Usage: /ticket worktree remove <ticket>"
!           exit 1
!         fi
!         echo "🗑️  Removing worktree for: $WT_ARGS"
!         vibe-ticket worktree remove "$WT_ARGS"
!         ;;
!       
!       *)
!         echo "❌ Unknown worktree command: $WT_CMD"
!         echo "Available: list, remove"
!         exit 1
!         ;;
!     esac
!     ;;
!   
!   "todo")
!     echo "📋 TODO tickets:"
!     vibe-ticket list --status todo --limit 10
!     ;;
!   
!   "doing")
!     echo "🚧 DOING tickets:"
!     vibe-ticket list --status doing --limit 10
!     ;;
!   
!   "help")
!     echo "🎫 vibe-ticket commands:"
!     echo ""
!     echo "Basic commands:"
!     echo "  /ticket                    Show current ticket status"
!     echo "  /ticket list               List all tickets"
!     echo "  /ticket todo               List TODO tickets"
!     echo "  /ticket doing              List DOING tickets"
!     echo "  /ticket new <slug> <title> Create a new ticket"
!     echo "  /ticket start <slug>       Start working on a ticket"
!     echo "  /ticket show <slug>        Show ticket details"
!     echo "  /ticket close <slug>       Close a ticket"
!     echo ""
!     echo "Task management:"
!     echo "  /ticket task add <desc>    Add a task to current ticket"
!     echo "  /ticket task list          List tasks for current ticket"
!     echo "  /ticket task complete <id> Complete a task"
!     echo ""
!     echo "Worktree management:"
!     echo "  /ticket worktree list      List ticket worktrees"
!     echo "  /ticket worktree remove    Remove a worktree"
!     ;;
!   
!   *)
!     echo "❌ Unknown command: $CMD"
!     echo "Try: /ticket help"
!     exit 1
!     ;;
! esac
```