# 2.1. User Guide

This guide helps end-users install and use Brush.

## 2.1.1. Installation

Brush can be run on Desktop (macOS, Windows, Linux), Android, and the Web.

*   **Desktop (macOS/Windows/Linux):**
    *   **From Source:** The primary way to run Brush currently is by building it from the source code.
        1.  [Install Rust](https://www.rust-lang.org/tools/install) (Version 1.82+ recommended).
        2.  Clone the repository: `git clone https://github.com/ArthurBrussee/brush.git`
        3.  Navigate to the repository directory: `cd brush`
        4.  Build and run the application:
            *   For development: `cargo run`
            *   For release (optimized): `cargo run --release`
    *   **(Pre-built Binaries):** *(TODO: Check if pre-built binaries are offered on the GitHub Releases page and add links if available.)*

*   **Web (WASM):**
    *   **Public Demo:** An experimental web demo is available at: [https://arthurbrussee.github.io/brush-demo](https://arthurbrussee.github.io/brush-demo)
        *   **Note:** Requires a browser supporting WebGPU (e.g., Chrome 131+ as of early 2025). Check [Can I use WebGPU?](https://caniuse.com/webgpu) for current browser support.
        *   You might need to enable specific browser flags (like "Unsafe WebGPU support" in Chrome) depending on the browser version and origin trial status.
    *   **(Running Locally):** See the [Developer Guide](developer-guide.md#building-the-project) for instructions on building and serving the web version locally using `trunk`.

*   **Android:**
    *   Refer to the specific instructions in `crates/brush-android/README.md` for building and running on Android.

## 2.1.2. Basic Workflows (Step-by-Step)

*(TODO: Detail the core user workflows with screenshots/GIFs for the UI. Add specific commands and options for the CLI.)*

*   **Workflow 1: Loading a Dataset:**
    *   **UI:** *(Describe steps, e.g., File -> Open, select COLMAP/Nerfstudio folder or Zip file for web)*
    *   **CLI:** *(Command: `brush load ...`?)*
*   **Workflow 2: Running 3D Reconstruction:**
    *   **UI:** *(Describe steps, e.g., click Train button, view progress)*
    *   **CLI:** *(Command: `brush train ...`?)*
*   **Workflow 3: Viewing the Scene:**
    *   **UI:** *(Describe navigation controls: orbit, flythrough)*
    *   **CLI:** *(View existing splat: `brush view <file.ply>`? Use `--with-viewer` flag during other operations?)*
*   **Workflow 4: Exporting Results:**
    *   **UI:** *(Describe steps, e.g., File -> Export Splat)*
    *   **CLI:** *(Command: `brush export ...`?)*
*   **Workflow 5: Using the CLI:**
    *   The main application binary also serves as the Command-Line Interface (CLI) for non-interactive operations.
    *   **Usage:** `brush_app [OPTIONS] [PATH_OR_URL]`
    *   Run from source (release build):
        ```bash
        cargo run --release -- [OPTIONS] [PATH_OR_URL]
        ```
    *   **Get Help:** For a full list of commands and options, run:
        ```bash
        cargo run --release -- --help
        ```
    *   **Key Operations & Options:**
        *   **Loading Data:** Provide a `PATH_OR_URL` argument pointing to your dataset (COLMAP folder, Nerfstudio folder, `.ply` file, URL for web). Example:
            ```bash
            cargo run --release -- /path/to/your/colmap/dataset
            ```
        *   **Training:** Training options are controlled via flags. Key flags include:
            *   `--total-steps <N>`: Number of training iterations (default: 30000).
            *   `--lr-*`: Various learning rate parameters (e.g., `--lr-mean`, `--lr-scale`).
            *   `--refine-every <N>`: Frequency of Gaussian densification/pruning (default: 150).
            *   `--export-every <N>`: How often to save a `.ply` snapshot (default: 5000).
            *   `--export-path <PATH>`: Directory for exported files.
            *   `--export-name <FILENAME_PATTERN>`: Pattern for exported filenames (e.g., `export_{iter}.ply`).
            *   `--sh-degree <N>`: Spherical Harmonics degree (default: 3).
            *   `--max-splats <N>`: Maximum number of Gaussians.
            *   Dataset options like `--max-frames`, `--max-resolution`, `--eval-split-every`.
            *   Example (Training for 10k steps):
                ```bash
                cargo run --release -- /path/to/dataset --total-steps 10000 --export-every 2000
                ```
        *   **Viewing During Operations:** Add the `--with-viewer` flag to *any* CLI command to open the UI alongside for visualization. Example:
            ```bash
            cargo run --release -- /path/to/dataset --total-steps 10000 --with-viewer
            ```
        *   **Viewing Only:** To just view an existing `.ply` file, simply provide its path:
            ```bash
            cargo run --release -- /path/to/scene.ply
            ```
        *   **Rerun Integration:** Use `--rerun-enabled` and related flags (`--rerun-log-*`) to log data to Rerun for visualization.
    *   *Note: Refer to the `--help` output for the complete and most current list of all options and their default values.*

## 2.1.3. Hardware & Software Requirements

*   **Operating Systems:**
    *   Desktop: macOS, Windows, Linux
    *   Mobile: Android
    *   Web: Browsers supporting WebGPU
*   **CPU/RAM:** *(TODO: Add general recommendations if available. Assume reasonable modern CPU/RAM.)*
*   **GPU:**
    *   A GPU compatible with WebGPU (`wgpu`) is required.
    *   Works on AMD, Nvidia, Intel GPUs.
    *   **CUDA is NOT required.**
*   **Web Browsers:**
    *   Requires a recent version of a browser supporting WebGPU.
    *   Chrome 131+ is recommended as of early 2025. Firefox/Safari support may vary.
    *   Check [Can I use WebGPU?](https://caniuse.com/webgpu). 