# Contributing to Brush

First off, thank you for considering contributing to Brush! We welcome contributions from everyone.

## Prerequisites

*   Rust 1.85.0 or later (see `rust-toolchain.toml`)
*   GPU with WebGPU support
*   8GB+ VRAM recommended
*   Basic understanding of Gaussian Splatting

## Development Setup

1.  **Clone and Build:**
    ```bash
    git clone https://github.com/ArthurBrussee/brush
    cd brush
    cargo build --workspace
    ```

2.  **Run Tests:**
    ```bash
    cargo test --workspace
    ```

3.  **Generate Documentation:**
    ```bash
    cargo doc --workspace --no-deps
    ```

## How Can I Contribute?

### Reporting Bugs

*   **Check Existing Issues:** Search [Issues](https://github.com/ArthurBrussee/brush/issues) first
*   **Create New Issue:** Include:
    - Clear title and description
    - Steps to reproduce
    - System information (OS, GPU, driver version)
    - Error messages or logs
    - Minimal reproducible example if possible

### Suggesting Enhancements

*   Open a new issue with:
    - Clear description of the enhancement
    - Use cases and benefits
    - Implementation considerations
    - Performance implications
    - Impact on existing features

### Pull Requests

1.  Fork and create branch from `main`
2.  Implement changes following our guidelines
3.  Add tests for new functionality
4.  Update documentation if needed
5.  Run quality checks:
    ```bash
    # Format code
    cargo fmt
    
    # Run clippy with all features
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    
    # Run tests
    cargo test --workspace
    
    # Generate docs
    cargo doc --workspace --no-deps
    ```
6.  Create pull request to `main`

## Code Guidelines

### Performance Considerations

*   Profile changes with Tracy (`--features=tracy`)
*   Consider GPU memory usage (splat data structure is 36 bytes)
*   Optimize for rendering performance
*   Optimize for training speed
*   Use 16x16 tile size for rendering

### Code Style

*   Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
*   Use `cargo fmt` for formatting
*   Address all clippy warnings
*   Document public APIs
*   Add comments for complex algorithms
*   Keep functions focused and reasonably sized

### Testing

*   Add unit tests for new functionality
*   Include performance tests for critical paths
*   Test edge cases and error conditions
*   Verify WebGPU compatibility
*   Test on different GPU vendors if possible

### Documentation

*   Update relevant documentation files
*   Add inline documentation for public APIs
*   Include examples for new features
*   Update performance characteristics if changed
*   Keep technical details accurate

### Commit Messages

*   Use present tense ("Add feature" not "Added feature")
*   Use imperative mood ("Move cursor" not "Moves cursor")
*   Limit first line to 72 characters
*   Reference issues after first line
*   Include technical context if needed

## Getting Help

*   Check the [Developer Guide](docs/getting-started/developer-guide.md)
*   Review the [Architecture Overview](docs/technical-deep-dive/architecture.md)
*   Read the [API Reference](docs/api-reference.md)
*   Ask questions in Issues

Thank you for contributing! 