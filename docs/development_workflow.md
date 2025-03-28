# Development Workflow 🔄

This document outlines the recommended workflow for contributing to the Brush project, including Git practices, code review processes, and development guidelines.

## Development Environment Setup 🛠️

### Recommended Tools

For an optimal development experience, we recommend the following tools:

- **IDE/Editor**: Visual Studio Code with Rust Analyzer, or Cursor
- **Terminal**: Any modern terminal with Git support
- **Graphics Debugger**: RenderDoc for graphics debugging
- **Profiler**: Tracy for performance profiling
- **Visualization**: Rerun for understanding algorithm behavior

### Editor Configuration

We provide configuration files for popular editors:

- **.vscode/**: Settings and launch configurations for VS Code
- **.zed/**: Configuration for Zed editor

## Development Workflow 🌊

The standard development workflow follows these steps:

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  Select an  │     │  Create a   │     │  Implement  │     │  Test Your  │
│    Issue    │────►│  Branch     │────►│   Changes   │────►│   Changes   │
└─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
                                                                   │
                                                                   ▼
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Submit    │     │  Address    │     │ Update Your │     │   Create    │
│  Pull Req   │◄────│   Review    │◄────│  Changes    │◄────│   Commit    │
└─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
```

### 1. Issue Selection

Start by selecting an issue from the GitHub issue tracker:

- **Bug fixes**: Choose issues labeled `bug`
- **Features**: Select issues labeled `enhancement`
- **Documentation**: Look for `documentation` labels
- **Good first issues**: Issues marked with `good first issue` are ideal for beginners

### 2. Branch Creation

Create a branch with a descriptive name:

```bash
# For a bug fix
git checkout -b fix/issue-number-short-description

# For a feature
git checkout -b feature/issue-number-short-description

# For documentation
git checkout -b docs/issue-number-short-description
```

### 3. Development Cycle

Follow these practices during development:

- **Incremental Development**: Make small, focused changes
- **Frequent Commits**: Commit early and often
- **Descriptive Commit Messages**: Use clear, concise messages
- **Testing**: Add tests for new features and bug fixes
- **Documentation**: Update documentation as needed

#### Commit Message Format

Use descriptive commit messages following this format:

```
<type>: <subject>

<body>

<footer>
```

Where:
- `<type>` is one of: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`
- `<subject>` is a short description of the change (50 chars or less)
- `<body>` provides detailed explanation (optional)
- `<footer>` references issues or breaking changes (optional)

For command line commits with descriptions, use multiple `-m` flags:

```bash
git commit -m "<type>: <subject>" -m "<body>

- Additional details
- More information"
```

The first `-m` creates the title, and subsequent `-m` flags add description paragraphs.

Example:
```
feat: add support for PLY animation loading

Implement loading of animated PLY files for 4D visualization.
Includes delta frame decoding and temporal interpolation.

Closes #123
```

### 4. Testing Your Changes

Before submitting changes, ensure they work correctly:

```bash
# Run all tests
cargo test --all

# Run specific tests
cargo test -p brush-render

# Check for linting issues
cargo clippy

# Build and run the application
cargo run --release
```

### 5. Creating a Pull Request

When your changes are ready:

1. Push your branch to your fork
2. Create a pull request against the main repository
3. Fill out the pull request template with:
   - A clear description of the changes
   - References to related issues
   - Any testing considerations
   - Screenshots or videos for visual changes

## Code Review Process 👀

### Submitting for Review

Once your PR is submitted:

1. GitHub Actions will run automated checks
2. Maintainers will be notified
3. Other contributors may review your code

### Addressing Feedback

When you receive review comments:

1. Discuss any points you need clarification on
2. Make requested changes
3. Push additional commits to the same branch
4. Request re-review when ready

### Merge Process

Once approved:

1. A maintainer will merge your PR
2. Your changes will be included in the next release
3. You can delete your branch

## Coding Guidelines 📝

### Rust Style

Follow standard Rust style guidelines:

- Use Rust's naming conventions
- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Format code with `rustfmt`
- Fix warnings from `clippy`

### Project-Specific Patterns

- **Error Handling**: Use `anyhow` for application code, `thiserror` for library code
- **Async Code**: Use `tokio` for async operations
- **Logging**: Use the `log` crate for logging
- **GPU Code**: Follow WebGPU conventions and use WGSL shaders

### Documentation

Document your code as you write it:

- Every public item should have a doc comment
- Include examples for non-trivial functions
- Explain complex algorithms with comments
- Update appropriate documentation files

## Debugging Tips 🐛

### Application Debugging

- Use `cargo run --features=tracy` for performance profiling
- Add logging with `log::debug!()`, `log::info!()`, etc.
- Use Rerun for visualizing algorithm behavior

### Graphics Debugging

1. Capture frames with RenderDoc
2. Inspect shader execution
3. Analyze GPU resource usage

### Common Issues

- **WebGPU Errors**: Check shader code and resource bindings
- **Memory Issues**: Watch for large allocations and tensor shapes
- **Performance Problems**: Profile with Tracy and look for bottlenecks

## Feature Development Process 🚀

For larger features, follow this process:

1. **Design**: Discuss the approach in a GitHub issue
2. **Prototype**: Create a minimal implementation to validate the concept
3. **Implementation**: Develop the full feature
4. **Testing**: Add comprehensive tests
5. **Documentation**: Update relevant documentation
6. **Review**: Submit for peer review

## Working with AI Agents 🤖

When using AI agents for development:

1. Clearly define the scope of AI contributions
2. Review all generated code thoroughly
3. Test the code independently
4. Document any AI-assisted sections
5. Follow guidelines in [AI-Assisted Development](/project/ai_assisted_development.md)

## Release Process 📦

Brush follows a release process managed by maintainers:

1. **Version Bump**: Update version in `Cargo.toml`
2. **Changelog**: Update `CHANGELOG.md`
3. **Tag**: Create a Git tag for the version
4. **Release**: Create a GitHub release
5. **Publish**: Deploy new web version if applicable

## Community Engagement 👥

Stay connected with the Brush community:

- Join the [Discord server](https://discord.gg/TbxJST2BbC)
- Watch the GitHub repository for updates
- Participate in discussions on issues and PRs
- Help other contributors

## Next Steps 🔍

- Learn about [Debugging and Profiling](debugging_profiling.md)
- Explore the [Testing](testing.md) strategy
- Understand how to use [AI-Assisted Development](/project/ai_assisted_development.md) in development 