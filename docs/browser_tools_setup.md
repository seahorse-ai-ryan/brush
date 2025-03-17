# Using Browser Tools with Brush Development

This guide explains how to set up and use the Browser Tools MCP (Model Context Protocol) with Brush for enhanced development capabilities, such as capturing console logs from your browser.

## Prerequisites

- [Cursor Editor](https://cursor.sh/) installed
- [Chrome Browser](https://www.google.com/chrome/) with the [Browser Tools extension](https://chromewebstore.google.com/detail/browsertools-mcp/gpoigdifkoadgajcincpehpelinkjpbd) installed

## Setup Steps

### 1. Install Browser Tools Server

```bash
npm install -g @agentdeskai/browser-tools-server
```

### 2. Configure Cursor to use Browser Tools

There are two ways to configure Cursor to connect to the Browser Tools MCP server:

#### Option A: Global Configuration (Recommended for Personal Development)

Create a global configuration file that works for all projects:

```bash
mkdir -p ~/.config/cursor
```

Create `~/.config/cursor/mcp.json` with the following content:

```json
{
  "servers": [
    {
      "name": "Browser Tools MCP",
      "url": "http://localhost:3025",
      "type": "browser"
    }
  ]
}
```

#### Option B: Project-Specific Configuration

Create a project-specific configuration in the Brush repository:

```bash
mkdir -p .cursor
```

Create `.cursor/mcp.json` with the following content:

```json
{
  "servers": [
    {
      "name": "Browser Tools MCP",
      "url": "http://localhost:3025",
      "type": "browser"
    }
  ]
}
```

### 3. Start the MCP Server

There are two ways to start the Browser Tools MCP server:

#### Option A: Start in a Dedicated Terminal (Recommended)

In a Cursor terminal, run:

```bash
cd /Users/ryanhickman/code/brush && npx @agentdeskai/browser-tools-server@1.2.0 --port 3025
```

#### Option B: Use AI Assistant to Start the Server

In an AI chat session, ask the AI to run this specific command:

```
Please run the MCP server in the background.
```

The AI will execute this command:

```bash
cd /Users/ryanhickman/code/brush && npx @agentdeskai/browser-tools-server@1.2.0 --port 3025
```

### 4. Restart Cursor

Close and reopen Cursor to ensure it picks up the MCP configuration.

### 5. Start Brush Development Server

There are two ways to start the Trunk server:

#### Option A: Start in a Dedicated Terminal (Recommended)

In a separate Cursor terminal, run:

```bash
cd /Users/ryanhickman/code/brush && (pkill -f "trunk serve" || true) && echo "Stopping Trunk server..." && trunk serve --no-autoreload --open=false
```

#### Option B: Use AI Assistant to Start the Server

In an AI chat session, ask the AI to run this specific command:

```
Please start the Trunk server.
```

The AI will execute this command:

```bash
cd /Users/ryanhickman/code/brush && (pkill -f "trunk serve" || true) && echo "Stopping Trunk server..." && trunk serve --no-autoreload --open=false
```

### 6. Open in Browser and Verify

1. Open Chrome and navigate to `http://localhost:8080`
2. Open Chrome DevTools (F12 or Right-click > Inspect)
3. Go to the Console tab
4. Input a test message: `console.log("Testing MCP connection")`
5. Check the terminal where the MCP server is running - you should see this message being captured

## Troubleshooting

- **MCP server not capturing logs**: Make sure the Browser Tools extension is enabled and properly configured for the localhost domain.
- **Cursor not connecting to MCP**: Verify the MCP configuration file is in the correct location and has the right format.
- **Port conflicts**: If port 3025 is already in use, choose a different port and update both the server startup command and the MCP configuration.
- **Web app not loading properly**: Check for SRI (Subresource Integrity) errors in the browser console. These are automatically handled by a fix in `index.html`.
- **Seeing "no filesystem on this platform" errors**: These are expected in WASM environments and are now handled properly in the code.

## Notes for Brush Development

- Console logs from the application should appear in the MCP server output
- Cursor can access these logs to provide enhanced debugging capabilities
- This setup is optional but recommended for a better development experience
- When testing file uploads, a web-specific code path is used to avoid filesystem errors

## Stopping the Servers

To stop both servers when you're done:

```bash
cd /Users/ryanhickman/code/brush && (pkill -f "trunk serve" || true) && (pkill -f "browser-tools-server" || true) && echo "Stopped all servers."
```

## References

- [Cursor MCP Documentation](https://docs.cursor.sh)
- [Browser Tools MCP GitHub Repository](https://github.com/AgentDeskAI/browser-tools-mcp) 