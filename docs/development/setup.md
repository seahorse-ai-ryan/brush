# Developer Environment Setup

This guide details how to set up your local environment for developing and contributing to the Brush project. This includes building the application, running tests, and using development tools.

> **Prerequisites** ⚙️
>
> *   **Git:** For cloning the repository.
> *   **Rust Toolchain:** Brush requires a specific Rust toolchain version defined in the project to ensure consistent builds.
> *   **System Dependencies:** Libraries needed for GUI development (windowing via `winit`/`eframe`), graphics rendering (WGPU backends like Vulkan/Metal/DX12), and potentially SSL.
> *   **WASM Tools (`rustup target`, `trunk`):** Required for building and testing the WebAssembly version of the application.
> *   **(Optional) Rerun Visualization Setup (for Live Visualization):**
>     *   Install the **Rerun Viewer application** using Cargo: `cargo install rerun-cli`.
>     *   This separate viewer application displays data sent by Brush when the `rerun` feature is enabled.
>     *   **Note:** Brush uses the `rerun` Rust crate internally. You do **not** need the Python `rerun-sdk` (installed via `pip`) for this workflow.
>     *   See the [Training Guide](../guides/training-a-scene.md#using-rerun-for-detailed-visualization) for detailed usage instructions.

## 1. Clone the Repository

First, clone the Brush repository (or your fork):

```bash
git clone https://github.com/ArthurBrussee/brush.git
cd brush
```

## 2. Install Rust Toolchain

Brush specifies the exact Rust version required in the [`rust-toolchain.toml`](../../rust-toolchain.toml) file.

1.  **Install `rustup`:** If you don't have it, install the Rust toolchain manager from [rustup.rs](https://rustup.rs/).
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
2.  **Install Toolchain:** `rustup` should automatically detect `rust-toolchain.toml` when you `cd` into the `brush` directory and install the correct toolchain version (`1.85.0` as of this writing) and components (`rustfmt`, `clippy`) if they aren't already present. You might be prompted to install them.
    > **Tip:** If `rustup` doesn't automatically install, you can manually trigger it by running `rustup show` inside the project directory.

## 3. Install System Dependencies

Install the necessary development libraries for your operating system. These are primarily needed for compiling dependencies like `winit` (windowing, used by `eframe`) and `wgpu` (graphics API backend). (The lists below are based on common setups and CI configurations. Brush aims to support macOS, Windows, and Linux - consult your distribution's package manager and upstream ([ArthurBrussee/brush](https://github.com/ArthurBrussee/brush)) documentation/issues if problems arise.)

*   **Linux (Debian/Ubuntu based):**
    ```bash
    sudo apt update
    sudo apt install build-essential pkg-config libssl-dev
    sudo apt install libgtk-3-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev
    sudo apt install libgl1-mesa-dev libvulkan1 mesa-vulkan-drivers
    ```
    > **Note:** Package names might differ slightly on other distributions.
*   **macOS:**
    ```bash
    xcode-select --install
    ```
    > **Note:** macOS typically handles graphics drivers (Metal) automatically.
*   **Windows:**
    *   Install **Visual Studio** (Community edition is fine) with the **"Desktop development with C++"** workload selected. [Download Visual Studio](https://visualstudio.microsoft.com/downloads/).
    *   Ensure up-to-date graphics drivers supporting Vulkan or DirectX 12.

## 4. Install Build Tools & Targets

Ensure you have the necessary targets and build tools installed:

*   **WASM Target & Trunk (Required for Web):**
    *   Install the WebAssembly target: `rustup target add wasm32-unknown-unknown`
    *   Install the Trunk build tool: `cargo install trunk`
*   **(Optional) Rerun SDK (for Live Visualization):**
    *   Follow the [Rerun SDK installation guide](https://www.rerun.io/docs/getting-started/installing-the-sdk).

## 5. Building the Project

You can build and run the project using standard `cargo` commands:

*   **Build All Crates (Debug):** Checks for compilation errors, faster build time.
    ```bash
    cargo build
    ```
*   **Build All Crates (Release):** Creates optimized artifacts, slower build time.
    ```bash
    cargo build --release
    ```
*   **Run Desktop App (`brush-app`):** The main GUI application.
    ```bash
    cargo run --bin brush_app
    cargo run --bin brush_app --release
    cargo run --bin brush_app --release --features=rerun
    ```
*   **Build/Run Web App:** Uses `trunk` to manage the WASM build.
    ```bash
    trunk serve --open
    trunk build --release
    ```

## 6. Running Checks & Tests

Before committing code, run these checks to ensure code quality and prevent regressions. These are also run by the CI pipeline ([`.github/workflows/ci.yml`](../../.github/workflows/ci.yml)):

*   **Format Check:** Ensure code adheres to `rustfmt` standards.
    ```bash
    cargo fmt --all -- --check
    ```
*   **Linter Check (Clippy):** Catch common mistakes and style issues.
    ```bash
    cargo clippy --all-targets --all-features -- -D warnings
    ```
*   **Basic Compile Check:** Check compilation for all features and targets.
    ```bash
    cargo check --locked --all-features --all-targets
    ```
*   **Run Unit & Integration Tests:** Execute automated tests within the codebase.
    ```bash
    cargo test --all
    ```
*   **Run Documentation Tests:** Ensure code examples in documentation comments compile and run correctly.
    ```bash
    cargo test --doc
    ```

## Next Steps

With the environment set up, you can explore:

*   [Project Architecture](./architecture.md)
*   [Core Technologies](./core-technologies.md)
*   Guides for specific areas like [UI Development](./ui.md) 