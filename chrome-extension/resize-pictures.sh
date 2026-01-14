#!/bin/bash

# Usage: ./png_to_jpeg.sh input.png [output.jpg]

INPUT="$1"
OUTPUT="${2:-${INPUT%.*}.jpg}"

if [ -z "$INPUT" ]; then
  echo "Usage: $0 input.png [output.jpg]"
  exit 1
fi

# Resize to exactly 1280x800 and convert to JPEG
sips -z 800 1280 "$INPUT" --setProperty format jpeg --out "$OUTPUT"

echo "Converted $INPUT -> $OUTPUT (1280x800 JPEG)"
