# 2.2. Developer Guide

This guide helps developers set up their environment, build the project, and contribute.

## 2.2.1. Development Environment Setup

To build and contribute to Brush, you'll need the following:

*   **Rust:** Install Rust using `rustup`. Brush requires Rust version 1.82 or newer.
    *   Visit [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) for installation instructions.
    *   The specific toolchain might be pinned in `rust-toolchain.toml`.
*   **WASM Target (for Web builds):** If you plan to build the WebAssembly version, add the WASM target:
    ```bash
    rustup target add wasm32-unknown-unknown
    ```
*   **Trunk (for Web builds):** Trunk is used to build and bundle the WASM application.
    ```bash
    cargo install trunk
    ```
*   **Rerun (Optional, for Visualization):** Brush uses [Rerun](https://rerun.io/) for enhanced visualization during training.
    ```bash
    cargo install rerun-cli
    ```
*   **System Dependencies:** *(TODO: Investigate if specific system libraries are needed, e.g., for Linux windowing (Wayland/X11) or other native functionalities based on `eframe` features enabled in `Cargo.toml`.)*
*   **IDE Setup:** A code editor with Rust support is recommended.
    *   [Visual Studio Code](https://code.visualstudio.com/) with the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension is a popular choice.

## 2.2.2. Building the Project

Once the environment is set up, you can build Brush from the root directory of the repository:

*   **Desktop (Native Application):**
    *   Development build: `cargo build`
    *   Optimized release build: `cargo build --release`
    *   To build and run directly:
        *   Development: `cargo run`
        *   Release: `cargo run --release`
*   **WebAssembly (WASM):**
    *   Development build and local server: `trunk serve`
    *   Optimized release build: `trunk build --release`
    *   To serve the release build locally: `trunk serve --release`
*   **Command-Line Interface (CLI only):**
    *   Build the CLI executable: `cargo build --release -p brush-cli`
    *   The executable will be located at `target/release/brush`.

## 2.2.3. Running Examples

*(TODO: Explain how to find and run bundled examples in `examples/`. Check `examples/Cargo.toml` or individual example folders for instructions.)*

## 2.2.4. Running Tests & Coverage

*(TODO: Explain how to run tests and describe test coverage. Check if specific tests require special setup.)*

*   Run the full test suite for the workspace:
    ```bash
    cargo test --workspace
    # Or potentially: cargo test --all
    ```
*   Coverage: *(Describe scope and limitations, e.g., focuses on kernel implementations, utility functions, etc.)*

## 2.2.5. Contribution Guidelines

*(TODO: Create `CONTRIBUTING.md` if it doesn't exist, summarizing code style, PR process, etc. Mention linters from `Cargo.toml`.)*

Please refer to the `CONTRIBUTING.md` file (if available) in the root of the repository for details on:

*   Code style (Linters like `clippy` are configured in `Cargo.toml`)
*   Branching strategy
*   Pull Request process
*   Reporting issues
*   Code of Conduct 