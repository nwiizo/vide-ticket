#!/usr/bin/env bash
# Build release binaries for vide-ticket

set -euo pipefail

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
VERSION=$(grep '^version' "$PROJECT_ROOT/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')

echo "Building vide-ticket v$VERSION release binaries..."

# Create release directory
RELEASE_DIR="$PROJECT_ROOT/release"
mkdir -p "$RELEASE_DIR"

# Build for current platform
echo "Building for current platform..."
cd "$PROJECT_ROOT"
cargo build --release

# Get platform info
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map architecture names
case "$ARCH" in
    x86_64)
        ARCH="x86_64"
        ;;
    arm64|aarch64)
        ARCH="aarch64"
        ;;
    *)
        echo "Unknown architecture: $ARCH"
        exit 1
        ;;
esac

# Binary name
BINARY_NAME="vide-ticket-v$VERSION-$PLATFORM-$ARCH"

# Copy and strip binary
echo "Creating release binary: $BINARY_NAME"
cp "target/release/vide-ticket" "$RELEASE_DIR/$BINARY_NAME"
strip "$RELEASE_DIR/$BINARY_NAME" 2>/dev/null || true

# Create tarball
echo "Creating tarball..."
cd "$RELEASE_DIR"
tar -czf "$BINARY_NAME.tar.gz" "$BINARY_NAME"
rm "$BINARY_NAME"

# Calculate checksums
echo "Calculating checksums..."
if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$BINARY_NAME.tar.gz" > "$BINARY_NAME.tar.gz.sha256"
else
    shasum -a 256 "$BINARY_NAME.tar.gz" > "$BINARY_NAME.tar.gz.sha256"
fi

echo "Release build complete!"
echo "Output files:"
echo "  - $RELEASE_DIR/$BINARY_NAME.tar.gz"
echo "  - $RELEASE_DIR/$BINARY_NAME.tar.gz.sha256"

# Display file sizes
ls -lh "$RELEASE_DIR"/*.tar.gz*