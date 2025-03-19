# Brush Debugging Guide

This document provides a consolidated guide for debugging Brush applications across different platforms, with special emphasis on web development.

## Contents
- [Setting Up the Debugging Environment](#setting-up-the-debugging-environment)
- [Web Development Debugging](#web-development-debugging)
- [Desktop Development Debugging](#desktop-development-debugging)
- [Common Issues and Solutions](#common-issues-and-solutions)
- [AI-Assisted Debugging](#ai-assisted-debugging)

## Setting Up the Debugging Environment

### Prerequisites
- Rust toolchain with wasm32-unknown-unknown target
- Trunk (for web development)
- Chrome with Browser Tools extension (for web debugging)
- NodeJS (for MCP server)

### Debug Tools Installation
```bash
# Install Rust target for WebAssembly
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk

# Install Browser Tools MCP server
npm install -g @agentdeskai/browser-tools-server
```

## Web Development Debugging

### Standard Debugging Setup

1. **Start the MCP Server** (Browser Console Monitoring)
   ```bash
   cd /Users/ryanhickman/code/brush && npx @agentdeskai/browser-tools-server --port 3025
   ```

2. **Start the Trunk Server**
   ```bash
   cd /Users/ryanhickman/code/brush && trunk serve --no-autoreload --open=false
   ```
   
   If you get "Address already in use" errors:
   ```bash
   pkill -f "trunk serve" || true
   ```
   Then try starting Trunk again.

3. **Open in Browser**
   ```
   http://localhost:8080/
   ```

### Quick Debug Environment
For rapid testing with test data:
```bash
cd /Users/ryanhickman/code/brush && ./debug_launcher.sh
```

### Testing URL Parameters
The application supports various URL parameters for testing:
```
http://localhost:8080/?debug=true
http://localhost:8080/?autoload=true
http://localhost:8080/?test=ply-loading
```

### Console Log Monitoring
The MCP server captures all browser console logs and errors, making them available for debugging.

To verify MCP is working:
1. Open the browser console (F12 or Cmd+Option+J)
2. Type: `console.log("Test MCP connection")`
3. Check the MCP server terminal - you should see the message captured

### Forcing Browser Reload
After making code changes, trigger a browser reload:
```bash
curl -X POST http://localhost:8080/_trunk/reload
```

## Desktop Development Debugging

### Running with Logging
```bash
RUST_LOG=debug cargo run
```

### Debugging with Rerun
For visual debugging with the Rerun tool:
```bash
cargo run --features rerun
```

## Common Issues and Solutions

### Web-Specific Issues

#### "No filesystem on this platform" errors
- This is expected in WASM environments where filesystem access is restricted
- Use `#[cfg(target_arch = "wasm32")]` blocks to provide web-specific implementations
- Use web-compatible APIs for file handling

#### "Address already in use" (Port conflicts)
- Check for processes using required ports:
  ```bash
  lsof -i :3025  # Check MCP server port
  lsof -i :8080  # Check Trunk server port
  ```
- Kill processes if needed:
  ```bash
  kill -9 <PID>
  ```
- Or stop all servers:
  ```bash
  pkill -f "trunk serve" || true
  pkill -f "browser-tools-server" || true
  ```

#### SRI (Subresource Integrity) warnings
- These are handled automatically by a script in index.html
- They don't affect application functionality
- Only appear during development, not in production builds

### Application Crashes

#### WASM memory issues
- Web browsers have memory limitations
- Use chunked processing for large data
- Implement proper cleanup of WebGL resources
- Avoid large allocations in web environment

#### Missing PLY file errors
- Verify file is accessible at expected URL
- Check data-trunk directives in index.html
- Ensure manual file placement in trunk_dist/ directory if needed

## AI-Assisted Debugging

Brush supports AI-assisted debugging with Cursor editor.

### Enabling AI Debugging

1. Install Cursor editor
2. Configure Cursor for Brush development
3. Start the MCP server as described above
4. Have AI start Trunk server inline in chat
5. Use structured debugging workflow (see [ai_assisted_workflow.md](ai_assisted_workflow.md))

### Effective AI Debugging Tips

1. Provide clear error descriptions
2. Share console logs via MCP
3. Explain expected vs. actual behavior
4. Let AI implement incremental changes
5. Verify fixes after each change

## Advanced Debugging Techniques

### Network Request Analysis
The MCP server captures network requests, allowing analysis of API calls and resource loading.

### Memory Profiling
For memory issues, use the Chrome DevTools Memory tab:
1. Open DevTools > Memory tab
2. Take a heap snapshot
3. Look for large objects and leak patterns

### Performance Optimization
Use the Performance tab in Chrome DevTools to identify bottlenecks in rendering and computation.

## Stopping the Development Environment

When finished debugging:
```bash
# Stop all servers
cd /Users/ryanhickman/code/brush && (pkill -f "trunk serve" || true) && (pkill -f "browser-tools-server" || true)
``` 