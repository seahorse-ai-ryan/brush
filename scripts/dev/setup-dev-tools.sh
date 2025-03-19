#!/bin/bash

# Setup script for Brush development tools

echo "Setting up development tools for Brush..."

# Check if npm is installed
if ! command -v npm &> /dev/null; then
    echo "Error: npm is not installed. Please install Node.js and npm first."
    echo "Visit https://nodejs.org/ for installation instructions."
    exit 1
fi

# Install development dependencies
echo "Installing Node.js dependencies for development tools..."
npm install

echo "Setup complete! You can now run 'npm run start-mcp' to start the BrowserTools MCP server."

echo "Note: These tools are optional and only needed for web debugging." 