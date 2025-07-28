# /check-ci

Run all CI checks locally to ensure the code is ready for continuous integration.

## Usage
```
/check-ci         # Run all checks
/check-ci fix     # Run checks and auto-fix issues where possible
/check-ci quick   # Run only essential checks (format & clippy)
```

## Description
This command runs the same checks that CI runs, helping you catch issues before pushing.

## Actions

```bash
! echo "🚀 Starting CI checks..."
! echo ""

# Handle arguments
! if [ "$ARGUMENTS" = "fix" ]; then
!   echo "🔧 Running in fix mode - will auto-fix issues where possible"
!   echo ""
!   
!   echo "📝 Applying code formatting..."
!   cargo fmt
!   echo "✅ Formatting applied"
!   echo ""
!   
!   echo "🔧 Applying clippy fixes..."
!   cargo clippy --fix --allow-dirty -- -D warnings 2>/dev/null || true
!   echo "✅ Clippy fixes applied"
!   echo ""
! fi

# Format check
! echo "1️⃣ Checking code formatting..."
! if cargo fmt -- --check 2>&1 | grep -q "Diff"; then
!   echo "❌ Formatting issues found. Run with 'fix' argument to auto-fix."
!   ERROR=1
! else
!   echo "✅ Formatting OK"
! fi
! echo ""

# Clippy check
! echo "2️⃣ Running clippy linter..."
! if cargo clippy -- -D warnings 2>&1 | grep -E "(error:|warning:)" > /dev/null; then
!   echo "❌ Clippy warnings found. Run with 'fix' argument to auto-fix some issues."
!   ERROR=1
! else
!   echo "✅ Clippy OK"
! fi
! echo ""

# Quick mode exits here
! if [ "$ARGUMENTS" = "quick" ]; then
!   echo "✅ Quick checks completed!"
!   exit ${ERROR:-0}
! fi

# Run tests
! echo "3️⃣ Running all tests..."
! if cargo test --quiet 2>&1 | grep -E "(FAILED|error:)" > /dev/null; then
!   echo "❌ Some tests failed"
!   ERROR=1
! else
!   echo "✅ Tests passed"
! fi
! echo ""

# Test without default features
! echo "4️⃣ Running tests without default features..."
! if cargo test --no-default-features --quiet 2>&1 | grep -E "(FAILED|error:)" > /dev/null; then
!   echo "❌ Some tests failed without default features"
!   ERROR=1
! else
!   echo "✅ Tests without default features passed"
! fi
! echo ""

# Build documentation
! echo "5️⃣ Building documentation..."
! if cargo doc --no-deps --quiet 2>&1 | grep -E "(error:|warning:)" > /dev/null; then
!   echo "⚠️  Documentation has warnings"
! else
!   echo "✅ Documentation OK"
! fi
! echo ""

# Summary
! echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
! if [ -n "$ERROR" ]; then
!   echo "❌ Some checks failed. Please fix the issues above."
!   exit 1
! else
!   echo "✅ All CI checks passed! Your code is ready for CI."
! fi
```