# AI Lessons Learned for Brush Development

This document captures important lessons and patterns discovered during AI-assisted development of the Brush project. These insights should be referenced by AI agents to avoid repeating past mistakes and to follow established effective practices.

## Environment Setup Issues

### Compilation and Building

```
When encountering compilation errors:
1. First check Cargo.toml for dependency version mismatches
2. Verify the Rust toolchain version matches the project requirements (1.82+)
3. Look for platform-specific dependencies that might be missing
4. Check GPU driver compatibility, especially for WebGPU features
```

### Development Environment

```
For efficient development workflow:
1. Use the main branch as the source of truth
2. Run with the --release flag for performance testing
3. Use cargo run --features=tracy for performance profiling
4. Set up the Rerun viewer for visualization during training
5. Remember the 25 tool call limit in Cursor chat sessions
```

## Code Pattern Recognition

### Architectural Patterns

```
The Brush codebase follows these architectural patterns:
1. Crates are modular with specific responsibilities
2. Interface abstractions separate platform-specific code
3. GPU operations are abstracted through the Burn ML framework
4. UI code uses the EGUI immediate mode pattern
5. The rendering pipeline follows a data-oriented design
```

### Common Error Fixes

```
For frequent error patterns:
1. When encountering "cannot borrow as mutable" errors, look for shared ownership patterns
2. For GPU-related errors, check buffer sizes and memory limits
3. WebGPU errors often relate to feature support or shader compatibility
4. Cross-platform issues typically involve API abstraction gaps
```

## Testing Guidelines

### Test Case Design

```
When writing tests:
1. Follow the Arrange-Act-Assert pattern
2. Create isolated test cases that don't depend on each other
3. Use the test helpers in the codebase for common setup
4. For GPU tests, create mock devices when possible
5. Include both success and failure test cases
```

### Regression Testing

```
To prevent regressions:
1. Run the full test suite before submitting changes
2. Test on multiple platforms when making cross-platform changes
3. Verify rendering output with visual tests or assertion helpers
4. Check performance metrics don't degrade
```

## Documentation Best Practices

### Code Comments

```
For effective code documentation:
1. Document public APIs thoroughly
2. Explain why, not just what, for complex code sections
3. Provide examples for non-obvious usage
4. Use rustdoc syntax for formatting
```

### User Documentation

```
For user-facing documentation:
1. Include examples and screenshots
2. Organize information hierarchically
3. Use emoji to highlight important points
4. Cross-reference related documentation
```

## Performance Optimization Insights

### Rendering Pipeline

```
For rendering performance:
1. Batch similar operations when possible
2. Use view frustum culling to reduce processing
3. Implement LOD (level of detail) for complex scenes
4. Profile different parts of the pipeline to identify bottlenecks
```

### Training Optimizations

```
For training performance:
1. Use smaller datasets for initial testing
2. Progressively increase complexity as training stabilizes
3. Monitor memory usage closely
4. Implement early stopping when metrics plateau
```

## UI Implementation Patterns

### EGUI Specifics

```
When working with EGUI:
1. Use windows for movable panels, panels for docked UI
2. Maintain a clear hierarchy of UI components
3. Test interaction across different screen sizes
4. Follow existing style conventions for consistency
```

### Cross-Platform Considerations

```
For consistent UI across platforms:
1. Use relative sizing rather than fixed pixel values
2. Implement different controls for touch vs mouse interaction
3. Test layout on small screens first
4. Consider device capabilities when enabling features
```

## GitHub Workflow Lessons

### Issue Management

```
For effective issue tracking:
1. Keep issues focused on single concerns
2. Include detailed reproduction steps for bugs
3. Link related issues using GitHub references
4. Prioritize issues by both impact and effort required
```

### Pull Request Process

```
For successful pull requests:
1. Keep changes focused and limited in scope
2. Include comprehensive tests for new functionality
3. Document API changes clearly
4. Never submit PRs automatically
```

---

*This document should be continuously updated as new patterns and lessons are discovered during development.* 