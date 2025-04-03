#!/bin/bash

PHERO_DIR="/home/matheus/STG/pherohormones"
INPUT_DIR="$PHERO_DIR/frames"
OUTPUT_DIR="$PHERO_DIR/rendered_frames"
OUTPUT_GIF="$PHERO_DIR/animation.gif"

# Create directories if they don't exist
mkdir -p "$INPUT_DIR" "$OUTPUT_DIR"

# Convert DOT files to PNG
for file in "$INPUT_DIR"/*.dot; do
    frame_number=$(basename "$file" .dot | cut -d'_' -f2)
    dot -Tpng "$file" -o "$OUTPUT_DIR/frame_${frame_number}.png"
done

# Create GIF (50ms delay between frames)
convert -delay 50 -loop 0 "$OUTPUT_DIR"/*.png "$OUTPUT_GIF"

echo "Animation created at: $OUTPUT_GIF"