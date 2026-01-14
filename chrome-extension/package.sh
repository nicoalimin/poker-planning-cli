#!/bin/bash

# Chrome Extension Packaging Script
# Creates a zip file ready for Chrome Web Store upload

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Extract version from manifest.json
VERSION=$(grep '"version"' manifest.json | sed 's/.*: "\([^"]*\)".*/\1/')
NAME=$(grep '"name"' manifest.json | sed 's/.*: "\([^"]*\)".*/\1/' | tr ' ' '-' | tr '[:upper:]' '[:lower:]')

OUTPUT_DIR="$SCRIPT_DIR/dist"
ZIP_NAME="${NAME}-v${VERSION}.zip"

echo "📦 Packaging Chrome Extension"
echo "   Name: $NAME"
echo "   Version: $VERSION"
echo ""

# Create dist directory if it doesn't exist
mkdir -p "$OUTPUT_DIR"

# Remove old zip if exists
rm -f "$OUTPUT_DIR/$ZIP_NAME"

# Files to include in the package (exclude README and this script)
FILES=(
    "manifest.json"
    "background.js"
    "content.js"
)

# Icon files in icons/ folder
ICON_FILES=(
    "icons/icon16.png"
    "icons/icon48.png"
    "icons/icon128.png"
)

# Validate required files exist
echo "✓ Validating required files..."
for file in "${FILES[@]}"; do
    if [ ! -f "$file" ]; then
        echo "  ✗ Missing required file: $file"
        exit 1
    fi
    echo "  ✓ $file"
done

for file in "${ICON_FILES[@]}"; do
    if [ ! -f "$file" ]; then
        echo "  ✗ Missing required file: $file"
        exit 1
    fi
    echo "  ✓ $file"
done

# Create the zip file
echo ""
echo "📁 Creating zip archive..."
zip -r "$OUTPUT_DIR/$ZIP_NAME" "${FILES[@]}" "${ICON_FILES[@]}"

# Show result
ZIP_SIZE=$(du -h "$OUTPUT_DIR/$ZIP_NAME" | cut -f1)
echo ""
echo "✅ Package created successfully!"
echo ""
echo "   Output: $OUTPUT_DIR/$ZIP_NAME"
echo "   Size:   $ZIP_SIZE"
echo ""
echo "📋 Next steps to publish:"
echo "   1. Go to https://chrome.google.com/webstore/devconsole"
echo "   2. Click 'New Item' and upload: $ZIP_NAME"
echo "   3. Fill in the store listing details"
echo "   4. Add screenshots (1280x800 or 640x400)"
echo "   5. Submit for review"
