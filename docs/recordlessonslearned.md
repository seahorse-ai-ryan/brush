# Lessons Learned from Brush Development

## Web Debugging Lessons

### Problem: Inconsistent Port Usage
- **Issue**: Using different ports for MCP server in different sessions caused confusion
- **Solution**: Standardized on port 3025 for MCP server and documented in debugging guide
- **Code Example**:
  ```json
  // ~/.cursor/mcp.json
  {
    "MCP_SERVER_TOKEN": "your_token",
    "MCP_SERVER_URL": "http://localhost:3025"
  }
  ```

### Problem: Difficulty Tracking Console Errors
- **Issue**: Console errors were hard to track without proper monitoring
- **Solution**: Monitor BrowserTools MCP server terminal for console logs
- **Code Example**:
  ```
  === Received Extension Log ===
  {
    dataType: 'console-error',
    data: {
      message: 'Error: Failed to load resource: net::ERR_CONNECTION_REFUSED',
      stack: '...'
    }
  }
  ```

### Problem: Confusing Debugging Workflow
- **Issue**: Unclear order of operations for starting services
- **Solution**: Documented clear workflow: MCP server → Trunk server → Browser
- **Code Example**:
  ```bash
  # Step 1: Start MCP server
  npm run start-mcp
  
  # Step 2: Start Trunk server
  trunk serve --no-autoreload --open=false
  
  # Step 3: Open browser to http://localhost:8080/
  ```

### Problem: Manual NPX Commands Were Error-Prone
- **Issue**: Using raw npx commands led to inconsistent versions and setup
- **Solution**: Created package.json with scripts and setup-dev-tools.sh for consistent setup
- **Code Example**:
  ```json
  // package.json
  {
    "name": "brush-dev-tools",
    "version": "1.0.0",
    "private": true,
    "scripts": {
      "start-mcp": "browser-tools-server"
    },
    "devDependencies": {
      "@agentdeskai/browser-tools-server": "^0.1.0"
    }
  }
  ```

### Problem: Trunk Server Auto-Reload Causing Issues
- **Issue**: Auto-reload feature in Trunk caused unexpected behavior and crashes
- **Solution**: Disabled auto-reload with --no-autoreload flag
- **Code Example**:
  ```bash
  trunk serve --no-autoreload --open=false
  ```

## Common Web Errors

### Problem: "No filesystem on this platform" Errors
- **Issue**: File operations failing in WASM environment
- **Solution**: Implement platform-specific code paths
- **Code Example**:
  ```rust
  #[cfg(not(target_arch = "wasm32"))]
  fn save_file(data: &[u8], path: &Path) -> Result<(), Error> {
      std::fs::write(path, data)?;
      Ok(())
  }
  
  #[cfg(target_arch = "wasm32")]
  fn save_file(data: &[u8], _path: &Path) -> Result<(), Error> {
      // Use web-specific APIs instead
      let blob = web_sys::Blob::new_with_u8_array_sequence(&[data])?;
      // ... web download code ...
      Ok(())
  }
  ```

### Problem: Integrity Attribute Warnings
- **Issue**: Console warnings about integrity attributes for preloaded resources
- **Solution**: These can be safely ignored as they don't affect functionality
- **Code Example**:
  ```
  The resource https://localhost:8080/brush-app-f7f7f7f.js was preloaded using link preload but not used within a few seconds. Make sure all attributes of the preload tag are set correctly.
  ```

### Problem: Port Conflicts
- **Issue**: "Address already in use" errors when starting servers
- **Solution**: Check for and terminate existing processes
- **Code Example**:
  ```bash
  # Check if ports are in use
  lsof -i :3025
  lsof -i :8080
  
  # Kill processes if needed
  pkill -f "trunk serve"
  pkill -f "browser-tools-server"
  ```

### Problem: Node.js Dependencies in Rust Project
- **Issue**: Managing Node.js dependencies in a primarily Rust project
- **Solution**: Created a minimal package.json with dev dependencies and setup script
- **Code Example**:
  ```bash
  #!/bin/bash
  # setup-dev-tools.sh
  
  echo "Installing Node.js dependencies for development tools..."
  npm install
  echo "Setup complete! You can now run 'npm run start-mcp' to start the BrowserTools MCP server."
  ```

## Debugging Best Practices

### Problem: Reproducing Issues
- **Issue**: Difficulty reproducing issues consistently
- **Solution**: Standardized environment setup and documented steps
- **Code Example**:
  ```bash
  # Clear cache and restart services
  rm -rf target/wasm32-unknown-unknown
  pkill -f "trunk serve" || true
  pkill -f "browser-tools-server" || true
  npm run start-mcp
  trunk serve --no-autoreload --open=false
  ```

### Problem: Server Configuration
- **Issue**: Inconsistent server configuration
- **Solution**: Documented standard configuration in README and debugging guide
- **Code Example**:
  ```markdown
  ## Development Tools
  
  This project uses the BrowserTools MCP Server for capturing console logs and network requests during web debugging.
  
  ### Setup
  1. Ensure you have Node.js and npm installed
  2. Run `./setup-dev-tools.sh` to install dependencies
  
  ### Usage
  1. Start the BrowserTools MCP server: `npm run start-mcp`
  2. Run the web app: `trunk serve --no-autoreload --open=false`
  ```

### Problem: Debugging Environment
- **Issue**: Inconsistent debugging environment
- **Solution**: Created standardized setup with dedicated terminal tabs
- **Code Example**:
  ```
  Terminal Tab 1: npm run start-mcp
  Terminal Tab 2: trunk serve --no-autoreload --open=false
  ``` 