#!/bin/bash

# Exit script if any command fails
set -e

echo "🔵 Starting Browser Tools MCP Server on port 3025..."
npx @agentdeskai/browser-tools-server@1.2.0 --port 3025 