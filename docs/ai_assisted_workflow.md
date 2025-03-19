# AI-Assisted Development Workflow for Brush

This guide outlines best practices for using AI agents (like Claude in Cursor) to help develop and debug the Brush application.

## Contents
- [Development Environment Setup](#development-environment-setup)
- [Effective AI Collaboration Patterns](#effective-ai-collaboration-patterns)
- [Structured Debugging Process](#structured-debugging-process)
- [AI-Assisted Testing](#ai-assisted-testing)
- [Knowledge Management](#knowledge-management)

## Development Environment Setup

### Prerequisites
- [Cursor Editor](https://cursor.sh/) installed
- Chrome browser with [Browser Tools extension](https://chromewebstore.google.com/detail/browsertools-mcp/gpoigdifkoadgajcincpehpelinkjpbd)
- Brush codebase cloned locally
- NodeJS installed (for MCP server)

### Cursor Configuration
1. **Install Cursor** from [cursor.sh](https://cursor.sh/)
2. **Configure MCP connection**:
   ```bash
   mkdir -p ~/.config/cursor
   ```
   Create `~/.config/cursor/mcp.json`:
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
3. **Set up project-specific rules**:
   ```bash
   mkdir -p .cursor/rules
   ```
   Create appropriate rule files in `.cursor/rules/` (see [Rule Files](#cursor-rule-files))

### Starting the Development Environment
To start an AI-assisted debugging session:

1. **Start the MCP Server** (Give this command to the AI):
   ```bash
   cd /Users/ryanhickman/code/brush && npx @agentdeskai/browser-tools-server --port 3025
   ```

2. **Have the AI start Trunk** (in-chat):
   ```bash
   cd /Users/ryanhickman/code/brush && trunk serve --no-autoreload --open=false
   ```

3. **Open Brush in Chrome**:
   ```
   http://localhost:8080/
   ```

## Effective AI Collaboration Patterns

### Communication Best Practices

1. **Be specific about goals**:
   - "Help me fix the PLY file loading error in the web version" (good)
   - "Fix the app" (too vague)

2. **Share error context**:
   - Include exact error messages
   - Specify where errors appear (console, compilation, etc.)
   - Describe steps to reproduce

3. **Break down complex tasks**:
   - Divide large problems into smaller steps
   - Focus on one component or issue at a time
   - Validate solutions before moving to the next step

### Effective AI Prompting

1. **Problem presentation template**:
   ```
   Goal: [What you're trying to accomplish]
   Current behavior: [What's happening now]
   Expected behavior: [What should happen]
   Error messages: [Exact error text if applicable]
   Relevant code: [File paths or code snippets]
   ```

2. **Follow-up guidance**:
   - Ask for explanations when needed
   - Request alternative approaches
   - Specify your preferences for implementation style

3. **Iterative refinement**:
   - Provide feedback on proposed solutions
   - Clarify misunderstandings
   - Build on partial solutions

## Structured Debugging Process

For effective AI-assisted debugging:

### 1. Problem Identification

Ask the AI to:
1. **Analyze error patterns** from compilation output, runtime logs, and browser console
2. **Form a hypothesis** about the root cause
3. **Identify affected components**
4. **Document environment details** (web/native, browser type, etc.)

### 2. Solution Design

Work with the AI to:
1. **Outline an approach** to fixing the issue
2. **Identify specific files** that need modification
3. **Discuss implementation strategies**
4. **Consider alternative solutions** and trade-offs

### 3. Implementation

Have the AI:
1. **Make incremental changes** to one file at a time
2. **Provide clear explanations** of each change
3. **Test after each change** using automated reload:
   ```bash
   cd /Users/ryanhickman/code/brush && curl -X POST http://localhost:8080/_trunk/reload
   ```
4. **Monitor console logs** for errors or warnings

### 4. Verification

Together with the AI:
1. **Test multiple scenarios**
2. **Check for regressions**
3. **Analyze browser performance**
4. **Document the solution**

## AI-Assisted Testing

Leverage AI for automated testing:

### Automated Test Scenarios
Use URL parameters to trigger specific tests:
```
http://localhost:8080/?debug=true&test=ply-loading
```

### Performance Testing
Ask the AI to analyze metrics and suggest optimizations:
```
# Example prompt
"Can you analyze these performance metrics and suggest optimizations for PLY file loading?"
```

### Regression Testing
Use AI to verify multiple features after changes:
```
# Example prompt
"Let's verify all these features still work after our changes: (1) file loading, (2) UI rendering, (3) exporting"
```

## Knowledge Management

### Cursor Rule Files
Maintain these rule files for AI context:

- **brush_project.mdc**: High-level project context
- **brush_technical.mdc**: Technical standards and patterns 
- **brush_coding_guide.mdc**: Coding conventions and best practices
- **brush_debug.mdc**: Debugging workflows and commands
- **ryan_workflow.mdc**: Personal preferences and workflows

### Documentation Updates
After solving issues:

1. **Update code comments** with clear explanations
2. **Document solutions** in `docs/lessons_learned.md`
3. **Record debugging patterns** in `docs/debugging.md`
4. **Update Cursor rules** if general approaches have changed

### Knowledge Transfer
Use the AI to:
1. **Summarize complex changes** for team communication
2. **Generate documentation** from code
3. **Extract lessons learned** from debugging sessions
4. **Create onboarding materials** for new developers 