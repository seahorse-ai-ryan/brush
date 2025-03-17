# Web Development Debugging Guide

> **Note**: This is a work-in-progress document capturing our current debugging process. We'll revisit and refine these procedures later.

> **Historical Note**: Our previous approach using command-line Chrome debugging and custom logging scripts proved ineffective and disruptive to workflow. We're now using BrowserTools MCP for better integration with Cursor and more reliable debugging.

## Setting up BrowserTools MCP

### Initial Setup
1. Install the BrowserTools Chrome extension:
   - Download from [GitHub Releases](https://github.com/AgentDeskAI/browser-tools-mcp/releases)
   - Open Chrome and go to `chrome://extensions/`
   - Enable "Developer mode" in the top right
   - Click "Load unpacked" and select the downloaded extension folder

2. Install Node.js dependencies:
   ```bash
   # Run the setup script to install dependencies
   ./setup-dev-tools.sh
   ```
   This will install the necessary Node.js packages including the BrowserTools MCP server.

3. Configure MCP Server:
   Create or edit `~/.cursor/mcp.json`:
   ```json
   {
     "MCP_SERVER_TOKEN": "your_token",
     "MCP_SERVER_URL": "http://localhost:3025"
   }
   ```
   Note: Always use port 3025 as the default. If this port is already in use, it likely means the server is already running.

4. Configure Chrome Extension:
   - Open Chrome DevTools (Cmd+Option+I on Mac, F12 on Windows)
   - Look for the "BrowserToolsMCP" tab (might be hidden under >>)
   - Set "Server Host" to `localhost`
   - Set "Server Port" to `3025`
   - Click "Test Connection"
   - You should see "Connected successfully to browser-tools-server"

### Debugging Workflow
1. Start BrowserTools MCP server in a dedicated terminal tab:
   ```bash
   npm run start-mcp
   ```

2. Start Trunk server in a dedicated terminal tab:
   ```bash
   trunk serve --no-autoreload --open=false
   ```

3. Open Chrome with the BrowserTools extension enabled
4. Navigate to `http://localhost:8080/`
5. Open Chrome DevTools and ensure the BrowserToolsMCP tab is configured correctly
6. Monitor the BrowserTools MCP server terminal for console logs and errors

### Recommended Terminal Setup
For the optimal debugging experience in Cursor:

1. **BrowserTools MCP**: Run in a dedicated terminal tab
   - This keeps the server running in the background
   - Allows you to check connection status when needed
   - Provides visibility into console logs and errors

2. **Trunk Server**: Run in a dedicated terminal tab (not inline)
   - Run with `trunk serve --no-autoreload --open=false`
   - Keeps the build process separate from the chat
   - Prevents chat hanging issues when errors occur
   - Allows you to monitor build output independently

This setup provides the best balance between monitoring different outputs while maintaining stability in the Cursor chat interface.

### Console Monitoring
While BrowserTools MCP captures console logs and errors, they are primarily visible in the BrowserTools MCP server terminal. To effectively monitor console output:

1. Keep the BrowserTools MCP terminal visible
2. Watch for lines starting with `=== Received Extension Log ===`
3. Look for entries with `dataType: 'console-error'` or `dataType: 'console-log'`
4. For critical errors, manually share relevant portions with the AI

### Common Web Errors
1. **"No filesystem on this platform" errors**
   - This is expected in WASM environments where filesystem access is restricted
   - Implement platform-specific code with `#[cfg(target_arch = "wasm32")]` blocks
   - Provide appropriate error messages for web users

2. **"Integrity attribute" warnings**
   - These warnings are related to preload destinations and can be safely ignored
   - They don't affect application functionality

3. **Port conflicts**
   - If you see "Address already in use" errors, a server is already running on that port
   - Check for existing processes and terminate them before starting a new server
   ```bash
   # Check for processes using a port
   lsof -i :3025  # Check if port 3025 is in use
   lsof -i :8080  # Check if port 8080 is in use
   
   # Kill process if needed
   kill -9 <PID>
   ```

### Troubleshooting
- If the BrowserTools MCP server fails to start, check if it's already running on port 3025
- The extension UI might show "Not connected" even when working - check the server logs to confirm connection
- If you see "no filesystem on this platform" errors, these are expected in the WASM environment
- If console messages aren't appearing in the server logs, try restarting the BrowserTools MCP server

### Best Practices
1. Always use port 3025 for consistency
2. Start services in the correct order: MCP server → Trunk
3. Keep all terminal windows visible for monitoring
4. Restart all services if you encounter connection issues

## Development Process

1. **Start BrowserTools MCP Server**
   ```bash
   npm run start-mcp
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

### Start MCP Server
```bash
npm run start-mcp
```

### Start Trunk Server
```bash
trunk serve --no-autoreload --open=false
```

### Check Port Usage
```bash
lsof -i :3025  # Check if MCP server port is in use
lsof -i :8080  # Check if Trunk server port is in use
```

### Kill Processes
```bash
pkill -f "trunk serve"  # Kill Trunk server
pkill -f "browser-tools-server"  # Kill MCP server
```

## Initial Setup

1. **Configure BrowserTools MCP in Cursor**
   - Open Cursor Settings
   - Navigate to "Cursor Settings > MCP"
   - Click "Add New MCP Server"
   - Select "BrowserTools MCP" from available tools
   - Follow the setup wizard to complete installation

2. **Install Chrome Extension**
   - The BrowserTools MCP extension will be installed automatically
   - Verify it appears in Chrome's extension list
   - Pin it to the toolbar for easy access

## Chrome Profile Setup
1. **Create Debug Profile** (One-time setup)
   - Open Chrome
   - Click profile icon in top-right
   - Select "Add new profile"
   - Name it "Brush Debug"
   - Choose a distinct avatar
   - Do not sync with Google account (keeps it clean for debugging)

2. **Configure Debug Profile** (One-time setup)
   - Open Chrome with Brush Debug profile
   - Configure any necessary developer settings
   - Enable the BrowserTools MCP extension for this profile
   - This profile will be used exclusively for Brush debugging

## Common Issues

### Permission Errors
If you see Crashpad settings.dat permission errors:
```
[ERROR:file_io_posix.cc(208)] open /Users/.../Chrome/Crashpad/settings.dat: Permission denied (13)
```
This is expected and doesn't affect functionality.

### GPU Process Errors
If you see GPU process errors:
```
[ERROR:gpu_process_host.cc(953)] GPU process exited unexpectedly: exit_code=15
```
These may affect WebGPU functionality but not basic app operation.

### Profile Issues
If BrowserTools MCP can't find the debug profile:
1. Verify the profile name matches exactly
2. Check if the profile directory exists
3. Try recreating the debug profile

### Connection Issues
If BrowserTools MCP can't connect:
1. Check if the Chrome extension is enabled
2. Verify Cursor's MCP settings
3. Try restarting Cursor

## Debugging Tips

1. **Process Order**
   - Always start Trunk first
   - Wait for Trunk to fully initialize
   - Use Cursor's Command Palette to start debugging

2. **Monitor Output**
   - Watch Trunk's compilation output
   - Use Cursor's integrated console view
   - Monitor Chrome's DevTools if needed

3. **Profile Management**
   - Keep the debug profile separate from daily browsing
   - Don't sync the debug profile with Google account
   - Clear profile data if you encounter persistent issues

## BrowserTools MCP Features

The BrowserTools MCP integration provides:
- Real-time console monitoring in Cursor
- WASM/Runtime error capture
- Network request monitoring
- Performance profiling
- Chrome profile management
- Screenshot capture
- DOM element inspection

## Useful Commands
All commands are accessed through Cursor's Command Palette (Cmd/Ctrl + Shift + P):

### Start Debugging
```
BrowserTools: Start Debug Session
```

### Capture Screenshot
```
BrowserTools: Capture Screenshot
```

### View Console Logs
```
BrowserTools: Show Console
```

### Inspect Element
```
BrowserTools: Inspect Element
``` 