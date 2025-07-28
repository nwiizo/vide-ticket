# /ticket-status

Quick overview of ticket status and recommendations.

## Usage
```
/ticket-status     # Show current status and recommendations
```

## Description
Provides a quick overview of:
- Current active ticket
- Pending tasks
- TODO tickets count
- Next recommended action

## Actions

```bash
! echo "📊 vibe-ticket Status Overview"
! echo "=============================="
! echo ""

! # Current ticket
! echo "🎯 Current Status:"
! CURRENT_OUTPUT=$(vibe-ticket check 2>&1)
! if echo "$CURRENT_OUTPUT" | grep -q "No active ticket"; then
!   echo "  No active ticket"
!   ACTIVE_TICKET=""
! else
!   ACTIVE_TICKET=$(echo "$CURRENT_OUTPUT" | grep "Ticket:" | awk '{print $2}')
!   echo "$CURRENT_OUTPUT" | grep -E "(Ticket:|Title:|Status:|Progress:)" | sed 's/^/  /'
!   
!   # Show tasks if any
!   TASKS=$(vibe-ticket task list 2>&1 | grep -E "^\[.\]" | head -5)
!   if [ -n "$TASKS" ]; then
!     echo ""
!     echo "  📝 Active Tasks:"
!     echo "$TASKS" | sed 's/^/    /'
!   fi
! fi

! echo ""
! echo "📈 Ticket Summary:"

! # Count tickets by status
! TODO_COUNT=$(vibe-ticket list --status todo --limit 100 | grep -c "│.*todo.*│" || echo "0")
! DOING_COUNT=$(vibe-ticket list --status doing --limit 100 | grep -c "│.*doing.*│" || echo "0")
! REVIEW_COUNT=$(vibe-ticket list --status review --limit 100 | grep -c "│.*review.*│" || echo "0")

! echo "  • TODO: $TODO_COUNT tickets"
! echo "  • DOING: $DOING_COUNT tickets"
! echo "  • REVIEW: $REVIEW_COUNT tickets"

! # High priority tickets
! HIGH_PRIORITY=$(vibe-ticket list --status todo --priority high --limit 5 | grep "│.*high.*│" | wc -l)
! if [ "$HIGH_PRIORITY" -gt 0 ]; then
!   echo "  • 🔥 High Priority: $HIGH_PRIORITY tickets"
! fi

! echo ""
! echo "💡 Recommendations:"

! if [ -n "$ACTIVE_TICKET" ]; then
!   # Has active ticket
!   echo "  1. Continue working on: $ACTIVE_TICKET"
!   echo "     → Use: /work-on-ticket continue"
!   echo ""
!   echo "  2. Or complete it and move to next:"
!   echo "     → Use: /work-on-ticket next"
! else
!   # No active ticket
!   if [ "$TODO_COUNT" -gt 0 ]; then
!     echo "  Start working on the next ticket:"
!     echo "  → Use: /work-on-ticket"
!     
!     if [ "$HIGH_PRIORITY" -gt 0 ]; then
!       echo ""
!       echo "  ⚠️  You have $HIGH_PRIORITY high priority tickets!"
!     fi
!   else
!     echo "  🎉 All tickets completed!"
!     echo "  Create a new ticket with:"
!     echo "  → vibe-ticket new <slug> --title \"<title>\""
!   fi
! fi

! # Show recent tickets for context
! echo ""
! echo "📋 Recent Tickets:"
! vibe-ticket list --limit 5 | grep -E "^│" | grep -v "┌\|└\|ID\|──" | head -5 | sed 's/^/  /'
```