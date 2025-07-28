# /work-on-ticket

Automatically find and work on the most appropriate ticket.

## Usage
```
/work-on-ticket              # Find and work on highest priority TODO ticket
/work-on-ticket <keyword>    # Find ticket matching keyword and work on it
/work-on-ticket continue     # Continue with current active ticket
/work-on-ticket next         # Complete current and move to next ticket
```

## Description
This command automates the ticket workflow by:
1. Finding the most appropriate ticket to work on
2. Starting work on it (creating worktree if needed)
3. Analyzing the ticket requirements
4. Suggesting implementation approach

## Actions

```bash
! echo "🤖 vibe-ticket Auto Work Mode"
! echo "================================"
! echo ""

! # Helper function to analyze ticket
! analyze_ticket() {
!   local SLUG="$1"
!   echo "📋 Analyzing ticket: $SLUG"
!   vibe-ticket show "$SLUG" | head -20
!   echo ""
!   echo "📁 Checking related files..."
!   
!   # Extract keywords from title and description for searching
!   TITLE=$(vibe-ticket show "$SLUG" | grep "Title:" | cut -d: -f2- | xargs)
!   DESC=$(vibe-ticket show "$SLUG" | grep -A5 "Description:" | tail -4)
!   
!   echo "🔍 Relevant context:"
!   echo "- Title: $TITLE"
!   echo ""
! }

! # Handle different modes
! MODE="$ARGUMENTS"

! case "$MODE" in
!   "continue")
!     # Continue with current ticket
!     echo "📍 Checking current ticket..."
!     CURRENT=$(vibe-ticket check | grep "Ticket:" | awk '{print $2}')
!     if [ -z "$CURRENT" ]; then
!       echo "❌ No active ticket found"
!       echo "💡 Use /work-on-ticket to find a ticket"
!       exit 1
!     fi
!     analyze_ticket "$CURRENT"
!     echo "💡 Ready to continue work on: $CURRENT"
!     ;;
!   
!   "next")
!     # Complete current and move to next
!     echo "✅ Completing current ticket..."
!     CURRENT=$(vibe-ticket check | grep "Ticket:" | awk '{print $2}')
!     if [ -n "$CURRENT" ]; then
!       vibe-ticket close "$CURRENT" --message "Completed via auto-work mode"
!       echo "✅ Closed ticket: $CURRENT"
!       echo ""
!     fi
!     # Fall through to find next ticket
!     MODE=""
!     ;;
! esac

! # If no specific mode or after completing, find next ticket
! if [ -z "$MODE" ] || [ "$MODE" = "next" ]; then
!   echo "🔍 Finding next ticket to work on..."
!   echo ""
!   
!   # Get highest priority TODO ticket
!   NEXT_TICKET=$(vibe-ticket list --status todo --limit 1 | grep "│" | grep -v "┌\|└\|ID\|──" | head -1 | awk -F'│' '{print $3}' | xargs)
!   
!   if [ -z "$NEXT_TICKET" ]; then
!     echo "🎉 No TODO tickets found! All caught up!"
!     exit 0
!   fi
!   
!   echo "📌 Selected ticket: $NEXT_TICKET"
!   echo ""
!   
!   # Start working on it
!   echo "🚀 Starting work on ticket..."
!   vibe-ticket start "$NEXT_TICKET"
!   echo ""
!   
!   # Analyze the ticket
!   analyze_ticket "$NEXT_TICKET"
!   
!   # Provide guidance
!   echo "🎯 Next steps:"
!   echo "1. Review the ticket requirements above"
!   echo "2. Check if worktree was created (cd to the directory if needed)"
!   echo "3. Implement the required changes"
!   echo "4. Run /check-ci to verify your changes"
!   echo "5. Use /work-on-ticket next when done"
!   
! elif [ -n "$MODE" ] && [ "$MODE" != "continue" ] && [ "$MODE" != "next" ]; then
!   # Search for specific ticket by keyword
!   echo "🔍 Searching for ticket with keyword: $MODE"
!   FOUND_TICKET=$(vibe-ticket search "$MODE" --limit 1 | grep "Slug:" | head -1 | awk '{print $2}')
!   
!   if [ -z "$FOUND_TICKET" ]; then
!     echo "❌ No ticket found matching: $MODE"
!     echo "💡 Try: /ticket list"
!     exit 1
!   fi
!   
!   echo "📌 Found ticket: $FOUND_TICKET"
!   echo ""
!   
!   # Check ticket status
!   STATUS=$(vibe-ticket show "$FOUND_TICKET" | grep "Status:" | awk '{print $2}')
!   
!   if [ "$STATUS" != "todo" ] && [ "$STATUS" != "doing" ]; then
!     echo "⚠️  Ticket is in status: $STATUS"
!     echo "Continue anyway? (The ticket might be done or blocked)"
!   fi
!   
!   # Start working on it
!   echo "🚀 Starting work on ticket..."
!   vibe-ticket start "$FOUND_TICKET"
!   echo ""
!   
!   # Analyze the ticket
!   analyze_ticket "$FOUND_TICKET"
! fi

! echo ""
! echo "💡 Pro tips:"
! echo "- Use @<filename> to reference files in your response"
! echo "- Use /check-ci quick to quickly verify formatting"
! echo "- Tasks can be managed with: vibe-ticket task add/complete"
```