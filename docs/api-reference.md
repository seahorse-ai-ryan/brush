# 4. API Reference

This section provides information on the Brush project's codebase APIs.

## 4.1. Generating Documentation

The most comprehensive and accurate API reference is the documentation generated directly from the Rust source code comments using `rustdoc`.

Run the following command from the root of the repository to generate and open the documentation for all workspace crates (excluding external dependencies):

```bash
# Ensure you are in the root directory of the brush repository
cargo doc --workspace --no-deps --open
```

This command will build the HTML documentation and attempt to open it in your default web browser. The output will typically be located in the `target/doc/` directory.

## 4.2. Key Public APIs

While the generated `rustdoc` is the definitive source, some key crates and modules that developers might interact with include:

*   **`brush_app`:** The main application logic and entry point.
*   **`brush_train`:** Core training loop, model definition (using Burn), and optimization logic.
*   **`brush_render`:** Forward rendering pipeline implementation.
*   **`brush_dataset`:** Data loading and representation structures.
*   **`brush_cli`:** Argument parsing and command handling for the CLI.

Navigating the `rustdoc` generated in section 4.1 is the best way to explore the specific functions, structs, traits, and modules available within these and other crates. 