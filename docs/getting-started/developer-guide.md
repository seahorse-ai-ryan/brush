# 2.2 Developer Guide

This guide helps developers set up their environment, build the project, and contribute.

## 2.2.1 Development Environment Setup

To build and contribute to Brush, you'll need the following:

*   **Rust:** Install Rust using `rustup`. Brush requires Rust version 1.82 or newer.
    *   Visit [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) for installation instructions.
    *   The specific toolchain version required is pinned in `rust-toolchain.toml`.
*   **WASM Target (for Web builds):** If you plan to build the WebAssembly version, add the WASM target. This is required for compiling Rust code to run in web browsers.
    ```bash
    rustup target add wasm32-unknown-unknown
    ```
*   **Trunk (for Web builds):** Trunk is used to build and bundle the WASM application, managing JavaScript interop and assets for the web.
    ```bash
    cargo install trunk
    ```
*   **Rerun (Optional, for Visualization):** Brush uses [Rerun](https://rerun.io/) for enhanced visualization during training. Install the CLI to enable these features in the UI.
    
    > [!NOTE]
    > If `rerun-cli` is not installed, the Rerun features in the UI can be enabled but will not display visualizations.

    ```bash
    cargo install rerun-cli
    ```
*   **Desktop System Dependencies:** Building the native desktop application requires certain development libraries depending on your operating system:
    *   **Linux (Wayland):** Requires `libwayland-dev`, `libxkbcommon-dev` (Debian/Ubuntu) or `wayland-devel`, `libxkbcommon-devel` (Fedora).
    *   **Linux (X11):** Requires `libx11-dev`, `libxcb-render0-dev`, `libxcb-shape0-dev`, `libxcb-xfixes0-dev` (Debian/Ubuntu) or `libX11-devel`, `libxcb-devel` (Fedora).
    *   **Linux (Common):** You may also need fontconfig libraries (`libfontconfig1-dev` or `fontconfig-devel`).
    *   **macOS:** Standard Xcode command-line tools should suffice.
    *   **Windows:** Standard MSVC build tools (usually installed with Visual Studio) are required.
*   **Android System Dependencies:** Requires Android SDK and NDK (see `crates/brush-android/README.md` for setup details).
*   **IDE Setup:** A code editor with Rust support is recommended.
    *   [Visual Studio Code](https://code.visualstudio.com/) with the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension is a popular choice.

## 2.2.2 Building the Project

Once the environment is set up, you can build Brush from the root directory of the repository:

*   **Desktop (Native Application):**
    *   Development build: `cargo build` (Builds all targets, including `brush_app`)
    *   Optimized release build: `cargo build --release` (Builds all targets, including `brush_app`)
    *   To build and run the desktop app directly:
        *   Development: `cargo run --bin brush_app`
        *   Release: `cargo run --bin brush_app --release`
*   **Web (WebAssembly):**
    *   Development build and local server: `trunk serve`
    *   Optimized release build: `trunk build --release`

    > [!IMPORTANT]
    > Building and running the WebAssembly version relies on WebGPU features.
    > * Ensure you are using an up-to-date browser that supports WebGPU (e.g., Chrome 113+, Edge 113+). WebGPU is generally enabled by default in these versions.
    > * Firefox support for WebGPU might still require enabling a flag (`dom.webgpu.enabled` in `about:config`).
    > * Performance and features (especially training) may differ significantly from the native desktop version due to the experimental nature of WebGPU and browser limitations.
    
    ![Brush Web UI Experimental Note](../media/Brush_web_Scene_panel_nothing_loaded_yet.png)
    *The web UI explicitly notes its experimental status.*
*   **Command-Line Interface (CLI only):**
    *   Build the CLI executable: `cargo build --release -p brush-cli`
    *   The executable will be located at `target/release/brush`.

## 2.2.3 Running Examples

Brush does not bundle large 3D sample datasets or pre-trained `.ply` models directly within the repository. For instructions on obtaining and using such datasets to experiment with Brush's full 3D capabilities, please refer to the **[User Guide](user-guide.md#213-core-ui-workflows)**.

The repository *does* include one small, self-contained example primarily for testing or basic demonstration:

*   **`train-2d`**: A simple application that demonstrates overfitting the 2D Gaussian Splatting components on a single image (`crab.jpg`). Useful for testing or understanding the 2D rendering/training parts.
    
    Run the 2D training example from the repository root:
    ```bash
    cargo run --example train-2d
    ```

## 2.2.4 Running Tests & Coverage

The project includes a suite of unit tests primarily focused on core algorithms and GPU kernels.

*   **Run the test suite:** To execute all tests across the workspace, run the following command from the root directory. The `--workspace` flag tells Cargo to run tests for all crates in the project.
    ```bash
    cargo test --workspace
    ```
*   **Test Coverage Scope:**
    *   Unit tests currently focus on functionalities within `brush-prefix-sum`, `brush-sort`, `brush-render`, and `brush-train`.
    *   Test coverage for application-level logic (`brush-app`, `brush-cli`), UI interactions (`brush-ui`), and end-to-end dataset loading/processing is limited.
    *   Manual testing is recommended for verifying UI behavior and complete workflows.

---

## Where to Go Next?

*   Understand the project structure: **[Architecture Overview](../technical-deep-dive/architecture.md)**.
*   Learn about the core algorithms: **[Reconstruction Pipeline](../technical-deep-dive/reconstruction-pipeline.md)** and **[Rendering Pipeline](../technical-deep-dive/rendering-pipeline.md)**.
*   Explore the code APIs: **[API Reference](../api-reference.md)**.
*   Want to contribute? See the **[Contribution Guidelines](../../CONTRIBUTING.md)** guidelines. 