# Brush Documentation Plan

This document outlines the documentation structure for the Brush project, defining what information should go where to minimize duplication and maximize usability for both humans and AI assistants.

## Documentation Types

The Brush project has two main types of documentation:

1. **Human-facing documentation** in `/docs/`
   - Comprehensive guides and references
   - Detailed setup instructions
   - Troubleshooting information
   - Development practices

2. **AI-specific context** in `/.cursor/rules/`
   - Concise project overviews
   - Technical standards and patterns
   - Common solutions and workflows
   - User-specific preferences

## Documentation Structure

### Human Documentation in `/docs/`

| File | Purpose | Primary Audience | Content |
|------|---------|------------------|---------|
| `README.md` | Project overview | New users & contributors | Introduction, key features, quick start |
| `development_environment.md` | Setup guide | Developers | Installation, configuration, tools setup |
| `debugging.md` | Debugging guide | Developers | Debugging tools, workflows, common issues |
| `ai_assisted_workflow.md` | AI collaboration | Developers using AI | Effective prompting, AI workflows, Cursor setup |
| `lessons_learned.md` | Knowledge repository | All developers | Solutions to common problems, patterns, tricks |
| `architecture.md` | Technical overview | Developers | System design, component interactions, rationale |
| `user_guide.md` | Usage documentation | End users | User interface, features, step-by-step guides |

### AI Context in `/.cursor/rules/`

| File | Purpose | Updates | Content |
|------|---------|---------|---------|
| `brush_project.mdc` | Project context | When architecture changes | Overview, tech stack, architecture, priorities |
| `brush_technical.mdc` | Technical standards | When standards change | Coding patterns, tools usage, implementation guidelines |
| `brush_coding_guide.mdc` | Code conventions | When conventions change | Style, naming, organization, error handling |
| `brush_debug.mdc` | Debugging workflows | When workflows change | Commands, debugging process, issue identification |
| `ryan_workflow.mdc` | Personal preferences | When preferences change | User-specific setup, tools, workflow |

## Content Ownership

To avoid duplication and ensure content is in the right place:

### Canonical Sources of Truth

1. **Project Architecture**: `docs/architecture.md` (detailed), `/.cursor/rules/brush_project.mdc` (concise)
2. **Development Setup**: `docs/development_environment.md`
3. **Debugging Process**: `docs/debugging.md` (detailed), `/.cursor/rules/brush_debug.mdc` (concise)
4. **Code Standards**: `/.cursor/rules/brush_coding_guide.mdc`
5. **Solutions & Patterns**: `docs/lessons_learned.md`

### Cross-References Instead of Duplication

When a document needs to reference information owned by another document:

```markdown
For detailed information on setting up the development environment, see 
[Development Environment Setup](./development_environment.md).
```

## Documentation Maintenance

### Update Process

1. When fixing a bug or implementing a feature:
   - Add the solution to `docs/lessons_learned.md` if broadly applicable
   - Update relevant human documentation if it affects user workflows
   - Update AI context files only if they're now incorrect

2. When changing development workflows:
   - Update human documentation first (`docs/debugging.md`, etc.)
   - Then update AI context with concise versions (`/.cursor/rules/brush_debug.mdc`)

### Documentation Review

Periodically review documentation for:
1. Outdated information
2. Duplicated content that should be consolidated
3. Missing information for new features or workflows
4. Effectiveness of AI context (is the AI getting accurate information?)

## Deprecated Documentation

The following files have been consolidated and should not be updated:
- `docs/debugging-web.md` → `docs/debugging.md`
- `docs/browser_tools_setup.md` → `docs/debugging.md`
- `docs/ai_assisted_debugging.md` → `docs/ai_assisted_workflow.md`
- `docs/recordinglessonslearned.md` → `docs/lessons_learned.md`

## Context Window Optimization for AI

Since AI systems have limited context windows:

1. Cursor rules should be concise and focused
2. Use headings and structure to enable selective reading
3. Include clear indicators of what information is where
4. Focus on decision guidance rather than exhaustive details
5. Use concrete examples for complex patterns

## Documentation Assessment Checklist

When evaluating documentation quality:

- [ ] Is information in the canonical location?
- [ ] Do cross-references point to the right documents?
- [ ] Are AI context files concise enough for context windows?
- [ ] Is human documentation sufficiently detailed?
- [ ] Are command examples up-to-date?
- [ ] Do documents follow a consistent structure?
- [ ] Is information organized for easy discovery? 