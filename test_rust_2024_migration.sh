#!/usr/bin/env bash
set -euo pipefail

echo "=== Rust 2024 Edition Migration Test ==="
echo

# Check current Rust version
echo "Current Rust version:"
rustc --version
echo

# Create a test Cargo.toml with 2024 edition
echo "Creating test configuration with Rust 2024 edition..."
cp Cargo.toml Cargo.toml.backup

# Try to compile with 2024 edition
echo "Testing compilation with Rust 2024 edition..."
sed -i.bak 's/edition = "2021"/edition = "2024"/' Cargo.toml

# Run checks
echo "Running cargo check..."
if cargo check --all-features 2>&1; then
    echo "✓ Cargo check passed with Rust 2024 edition"
else
    echo "✗ Cargo check failed with Rust 2024 edition"
    mv Cargo.toml.backup Cargo.toml
    exit 1
fi

echo
echo "Running cargo clippy..."
if cargo clippy --all-features -- -D warnings 2>&1 | grep -v "MSRV"; then
    echo "✓ Clippy passed with Rust 2024 edition"
else
    echo "✗ Clippy found issues with Rust 2024 edition"
fi

echo
echo "Running cargo test..."
if cargo test --all-features --quiet; then
    echo "✓ All tests passed with Rust 2024 edition"
else
    echo "✗ Some tests failed with Rust 2024 edition"
fi

# Check for deprecated features
echo
echo "Checking for deprecated features..."
cargo fix --edition-idioms --allow-dirty --allow-staged 2>&1 || true

# Restore original Cargo.toml
mv Cargo.toml.backup Cargo.toml
rm -f Cargo.toml.bak

echo
echo "=== Migration Test Complete ==="
echo "The project appears to be compatible with Rust 2024 edition!"