#!/bin/bash

# move-all-to-project.sh - Move all files from novelenv subdirectories to project

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Usage function
usage() {
    echo "Usage: $0 <target-project-path> [directories...]"
    echo "Example: $0 /path/to/my-novel character_profile episode"
    echo ""
    echo "If no directories specified, defaults to: character_profile episode scene_sketch summary official"
    exit 1
}

# Check arguments
if [ $# -lt 1 ]; then
    usage
fi

TARGET_PROJECT="$1"
shift

# Default directories if none specified
if [ $# -eq 0 ]; then
    DIRS=(character_profile episode scene_sketch summary official)
else
    DIRS=("$@")
fi

# Get the directory of this script (novelenv root)
NOVELENV_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MOVE_SCRIPT="$NOVELENV_ROOT/move-to-project.sh"

# Check if target project exists
if [ ! -d "$TARGET_PROJECT" ]; then
    echo -e "${RED}Error: Target project directory does not exist: $TARGET_PROJECT${NC}"
    exit 1
fi

echo -e "${BLUE}Moving files from novelenv to: $TARGET_PROJECT${NC}"
echo -e "${BLUE}Processing directories: ${DIRS[*]}${NC}"
echo ""

# Counter for moved files
MOVED_COUNT=0
SKIPPED_COUNT=0

# Process each directory
for DIR in "${DIRS[@]}"; do
    SOURCE_DIR="$NOVELENV_ROOT/$DIR"
    
    if [ ! -d "$SOURCE_DIR" ]; then
        echo -e "${YELLOW}Skipping non-existent directory: $DIR${NC}"
        continue
    fi
    
    echo -e "${GREEN}Processing $DIR/:${NC}"
    
    # Find all .md files in the directory
    while IFS= read -r -d '' file; do
        # Get relative path from novelenv root
        REL_PATH="${file#$NOVELENV_ROOT/}"
        
        # Skip if it's already a symlink
        if [ -L "$file" ]; then
            echo -e "${YELLOW}  Skipping symlink: $REL_PATH${NC}"
            ((SKIPPED_COUNT++))
            continue
        fi
        
        echo -e "${BLUE}  Moving: $REL_PATH${NC}"
        
        # Use the move-to-project.sh script (run in subshell to prevent exit from affecting parent)
        if ( "$MOVE_SCRIPT" "$REL_PATH" "$TARGET_PROJECT" ); then
            ((MOVED_COUNT++))
        else
            echo -e "${RED}  Failed to move: $REL_PATH${NC}"
        fi
        
        echo ""
    done < <(find "$SOURCE_DIR" -name "*.md" -type f -print0 2>/dev/null)
done

echo -e "${GREEN}Summary:${NC}"
echo -e "  Files moved: $MOVED_COUNT"
echo -e "  Files skipped (already symlinks): $SKIPPED_COUNT"
echo ""
echo -e "${GREEN}✓ Operation complete!${NC}"