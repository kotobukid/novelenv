#!/bin/bash

# move-to-project.sh - Move files from novelenv to project and create symlinks

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Usage function
usage() {
    echo "Usage: $0 <source-file> <target-project-path>"
    echo "Example: $0 character_profile/alice.md /path/to/my-novel"
    echo ""
    echo "This script will:"
    echo "  1. Move the file from novelenv to the target project"
    echo "  2. Create a symlink in novelenv pointing to the moved file"
    exit 1
}

# Check arguments
if [ $# -ne 2 ]; then
    usage
fi

SOURCE_FILE="$1"
TARGET_PROJECT="$2"

# Get the directory of this script (novelenv root)
NOVELENV_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Construct full source path
FULL_SOURCE="$NOVELENV_ROOT/$SOURCE_FILE"

# Check if source file exists
if [ ! -f "$FULL_SOURCE" ]; then
    echo -e "${RED}Error: Source file does not exist: $FULL_SOURCE${NC}"
    exit 1
fi

# Check if target project exists
if [ ! -d "$TARGET_PROJECT" ]; then
    echo -e "${RED}Error: Target project directory does not exist: $TARGET_PROJECT${NC}"
    exit 1
fi

# Extract the subdirectory structure (e.g., character_profile from character_profile/alice.md)
SUBDIR=$(dirname "$SOURCE_FILE")
FILENAME=$(basename "$SOURCE_FILE")

# Create target directory if it doesn't exist
TARGET_DIR="$TARGET_PROJECT/$SUBDIR"
if [ ! -d "$TARGET_DIR" ]; then
    echo -e "${YELLOW}Creating directory: $TARGET_DIR${NC}"
    mkdir -p "$TARGET_DIR"
fi

# Construct full target path
FULL_TARGET="$TARGET_DIR/$FILENAME"

# Check if target already exists
if [ -e "$FULL_TARGET" ]; then
    echo -e "${RED}Error: Target file already exists: $FULL_TARGET${NC}"
    echo "Please resolve this manually."
    exit 1
fi

# Move the file
echo -e "${GREEN}Moving file:${NC}"
echo "  From: $FULL_SOURCE"
echo "  To:   $FULL_TARGET"
mv "$FULL_SOURCE" "$FULL_TARGET"

# Create symlink
echo -e "${GREEN}Creating symlink:${NC}"
echo "  Link: $FULL_SOURCE"
echo "  Target: $FULL_TARGET"
ln -s "$FULL_TARGET" "$FULL_SOURCE"

# Verify the symlink
if [ -L "$FULL_SOURCE" ] && [ -e "$FULL_SOURCE" ]; then
    echo -e "${GREEN}✓ Successfully moved and linked!${NC}"
    ls -la "$FULL_SOURCE"
else
    echo -e "${RED}Error: Symlink creation may have failed${NC}"
    exit 1
fi