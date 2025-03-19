#!/bin/bash

# Debug launcher script for Brush development

# Stop existing servers
echo "Stopping existing servers..."
pkill -f "trunk serve" || true
pkill -f "browser-tools-server" || true

# Determine host OS
OPEN_CMD="xdg-open"
if [[ "$OSTYPE" == "darwin"* ]]; then
    OPEN_CMD="open"
fi

# Ensure test data directory exists
echo "Setting up test environment..."
mkdir -p test_data

# Check if we have a sample PLY file
if [ ! -f "test_data/sample.ply" ]; then
    echo "No sample PLY file found. Looking for a PLY file to copy..."
    
    # Look for any PLY files in the project
    SAMPLE_PLY=$(find . -name "*.ply" -not -path "./target/*" | head -n 1)
    
    if [ -z "$SAMPLE_PLY" ]; then
        echo "No PLY file found in the project. Creating a dummy PLY file..."
        echo "ply
format ascii 1.0
element vertex 3
property float x
property float y
property float z
element face 1
property list uchar int vertex_indices
end_header
0 0 0
1 0 0
0 1 0
3 0 1 2" > test_data/sample.ply
    else
        echo "Found PLY file at $SAMPLE_PLY. Copying to test_data/sample.ply..."
        cp "$SAMPLE_PLY" test_data/sample.ply
    fi
fi

# Start MCP server
echo "Starting MCP server..."
npx @agentdeskai/browser-tools-server@1.2.0 --port 3025 > mcp_log.txt 2>&1 &
MCP_PID=$!
echo "MCP server started with PID $MCP_PID"

# Start Trunk server
echo "Starting Trunk server..."
trunk serve --no-autoreload --open=false > trunk_log.txt 2>&1 &
TRUNK_PID=$!
echo "Trunk server started with PID $TRUNK_PID"

# Wait for servers to be ready
echo "Waiting for servers to start..."
sleep 3

# Launch debug page
echo "Opening debug launcher..."
$OPEN_CMD public/debug.html

echo "Debug environment ready."
echo "Press Ctrl+C to stop all servers and clean up."

# Cleanup function
function cleanup {
    echo "Shutting down servers..."
    kill $MCP_PID $TRUNK_PID 2>/dev/null
    echo "Servers stopped."
    exit 0
}

# Register cleanup handler
trap cleanup INT TERM

# Keep script running until interrupted
echo "Streaming logs:"
echo "----------------"
tail -f trunk_log.txt mcp_log.txt

# This point should only be reached if something goes wrong with tail
cleanup 