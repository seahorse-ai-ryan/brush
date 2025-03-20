# Brush + MCP + Trunk Integration

This integration enhances the development workflow for Brush by connecting the BrowserTools MCP server with the Trunk build system. This enables AI assistants to:

1. Monitor browser console logs and build errors in one place
2. Trigger rebuilds when making code changes
3. Check for build errors before testing changes in the browser

## Summary of Enhancements

We've forked the BrowserTools MCP server to add new Trunk integration endpoints:

- **Trunk Status**: Check if Trunk is running and get build status
- **Build Errors**: Get a list of compilation errors
- **Trigger Rebuild**: Manual rebuild via API
- **Combined Errors**: Unified endpoint for both browser console errors and build errors

## Installation

```bash
# From the repository root
npm install /Users/ryanhickman/code/browser-tools-mcp-trunk/browser-tools-server/agentdeskai-browser-tools-server-1.2.0.tgz
```

## Development Workflow

1. Start the Trunk server (once per session):
   ```bash
   trunk serve
   ```

2. Start the MCP server with Trunk integration (once per session):
   ```bash
   npx @agentdeskai/browser-tools-server@1.2.0 --port 3025
   ```

   > **IMPORTANT**: Always use port 3025. If you get an "Address already in use" error, assume the MCP server is already running and do not try a different port.

3. Open Chrome with:
   - DevTools panel open
   - BrowserTools extension active
   - Navigate to http://localhost:8080/?diagnostic=true

4. Make code changes:
   - Trunk will automatically rebuild on file changes
   - Check for errors with: `curl http://localhost:3025/api/combined-errors`

## Benefits

- **Single Source of Truth**: Consolidated error reporting
- **AI-Friendly Development**: Structured APIs for AI agents to understand code status
- **Reduced Terminal Tabs**: No need for multiple terminal tabs to check different logs
- **Automatic Error Analysis**: Combined endpoint makes error detection more efficient

## Source Code

The extended MCP server code is available at:
https://github.com/seahorse-ai-ryan/browser-tools-mcp-trunk

## Cursor Rules

For AI assistants, we've added a Cursor rule with detailed instructions:
`.cursor/rules/brush_mcp_trunk.mdc` 