# AI Agent Lessons Learned - Brush Project

```json
{
  "document_type": "lessons_learned_log",
  "project_name": "Brush",
  "purpose": "Document non-obvious bugs and solutions to help AI agents learn from past mistakes",
  "usage": "Insert new entries at the top of the file, below this header section",
  "entry_format": "structured markdown with metadata and content sections",
  "last_updated": "2024-03-07"
}
```

This document serves as a knowledge base of lessons learned by AI agents while contributing to the Brush project. Each entry documents a non-obvious bug or issue that required significant effort to resolve, along with the solution and insights gained.

## How to Use This Document

- **AI Agents**: Before suggesting solutions to complex errors, check this document for similar patterns
- **Human Developers**: Review these lessons to understand common pitfalls in the Brush codebase
- **New entries**: Should be added at the top of the file, immediately below this header section

## Entry Format

Each entry should follow this format:

```markdown
---
timestamp: "YYYY-MM-DD HH:MM:SS UTC"
agent: "Agent Name and Version"
issue_category: ["ownership", "lifetime", "cross-platform", "performance", "dependency", "other"]
files_affected: ["path/to/file1.rs", "path/to/file2.rs"]
---

### Issue: Brief description of the problem

**Context**: What the developer was trying to accomplish

**Error Symptoms**: 
- Error messages or unexpected behaviors observed
- Include relevant error codes or patterns

**Root Cause**: The underlying cause of the issue

**Solution**: 
- How the issue was resolved
- Include code snippets if helpful

**Better Approach**: What would have been a better way to implement the change from the beginning

**Generalizable Lesson**: The broader principle that can be applied to similar situations
```

---

<!-- New entries should be added BELOW this line and ABOVE existing entries -->

<!-- ENTRIES START -->

<!-- TEMPLATE (copy and adapt for new entries)
---
timestamp: "YYYY-MM-DD HH:MM:SS UTC"
agent: "Agent Name and Version"
issue_category: ["category1", "category2"]
files_affected: ["path/to/file1.rs", "path/to/file2.rs"]
---

### Issue: Brief description of the problem

**Context**: What the developer was trying to accomplish

**Error Symptoms**: 
- Error messages or unexpected behaviors observed
- Include relevant error codes or patterns

**Root Cause**: The underlying cause of the issue

**Solution**: 
- How the issue was resolved
- Include code snippets if helpful

**Better Approach**: What would have been a better way to implement the change from the beginning

**Generalizable Lesson**: The broader principle that can be applied to similar situations
-->

<!-- ENTRIES END --> 