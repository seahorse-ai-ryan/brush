# Web Development Debugging Guide

> **Note**: This document captures the current debugging process for Brush web development.

## Setting up BrowserTools MCP

For a complete setup guide, see [Browser Tools Setup](./browser_tools_setup.md).

### Recommended Terminal Setup
For the optimal debugging experience in Cursor:

1. **BrowserTools MCP**: Run in a dedicated terminal
   ```bash
   npx @agentdeskai/browser-tools-server@1.2.0 --port 3025
   ```
   - This keeps the server running in the background
   - Allows you to check connection status when needed
   - Provides visibility into console logs and errors

2. **Trunk Server**: Run in a dedicated terminal
   ```bash
   trunk serve --no-autoreload --open=false
   ```
   - Keeps the build process separate from the chat
   - Prevents chat hanging issues when errors occur
   - Allows you to monitor build output independently

This setup provides the best balance between monitoring different outputs while maintaining stability in the Cursor chat interface.

## AI-Assisted Debugging Workflow

For effective AI-assisted debugging, follow this structured approach:

### 1. Bug Identification Phase

Before attempting any fixes, the AI agent should:

1. **Analyze Error Patterns**:
   - Examine compilation errors, runtime logs, and console output
   - Group related errors and identify the root cause
   - Determine if the issue is platform-specific (native vs. WASM)

2. **Form a Hypothesis**:
   - Create a clear statement describing what's causing the bug
   - Identify the specific code modules/components involved
   - Document any environmental factors (browser type, device, etc.)

3. **Document Expected vs. Actual Behavior**:
   - Describe what should happen when the application works correctly
   - Detail what's currently happening instead
   - Note any error messages or unexpected behavior

4. **Propose Solution Strategy**:
   - Outline a specific approach to fixing the issue
   - List the files that will need modification
   - Highlight potential side effects or risks of the proposed changes

Example bug identification:
```
Bug: WASM filesystem operations causing application crash
Root cause: Native filesystem API calls being executed in WASM environment
Files affected: dataset_detail.rs, app.rs
Solution approach: Add platform-specific code paths using #[cfg(target_arch = "wasm32")] directives
```

### 2. Automated Testing with Reload

After implementing fixes, the AI agent should:

1. **Trigger Application Reload**:
   ```bash
   # Refresh the application in Chrome
   cd /Users/ryanhickman/code/brush && curl -X POST http://localhost:8080/_trunk/reload
   ```

2. **Monitor Console Output**:
   - Watch the MCP server logs for new errors or warnings
   - Verify that the previous error pattern is no longer occurring
   - Check for any new issues that might have been introduced

3. **Perform Regression Testing**:
   - Validate that the original functionality still works
   - Test related features that might be affected by the changes
   - Ensure cross-platform compatibility (if applicable)

### 3. Bug Resolution Documentation

After resolving the issue, the AI should document:

1. **Root Cause Analysis**:
   - What caused the bug (detailed explanation)
   - How the fix addresses the root cause
   - Any related issues or technical debt discovered

2. **Implementation Notes**:
   - Code changes made (files, functions, patterns)
   - Testing performed to verify the fix
   - Performance implications (if any)

3. **Future Prevention Strategies**:
   - Patterns to avoid in future development
   - Monitoring recommendations
   - Test cases to add

## Console Monitoring
The BrowserTools MCP server captures console logs and errors from the browser. To effectively monitor console output:

1. Keep the BrowserTools MCP terminal visible
2. Watch for lines starting with `=== Received Extension Log ===`
3. Look for entries with `dataType: 'console-error'` or `dataType: 'console-log'`
4. Error messages will include a timestamp and complete error details

## Common Web Errors
1. **"No filesystem on this platform" errors**
   - This is expected in WASM environments where filesystem access is restricted
   - Implement platform-specific code with `#[cfg(target_arch = "wasm32")]` blocks
   - Provide appropriate error handling for web users

2. **"Integrity attribute" warnings**
   - These warnings are related to Subresource Integrity (SRI) checks
   - A script has been added to index.html to automatically disable these checks during development
   - They don't affect application functionality

3. **"IndexedDB storage not implemented" errors**
   - These are expected in certain build configurations
   - The application will continue to function without IndexedDB storage

4. **Port conflicts**
   - If you see "Address already in use" errors, a server is already running on that port
   - Check for existing processes and terminate them before starting a new server
   ```bash
   # Check for processes using a port
   lsof -i :3025  # Check if port 3025 is in use
   lsof -i :8080  # Check if port 8080 is in use
   
   # Kill process if needed
   kill -9 <PID>
   ```

## Troubleshooting
- If the BrowserTools MCP server fails to start, check if it's already running on port 3025
- If console messages aren't appearing in the server logs, try:
  1. Refreshing the browser page
  2. Verifying the Browser Tools extension is enabled
  3. Restarting both the MCP server and Trunk server
- If you see typos in URLs like `http://localhost.:8080/` (with an extra dot), this can cause connection issues

## Best Practices
1. Always use port 3025 for the MCP server for consistency
2. Start services in the correct order: MCP server → Trunk server → Browser
3. Keep all terminal windows visible for monitoring
4. Restart all services if you encounter connection issues

## Development Process

1. **Start BrowserTools MCP Server**
   ```bash
   npx @agentdeskai/browser-tools-server@1.2.0 --port 3025
   ```
   - Wait for the server to fully start
   - Look for "Browser Tools Server Started" message
   - Verify it's running on port 3025

2. **Start Trunk Server**
   ```bash
   trunk serve --no-autoreload --open=false
   ```
   - Wait for the server to fully start
   - Look for "server listening at" message
   - Check for any compilation errors

3. **Open Browser**
   - Navigate to `http://localhost:8080/`
   - Ensure the BrowserTools extension is enabled
   - Check connection status in the extension

## Useful Commands

### Kill Existing Servers and Start New Ones
```bash
# Kill existing MCP server
pkill -f "browser-tools-server" && echo "Stopping MCP server..."

# Start MCP server
npx @agentdeskai/browser-tools-server@1.2.0 --port 3025

# Kill existing Trunk server
pkill -f "trunk serve" && echo "Stopping Trunk server..."

# Start Trunk server
trunk serve --no-autoreload --open=false
```

### Check Port Usage
```bash
lsof -i :3025  # Check if MCP server port is in use
lsof -i :8080  # Check if Trunk server port is in use
```

### Kill Other Web Servers
```bash
# Kill servers on common ports
lsof -i :8000 -t | xargs kill -9 2>/dev/null || true
lsof -i :3000 -t | xargs kill -9 2>/dev/null || true
lsof -i :5000 -t | xargs kill -9 2>/dev/null || true
```

### Force Browser Reload After Code Changes
```bash
# Trigger browser reload via Trunk's API
curl -X POST http://localhost:8080/_trunk/reload
```

## Testing MCP Connection

To verify the MCP server is properly capturing console logs:

1. Navigate to `http://localhost:8080/` in your browser
2. Open the browser console (F12 or Cmd+Option+J)
3. Type in the console:
   ```javascript
   console.log("Test MCP connection")
   ```
4. Check the MCP server terminal - you should see the message captured in the logs
5. If successful, you'll see details of the log in the format:
   ```
   Adding console log: {
     level: 'log',
     message: 'Test MCP connection',
     timestamp: 1742248820688
   }
   ```

## MCP Features for Debugging

The BrowserTools MCP server provides these key features:
1. Console log capture (info, debug, warning, error)
2. Network request monitoring
3. URL change detection
4. Detailed error reporting

These features are especially valuable when debugging WASM applications where traditional debugging tools may be limited. 