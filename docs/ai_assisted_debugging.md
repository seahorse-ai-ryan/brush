# AI-Assisted Debugging for Brush

This document outlines best practices for using AI agents (like Claude) to help debug the Brush application, with a particular focus on web/WASM development.

## Setting Up the Environment

Before starting an AI-assisted debugging session:

1. **Start the MCP Server** to capture browser console logs:
   ```bash
   cd /Users/ryanhickman/code/brush && npx @agentdeskai/browser-tools-server@1.2.0 --port 3025
   ```

2. **Start the Trunk Server** with error output filtering:
   ```bash
   cd /Users/ryanhickman/code/brush && (pkill -f "trunk serve" || true) && trunk serve --no-autoreload --open=false 2>&1 | tee >(grep -E "error|panicked|exception|warning" --color=always)
   ```

3. **Navigate to the debug launcher** which provides testing scenarios:
   ```bash
   open /Users/ryanhickman/code/brush/debug.html
   ```

## Structured AI Debugging Process

### 1. Clear Bug Identification

The AI agent must start by clearly identifying the bug before attempting any fixes:

```
Bug identification:
- Issue: [Describe the observed issue/error]
- Error messages: [Include specific error messages]
- Affected files: [List the files involved]
- Environment: [Web/Native, Browser type if relevant]
- Reproduction steps: [How to trigger the error]
```

**Example:**
```
Bug identification:
- Issue: Application crashes when attempting to upload a PLY file in web environment
- Error messages: "no filesystem on this platform" error, RuntimeError: unreachable
- Affected files: src/overlays/dataset_detail.rs, src/app.rs
- Environment: Web/WASM in Chrome browser
- Reproduction steps: 1) Open app in browser, 2) Click "Browse Zip", 3) Select a PLY file
```

### 2. Root Cause Analysis

Before proposing a fix, the AI should form a hypothesis about the root cause:

```
Root cause analysis:
- Suspected cause: [Describe why the bug is occurring]
- Evidence: [What in the code or logs supports this theory]
- Related patterns: [Similar bugs or patterns observed elsewhere]
```

**Example:**
```
Root cause analysis:
- Suspected cause: The application is using native filesystem APIs which are not available in web/WASM environment
- Evidence: Error location in dataset_detail.rs where file copying operations occur without web-specific handling
- Related patterns: Similar issues in the initialization code for storage systems
```

### 3. Solution Design

The AI should outline a clear approach to solving the issue:

```
Solution design:
- Approach: [High-level approach to fixing the bug]
- Files to modify: [Specific files that need changes]
- Implementation strategy: [Details of code changes needed]
- Testing strategy: [How to verify the fix works]
```

**Example:**
```
Solution design:
- Approach: Add platform-specific code paths for web vs. native environments
- Files to modify: src/overlays/dataset_detail.rs, src/app.rs
- Implementation strategy:
  1. Use #[cfg(target_arch = "wasm32")] for web-specific code
  2. Create alternative implementations for file operations in web context
  3. Add proper error handling for web environment
- Testing strategy:
  1. Run in browser and attempt file upload
  2. Verify console logs show proper handling
  3. Check that the application doesn't crash
```

### 4. Implementation with Incremental Verification

When implementing changes, the AI should:

1. **Make targeted changes** to one file/component at a time
2. **Use clear commit messages** describing the rationale for each change
3. **Verify after each change** using automated reload and console monitoring

### 5. Automated Reload

After code changes, the AI should refresh the browser automatically:

```bash
# Force browser reload via Trunk's internal API
cd /Users/ryanhickman/code/brush && curl -X POST http://localhost:8080/_trunk/reload
```

This allows testing without requiring the developer to manually refresh.

### 6. Comprehensive Fix Validation

After implementing all changes, validate the fix:

1. **Try multiple test scenarios** from the debug launcher
2. **Check for regression** in related functionality
3. **Analyze MCP logs** for any new issues
4. **Parse logs with analysis tool** for automated detection:
   ```bash
   cd /Users/ryanhickman/code/brush && ./analyze_logs.js mcp_log.txt
   ```

### 7. Documentation and Knowledge Capture

After resolving the issue:

1. **Document the fix** in the code with clear comments
2. **Update any relevant documentation** in the `/docs` folder
3. **Capture lessons learned** in `brush_lessons.mdc` for future reference
4. **Create test cases** to prevent regression

## Example Log Analysis

When analyzing logs, look for patterns like:

```
// Critical errors (usually indicate crashes)
RuntimeError: unreachable
panicked at 'assertion failed

// Web-specific issues
no filesystem on this platform
Failed to initialize IndexedDB

// UI rendering issues
Error: Render failed
TypeError: Cannot read property
```

## Troubleshooting Common AI Debugging Issues

1. **If the AI can't see console logs**: Ensure MCP server is running and the browser has the BrowserTools extension enabled

2. **If code changes don't appear to apply**: Check for compilation errors in the Trunk server output

3. **If the browser doesn't update after changes**: Use the automated reload command to force refresh

4. **If the AI can't reproduce the issue**: Use the debug launcher to systematically test specific scenarios 