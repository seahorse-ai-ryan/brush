# 4. API Reference

This section provides information on how to access documentation for Brush's codebase APIs, which define how software components interact.

## 4.1 Key Public APIs

Understanding Brush's library APIs is essential if you plan to contribute directly to the codebase or use its components as libraries within your own Rust applications, as described in the **[Extending Brush](technical-deep-dive/extending-brush.md)** guide. While the `rustdoc` generated in the next section is the definitive source for API structure, some key crates developers frequently interact with include:

*   **`brush_app`:** The main graphical application logic and entry point.
*   **`brush_train`:** Core training loop, model definition (using Burn), and optimization logic.
*   **`brush_render`:** Forward rendering pipeline implementation.
*   **`brush_dataset`:** Data loading and representation structures.
*   **`brush_cli`:** Argument parsing and command handling for the CLI.

Navigating the `rustdoc` is the best way to explore the specific functions, structs, traits, and modules available within these and other crates.

## 4.2 Generating `rustdoc` Documentation

The most comprehensive and accurate low-level API reference is the documentation generated directly from the Rust source code comments using `rustdoc`.

> Note: `rustdoc` is the standard Rust tool for generating detailed API documentation directly from comments in the source code (`.rs` files). Running the command below will generate HTML documentation outlining the structure of Brush's public library API (modules, functions, structs, etc.), which is essential if you plan to use Brush crates as libraries. However, please be aware that detailed explanations and examples within the generated `rustdoc` may currently be limited due to sparse documentation comments in the source code. For a conceptual understanding of the components, please refer to the relevant guides in the `/docs/technical-deep-dive/` directory.

Run the following command from the root of the repository to generate and open the documentation for all workspace crates (excluding external dependencies):

```bash
# Ensure you are in the root directory of the brush repository
cargo doc --workspace --no-deps --open
```

This command will build the HTML documentation and attempt to open it in your default web browser. The output will typically be located in the `target/doc/` directory.

---

## 4.3 Where to Go Next?

*   See the overall structure: **[Architecture Overview](technical-deep-dive/architecture.md)**.
*   Understand the reconstruction process: **[Reconstruction Pipeline](technical-deep-dive/reconstruction-pipeline.md)**.
*   Learn about the rendering algorithm: **[Gaussian Splat Rendering](technical-deep-dive/rendering-pipeline.md)**.
*   Get started as a developer: **[Developer Guide](getting-started/developer-guide.md)**. 