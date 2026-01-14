#!/bin/bash

# Icon Resize Script for Chrome Extension
# Creates properly sized icons from a source image

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

SOURCE="icon.png"
SIZES=(16 48 128)

if [ ! -f "$SOURCE" ]; then
    echo "❌ Source file '$SOURCE' not found"
    exit 1
fi

echo "🖼️  Processing $SOURCE"

# Check current dimensions
if command -v sips &> /dev/null; then
    CURRENT_SIZE=$(sips -g pixelWidth "$SOURCE" | tail -1 | awk '{print $2}')
    echo "   Current size: ${CURRENT_SIZE}x${CURRENT_SIZE}"
fi

# Create icons directory for multiple sizes
mkdir -p icons

for SIZE in "${SIZES[@]}"; do
    OUTPUT="icons/icon${SIZE}.png"
    echo "   Creating ${SIZE}x${SIZE} → $OUTPUT"
    
    if command -v sips &> /dev/null; then
        # macOS: use sips (built-in)
        cp "$SOURCE" "$OUTPUT"
        sips -z $SIZE $SIZE "$OUTPUT" --out "$OUTPUT" > /dev/null 2>&1
    elif command -v convert &> /dev/null; then
        # Linux/other: use ImageMagick
        convert "$SOURCE" -resize ${SIZE}x${SIZE} "$OUTPUT"
    else
        echo "❌ No image processing tool found (need sips or ImageMagick)"
        exit 1
    fi
done

# Also create the main 128x128 icon.png
echo "   Creating 128x128 → icon.png (main)"
if command -v sips &> /dev/null; then
    sips -z 128 128 "$SOURCE" --out "$SOURCE" > /dev/null 2>&1
elif command -v convert &> /dev/null; then
    convert "$SOURCE" -resize 128x128 "$SOURCE"
fi

echo ""
echo "✅ Icons created successfully!"
echo ""
echo "📁 Generated files:"
for SIZE in "${SIZES[@]}"; do
    echo "   icons/icon${SIZE}.png (${SIZE}x${SIZE})"
done
echo "   icon.png (128x128 - main icon)"
echo ""
echo "💡 To use separate icon files, update manifest.json:"
echo '   "icons": {'
echo '     "16": "icons/icon16.png",'
echo '     "48": "icons/icon48.png",'
echo '     "128": "icons/icon128.png"'
echo '   }'
