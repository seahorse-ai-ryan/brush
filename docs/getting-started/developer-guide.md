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
*   **System Dependencies:**
    *   **Linux:** Building the native desktop application requires development libraries for the windowing system. The specific packages depend on your distribution, but typically include:
        *   **For Wayland:** `libwayland-dev`, `libxkbcommon-dev` (Debian/Ubuntu) or `wayland-devel`, `libxkbcommon-devel` (Fedora).
        *   **For X11:** `libx11-dev`, `libxcb-render0-dev`, `libxcb-shape0-dev`, `libxcb-xfixes0-dev` (Debian/Ubuntu) or `libX11-devel`, `libxcb-devel` (Fedora).
        *   You might need additional fontconfig libraries (`libfontconfig1-dev` or `fontconfig-devel`).
    *   **macOS:** Standard Xcode command-line tools.
    *   **Windows:** Standard MSVC build tools (usually installed with Visual Studio).
    *   **(Android):** Requires Android SDK and NDK (see `crates/brush-android/README.md`).
*   **IDE Setup:** A code editor with Rust support is recommended.
    *   [Visual Studio Code](https://code.visualstudio.com/) with the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension is a popular choice.

## 2.2.2. Building the Project

Once the environment is set up, you can build Brush from the root directory of the repository:

*   **Desktop (Native Application):**
    *   Development build: `cargo build` (Builds all targets, including `brush_app`)
    *   Optimized release build: `cargo build --release` (Builds all targets, including `brush_app`)
    *   To build and run the desktop app directly:
        *   Development: `cargo run --bin brush_app`
        *   Release: `cargo run --bin brush_app --release`
*   **WebAssembly (WASM):**
    *   Development build and local server: `trunk serve`
    *   Optimized release build: `trunk build --release`
    *   To serve the release build locally: `trunk serve --release`
*   **Command-Line Interface (CLI only):**
    *   Build the CLI executable: `cargo build --release -p brush-cli`
    *   The executable will be located at `target/release/brush`.

## 2.2.3. Running Examples

The `examples/` directory in the repository currently does not contain standard runnable examples that can be executed with `cargo run --example <example_name>`.

To experiment with Brush's capabilities using sample data, the recommended approach is to use the main application:

1.  **Via the UI:** Run `cargo run --bin brush_app --release` and navigate to the "Presets" tab. This tab often provides links to download sample datasets suitable for testing.
2.  **Via the CLI:** Download a sample dataset (e.g., from the original 3D Gaussian Splatting sources or other repositories) and use the `brush_app` CLI to load and process it, as described in the **[User Guide](user-guide.md#212-basic-workflows-step-by-step)**.

## 2.2.4. Running Tests & Coverage

*(Note: This section describes how to run tests. Determining the exact scope and limitations of test coverage requires deeper code analysis or code coverage tooling.)*

*   **Run the full test suite:** To execute all tests across the workspace, run the following command from the root directory:
    ```bash
    cargo test --workspace
    ```
*   **Specific Setup:** Based on the dependencies, most tests should run without special setup, assuming the general development environment is configured correctly. Some integration tests, if present, might have specific requirements not immediately apparent.

## 2.2.5. Contribution Guidelines

Please refer to the [`CONTRIBUTING.md`](../../CONTRIBUTING.md) file in the root of the repository for details on:

*   Code style (including `rustfmt` and `clippy` usage based on linters configured in `Cargo.toml`)
*   Branching strategy
*   Pull Request process
*   Reporting issues

---

## Where to Go Next?

*   Understand the project structure: **[Architecture Overview](../technical-deep-dive/architecture.md)**.
*   Learn about the core algorithms: **[Reconstruction Pipeline](../technical-deep-dive/reconstruction-pipeline.md)** and **[Gaussian Splat Rendering](../technical-deep-dive/gaussian-splatting.md)**.
*   Explore the code APIs: **[API Reference](../api-reference.md)**. 