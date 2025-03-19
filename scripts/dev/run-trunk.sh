#!/bin/bash

# Script to reliably run Trunk server

# Kill any existing Trunk processes
echo "Killing any existing Trunk processes..."
pkill -f "trunk serve" || true

# Clean up any lock files
echo "Cleaning up lock files..."
rm -rf target/.rustc_info.json 2>/dev/null || true
find target -name "*.lock" -delete 2>/dev/null || true

# Create logs directory if it doesn't exist
mkdir -p logs

# Start Trunk in the background and redirect output to a log file
echo "Starting Trunk server..."
trunk serve --no-autoreload --open=false > logs/trunk.log 2>&1 &
TRUNK_PID=$!

# Wait a moment to see if it starts successfully
sleep 3

# Check if the process is still running
if ps -p $TRUNK_PID > /dev/null; then
    echo "Trunk server started successfully with PID $TRUNK_PID"
    echo "Server is running at http://localhost:8080/"
    echo "Log file: $(pwd)/logs/trunk.log"
    echo "To view logs in real-time: tail -f logs/trunk.log"
    echo "To stop the server: kill $TRUNK_PID"
else
    echo "Trunk server failed to start. Check logs/trunk.log for details."
    tail -n 20 logs/trunk.log
fi 