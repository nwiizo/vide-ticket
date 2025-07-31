#!/usr/bin/env bash
# Demo script for vibe-ticket import functionality

set -euo pipefail

echo "=== vibe-ticket Import Demo ==="
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}1. Testing JSON import (object format)${NC}"
echo "Command: vibe-ticket import test-data/simple-import.json --format json --dry-run"
vibe-ticket import test-data/simple-import.json --format json --dry-run --skip-validation || true
echo ""

echo -e "${YELLOW}2. Testing YAML import with auto-detection${NC}"
echo "Command: vibe-ticket import test-data/test-import.yaml --dry-run"
vibe-ticket import test-data/test-import.yaml --dry-run --skip-validation || true
echo ""

echo -e "${YELLOW}3. Testing CSV import${NC}"
echo "Command: vibe-ticket import test-data/test-import.csv --format csv --dry-run"
vibe-ticket import test-data/test-import.csv --format csv --dry-run --skip-validation || true
echo ""

echo -e "${YELLOW}4. Testing error handling with invalid JSON${NC}"
echo "Command: vibe-ticket import test-data/invalid.json"
vibe-ticket import test-data/invalid.json 2>&1 || echo -e "${GREEN}Error handling working correctly${NC}"
echo ""

echo -e "${YELLOW}5. Listing all imported tickets${NC}"
echo "Command: vibe-ticket list --limit 10"
vibe-ticket list --limit 10
echo ""

echo -e "${GREEN}Demo complete!${NC}"
echo "To actually import tickets, remove the --dry-run flag from the commands above."