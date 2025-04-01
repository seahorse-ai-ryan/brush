# 4. API Reference

This section provides information on how to access documentation for Brush's codebase APIs, which define how software components interact.

## 4.1 Key Public APIs

Understanding Brush's library APIs is essential if you plan to contribute directly to the codebase or use its components as libraries within your own Rust applications. The codebase is organized into several key crates:

### Core Functionality
*   **`brush_app`:** The main graphical application logic and entry point
    * Handles UI initialization and event loop
    * Manages the application state and panel system
*   **`brush_train`:** Core training loop and optimization
    * Implements the training configuration and parameters
    * Provides the Adam optimizer with custom scaling
*   **`brush_render`:** Forward rendering pipeline implementation
    * GPU kernel management and WebGPU integration
    * Gaussian splat rendering algorithms
*   **`brush_dataset`:** Data loading and representation
    * Handles various dataset formats and import/export
    * Manages scene and camera data structures

### Platform-Specific
*   **`brush_android`:** Android platform integration
    * Uses winit with android-game-activity
    * Provides Android-specific logging and file handling
*   **`brush_ui`:** Cross-platform UI components
    * WebGPU configuration and device setup
    * Shared UI elements and panels

### Utility Crates
*   **`brush_wgsl`:** WGSL shader compilation and management
*   **`brush_prefix_sum`:** GPU-accelerated prefix sum operations
*   **`brush_sort`:** Sorting algorithms for GPU data
*   **`brush_cli`:** Command-line interface and argument parsing

## 4.2 Feature Flags

The codebase includes several optional features that affect API availability.

Features available in brush-app:
```rust
tracy = ["tracing", "dep:tracing-tracy"]
tracing = ["tracing-subscriber"]
```
The `tracy` feature enables performance tracing integration, while `tracing` enables debug logging with tracing-subscriber.

Features available in brush-render:
```rust
debug_validation = []
```
Enables additional validation checks during rendering.

## 4.3 Generating Documentation

The most comprehensive and accurate low-level API reference is the documentation generated directly from the Rust source code comments using `rustdoc`.

To generate documentation for all workspace crates (excluding dependencies) and view it in your browser:
```bash
cargo doc --workspace --no-deps --open
```

For development purposes, you can also generate documentation including private items:
```bash
cargo doc --workspace --no-deps --document-private-items
```

The documentation will be generated in `target/doc/` and includes:
- Public API interfaces and types
- Module structure and relationships
- Feature flag documentation
- Platform-specific functionality

Note: While the API structure is complete, some implementation details may have limited documentation. We encourage contributions to improve documentation coverage. The codebase is verified with `cargo doc` in CI to ensure documentation builds successfully.

---

## Where to Go Next

*   See the overall structure: **[Architecture Overview](technical-deep-dive/architecture.md)**
*   Understand the reconstruction process: **[Reconstruction Pipeline](technical-deep-dive/reconstruction-pipeline.md)**
*   Learn about the rendering algorithm: **[Gaussian Splat Rendering](technical-deep-dive/rendering-pipeline.md)**
*   Get started as a developer: **[Developer Guide](getting-started/developer-guide.md)** 