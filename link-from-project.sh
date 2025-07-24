#!/bin/bash

# link-from-project.sh - Create symlinks in novelenv pointing to project files

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Usage function
usage() {
    echo "Usage: $0 <project-file> [novelenv-path]"
    echo "Example: $0 /path/to/my-novel/character_profile/alice.md"
    echo "         $0 /path/to/my-novel/character_profile/alice.md character_profile/alice_v2.md"
    echo ""
    echo "If novelenv-path is not specified, it will mirror the project structure"
    exit 1
}

# Check arguments
if [ $# -lt 1 ] || [ $# -gt 2 ]; then
    usage
fi

PROJECT_FILE="$1"
NOVELENV_PATH="${2:-}"

# Get the directory of this script (novelenv root)
NOVELENV_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Check if project file exists
if [ ! -f "$PROJECT_FILE" ]; then
    echo -e "${RED}Error: Project file does not exist: $PROJECT_FILE${NC}"
    exit 1
fi

# Get absolute path of project file
PROJECT_FILE="$(cd "$(dirname "$PROJECT_FILE")" && pwd)/$(basename "$PROJECT_FILE")"

# If novelenv path not specified, try to extract from project structure
if [ -z "$NOVELENV_PATH" ]; then
    # Try to find common subdirectories in the path
    for SUBDIR in character_profile episode scene_sketch summary official writing_style; do
        if [[ "$PROJECT_FILE" == *"/$SUBDIR/"* ]]; then
            # Extract the part after the subdirectory
            NOVELENV_PATH="${PROJECT_FILE#*/$SUBDIR/}"
            NOVELENV_PATH="$SUBDIR/$NOVELENV_PATH"
            break
        fi
    done
    
    if [ -z "$NOVELENV_PATH" ]; then
        echo -e "${RED}Error: Could not determine novelenv path automatically.${NC}"
        echo "Please specify the target path explicitly."
        exit 1
    fi
fi

# Construct full novelenv path
FULL_NOVELENV_PATH="$NOVELENV_ROOT/$NOVELENV_PATH"

# Check if symlink already exists
if [ -e "$FULL_NOVELENV_PATH" ]; then
    if [ -L "$FULL_NOVELENV_PATH" ]; then
        echo -e "${YELLOW}Warning: Symlink already exists at: $FULL_NOVELENV_PATH${NC}"
        EXISTING_TARGET=$(readlink "$FULL_NOVELENV_PATH")
        echo "  Currently points to: $EXISTING_TARGET"
        
        if [ "$EXISTING_TARGET" = "$PROJECT_FILE" ]; then
            echo -e "${GREEN}✓ Already linked to the correct file${NC}"
            exit 0
        else
            echo -e "${RED}Error: Points to a different file${NC}"
            exit 1
        fi
    else
        echo -e "${RED}Error: Regular file already exists at: $FULL_NOVELENV_PATH${NC}"
        exit 1
    fi
fi

# Create directory if needed
TARGET_DIR=$(dirname "$FULL_NOVELENV_PATH")
if [ ! -d "$TARGET_DIR" ]; then
    echo -e "${YELLOW}Creating directory: $TARGET_DIR${NC}"
    mkdir -p "$TARGET_DIR"
fi

# Create symlink
echo -e "${GREEN}Creating symlink:${NC}"
echo "  Link: $FULL_NOVELENV_PATH"
echo "  Target: $PROJECT_FILE"
ln -s "$PROJECT_FILE" "$FULL_NOVELENV_PATH"

# Verify the symlink
if [ -L "$FULL_NOVELENV_PATH" ] && [ -e "$FULL_NOVELENV_PATH" ]; then
    echo -e "${GREEN}✓ Successfully linked!${NC}"
    ls -la "$FULL_NOVELENV_PATH"
else
    echo -e "${RED}Error: Symlink creation may have failed${NC}"
    exit 1
fi