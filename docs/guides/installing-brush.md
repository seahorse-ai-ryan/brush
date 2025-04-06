# Installing Brush

This guide covers how to get Brush running on your system. Choose the method that best suits your needs:

1.  [Pre-built Binaries](#pre-built-binaries-recommended) (Easiest)
2.  [Web Demo](#web-demo) (Quickest to try, experimental)
3.  [Build from Source](#build-from-source) (For users & developers needing the latest changes)

## Pre-built Binaries (Recommended)

Using pre-built binaries is the simplest way to get started on Windows, macOS (Apple Silicon), and Linux (x64).

*   **Download:** Go to the [**GitHub Releases page**](https://github.com/ArthurBrussee/brush/releases). Find the latest release (e.g., `0.2.0`) and download the appropriate archive (`.zip` for Windows, `.tar.xz` for macOS/Linux) for your system.
*   **Extract:** Unzip (Windows) or extract (`tar -xf <filename>`) the downloaded file.
*   **Run:** Find the `brush_app` executable within the extracted folder and run it.

## Web Demo

A web-based version of Brush is available for quick experimentation without installing anything locally.

*   **Access:** Visit [**arthurbrussee.github.io/brush-demo/**](https://arthurbrussee.github.io/brush-demo/)
*   **Warning:** The public web demo is experimental.
*   **Browser Requirements:** Requires a modern browser supporting **WebGPU** and the **`subgroups`** feature. Upstream testing indicates Chrome 131+ works well, often requiring the `enable-unsafe-webgpu` flag. Firefox/Safari support may be incomplete.
*   **Limitations:** Web performance may be lower than native. As of mid-2024, a known issue ([Burn #2901](https://github.com/tracel-ai/burn/issues/2901)) prevents *training* datasets reliably in the browser, though viewing pre-trained `.ply` files generally works.

## Build from Source

Building from source ensures you have the latest features and is necessary for development or if pre-built binaries aren't available for your platform.

### Prerequisites ⚙️

1.  **Rust:** Install Rust (latest stable version recommended, 1.78+ required) via [rustup](https://rustup.rs/):
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
    Follow the on-screen instructions.
2.  **System Dependencies:** Install necessary development libraries for your OS. (The lists below are based on common setups and CI configurations. Brush aims to support macOS, Windows, and Linux - consult your distribution's package manager and upstream ([ArthurBrussee/brush](https://github.com/ArthurBrussee/brush)) if problems arise.)
    *   **Linux (Debian/Ubuntu):**
        ```bash
        # General build tools
        sudo apt update
        sudo apt install build-essential pkg-config libssl-dev
        # GUI dependencies (GTK, XCB, XKB)
        sudo apt install libgtk-3-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev
        # Graphics dependencies (OpenGL, Vulkan for WGPU)
        sudo apt install libgl1-mesa-dev libvulkan1 mesa-vulkan-drivers
        ```
    *   **macOS:**
        ```bash
        xcode-select --install
        ```
    *   **Windows:** Install Visual Studio with C++ development workload. [Download Visual Studio](https://visualstudio.microsoft.com/downloads/). Ensure "Desktop development with C++" is selected during installation.

3.  **(Optional) WebAssembly (WASM) Target:** Required only if you plan to build the web version:
    ```bash
    rustup target add wasm32-unknown-unknown
    cargo install trunk
    ```
4.  **(Optional) Rerun SDK:** Required only if you plan to use the live training visualization feature (`--features=rerun`):
    *   Follow the [Rerun SDK installation guide](https://www.rerun.io/docs/getting-started/installing-the-sdk).

### Build Steps

1.  **Clone the Repository:**
    ```bash
    git clone https://github.com/ArthurBrussee/brush.git
    cd brush
    ```
    (Replace URL with your fork if necessary)

2.  **Build and Run Desktop App:**
    *   **Debug Build (Faster compile, slower runtime):**
        ```bash
        cargo run --bin brush_app
        ```
    *   **Release Build (Slower compile, faster runtime):**
        ```bash
        cargo run --bin brush_app --release
        ```
    *   **(Optional) Enable Rerun Visualization:**
        ```bash
        # Ensure Rerun SDK is installed first
        cargo run --bin brush_app --release --features=rerun
        ```

3.  **(Optional) Build and Run Web App:**
    *   Navigate to the project root directory in your terminal.
    *   Run Trunk to build and serve:
        ```bash
        trunk serve --open
        ```
    *   This will build the WASM package and open the web app in your default browser.

### Building for Android

Instructions for building and running Brush on Android devices can be found in the Android-specific README:

*   [`crates/brush-android/README.md`](../../crates/brush-android/README.md)

<video src="https://github.com/user-attachments/assets/d6751cb3-ff58-45a4-8321-77d3b0a7b051" controls width="100%"></video>
*Brush training live on an Android device (Pixel 7).*

### Next Steps

Once installed, you can proceed to:

*   [Training a Scene](./training-a-scene.md)
*   [Viewing Pre-Trained Scenes](./viewing-scenes.md)
*   Learn about the [Command Line Interface](./cli-usage.md)

See the [Development Setup Guide](../development/setup.md) for more advanced build configurations and troubleshooting. 