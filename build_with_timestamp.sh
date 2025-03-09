#!/bin/bash

# Set error handling
set -e

# Get current datetime in a filename-friendly format
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

# Compile the app in release mode
echo "Compiling Brush in release mode..."
cargo build --release

# Determine the executable path based on the platform
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    EXECUTABLE="target/release/brush_app"
    EXTENSION=""
elif [[ "$OSTYPE" == "msys"* ]] || [[ "$OSTYPE" == "win"* ]]; then
    # Windows
    EXECUTABLE="target/release/brush_app.exe"
    EXTENSION=".exe"
else
    # Linux and others
    EXECUTABLE="target/release/brush_app"
    EXTENSION=""
fi

# Create the destination filename with timestamp
DEST_FILENAME="builds/brush_app_${TIMESTAMP}${EXTENSION}"

# Copy the executable to the builds directory with the timestamped name
echo "Copying executable to ${DEST_FILENAME}..."
cp "$EXECUTABLE" "$DEST_FILENAME"

# Make the timestamped executable executable (for Unix-like systems)
if [[ "$OSTYPE" != "msys"* ]] && [[ "$OSTYPE" != "win"* ]]; then
    chmod +x "$DEST_FILENAME"
fi

echo "Build completed successfully!"
echo "Executable saved to: ${DEST_FILENAME}" 