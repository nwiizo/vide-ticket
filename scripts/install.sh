#!/usr/bin/env bash
# Install vide-ticket to the system

set -euo pipefail

# Default installation directory
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo -e "${BLUE}Installing vide-ticket...${NC}"

# Check if cargo is available
if ! command -v cargo >/dev/null 2>&1; then
    echo -e "${RED}Error: cargo not found. Please install Rust first.${NC}"
    echo "Visit https://rustup.rs/ for installation instructions."
    exit 1
fi

# Build release binary
echo "Building release binary..."
cd "$PROJECT_ROOT"
cargo build --release

# Create installation directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Copy binary
echo "Installing to $INSTALL_DIR/vide-ticket"
cp target/release/vide-ticket "$INSTALL_DIR/vide-ticket"
chmod +x "$INSTALL_DIR/vide-ticket"

# Check if installation directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${BLUE}Note: $INSTALL_DIR is not in your PATH.${NC}"
    echo "Add the following line to your shell configuration file:"
    echo ""
    echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
    echo ""
    echo "For bash: ~/.bashrc or ~/.bash_profile"
    echo "For zsh:  ~/.zshrc"
fi

# Verify installation
if "$INSTALL_DIR/vide-ticket" --version >/dev/null 2>&1; then
    VERSION=$("$INSTALL_DIR/vide-ticket" --version)
    echo -e "${GREEN}✓ Successfully installed $VERSION to $INSTALL_DIR${NC}"
else
    echo -e "${RED}✗ Installation verification failed${NC}"
    exit 1
fi

echo -e "${GREEN}Installation complete!${NC}"
echo ""
echo "To get started, run:"
echo "  vide-ticket init"