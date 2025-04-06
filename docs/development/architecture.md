# Project Architecture

This document provides a high-level overview of the Brush project's architecture, focusing on crate responsibilities and key design patterns. Understanding this structure is crucial for developers looking to modify existing functionality or add new features.

## Crate Structure and Responsibilities

Brush utilizes a workspace with multiple crates (`/crates/*`) to promote modularity and separation of concerns. This design helps isolate functionality, making the codebase easier to navigate and maintain.

*   **`brush-app`:** The main application entry point for desktop and web. Handles user-facing aspects:
    *   GUI implementation using [`egui`](https://github.com/emilk/egui) and [`eframe`](https://github.com/emilk/egui/tree/master/crates/eframe), defining panels like `SettingsPanel`, `PresetsPanel`, `ScenePanel`, `DatasetPanel`, `StatsPanel`.
    *   Panel management and layout using [`egui_tiles`](https://github.com/emilk/egui_tiles) (allowing different arrangements, e.g., sidebars or vertical splits).
    *   User input handling (mouse, keyboard) for camera control (`Controls` tooltip) and UI interaction.
    *   Command-line argument parsing (`clap`) for initial setup.
    *   Spawning and managing the background process (`brush-process`) triggered by UI actions (e.g., `Load file` button, `Presets` links).
    *   Receiving status messages (`ProcessMessage`) from the background process and updating shared UI state (`AppContext`).
    *   Sending commands (`ControlMessage`) to the background process based on user actions (e.g., changing settings sliders, toggling `▶ training` or `🔴 Live update splats`).
    *   Rendering the final image/splats to the screen/canvas via `egui_wgpu` in the `ScenePanel`.
*   **`brush-process`:** Orchestrates the main data loading, training, and viewing workflows. Acts as the bridge between the UI and the core algorithms:
    *   Receives commands (`ControlMessage`) from `brush-app`.
    *   Loads data from various sources (`DataSource`) using `brush-dataset` and a VFS abstraction (see `brush-dataset/src/brush_vfs`).
    *   Decides whether to run the viewing stream (`view_stream.rs`) or the training stream (`train_stream.rs`) based on input data.
    *   Drives the training loop (using `brush-train`) or viewing loop.
    *   Sends status updates (`ProcessMessage`) back to `brush-app` (e.g., stats for `StatsPanel`, loaded data for `DatasetPanel` and `ScenePanel`, errors).
    *   Manages the core lifecycle (e.g., loading -> training/viewing -> exporting).
*   **`brush-dataset`:** Responsible for data handling, loading, and representation:
    *   Parsing input dataset formats (COLMAP, Nerfstudio JSON) via `formats/` module.
    *   Representing scene data (cameras, images, point clouds) via structs like `Scene`, `SceneView` (in `src/scene.rs`).
    *   Loading point clouds (`.ply`) via `splat_import.rs`.
    *   Exporting trained splats to `.ply` format via `splat_export.rs`.
    *   Defines configuration structs related to data loading (`LoadDataseConfig`) and model structure (`ModelConfig`) in `src/lib.rs`.
    *   Includes a Virtual File System implementation (`src/brush_vfs.rs`) for abstracting data sources.
*   **`brush-train`:** Contains the core Gaussian Splatting training logic:
    *   Implements the optimization loop (`train.rs`) using `burn`.
    *   Defines the custom `AdamScaled` optimizer (`adam_scaled.rs`).
    *   Manages learning rate scheduling (`ExponentialLrScheduler`).
    *   Calculates losses (L1 + SSIM) using `brush-render`, `brush-render-bwd`, and the `ssim` module.
    *   Implements density control (pruning and densification/cloning) logic (`refine_if_needed` in `train.rs`).
    *   Defines training configuration (`TrainConfig` in `train.rs`).
*   **`brush-render`:** Implements the forward rendering pass for Gaussian Splatting:
    *   Defines the core `Splats` data structure (`gaussian_splats.rs`).
    *   Handles projecting 3D Gaussians to screen space.
    *   Orchestrates sorting (using `brush-sort`).
    *   Manages the rasterization pipeline using custom WGSL kernels (`brush-kernel`, `brush-wgsl`).
    *   Utilizes `burn` and `wgpu` for GPU operations.
*   **`brush-render-bwd`:** Implements the backward pass (gradients) for the differentiable renderer, enabling training:
    *   Contains specialized WGSL kernels for gradient calculation.
    *   Integrates with `burn`'s autodiff capabilities via `burn_glue.rs`.
*   **`brush-kernel` / `brush-wgsl`:** Low-level WGSL shader code for GPU computations (rasterization, sorting components, backward pass elements). `brush-kernel` provides Rust bindings/interfaces.
*   **`brush-sort` / `brush-prefix-sum`:** Helper crates providing GPU-accelerated radix sort and prefix sum implementations, crucial for efficient rendering.
*   **`brush-vfs`:** Provides a Virtual File System abstraction (`crates/brush-dataset/src/brush_vfs.rs`) for loading data from local files, zip archives, or URLs consistently.
*   **`brush-cli`:** Defines the main `clap` argument structure (`Cli`) including `ProcessArgs` and viewer options (`--with-viewer`). Although `brush-app` is the primary binary consuming these in the default setup, `brush-cli` contains the definitions relevant for headless operation or alternative frontends.
*   **`brush-ui`:** Contains shared UI helper functions (e.g., `draw_checkerboard`, `create_egui_options`) and components (`BurnTexture`) used by `brush-app`.
*   **`brush-rerun`:** Integration logic for logging data to the [Rerun](https://www.rerun.io/) visualizer (used when `rerun` feature is enabled).
*   **`brush-android`:** Code specific to building and running on the Android platform.
*   **Helper Crates:** `colmap-reader` (parsing COLMAP), `rrfd` (native file dialogs), `sync-span` (tracing utility).

## Data Flow Overview

1.  **Startup:** `brush-app` launches, parses optional CLI args, initializes `egui` and `wgpu` via `eframe`, sets up initial panel layout.
2.  **Data Load Request:** User interacts with `brush-app` UI (`SettingsPanel` buttons or `PresetsPanel` links) or provides `[DATA_SOURCE]` CLI arg specifying a `DataSource`.
3.  **Process Spawn:** `brush-app` spawns a background task/thread (`start_process` in `running_process.rs`) that runs the main logic in `brush-process`.
4.  **VFS & Data Type Check:** `brush-process` creates a VFS (`brush_vfs`) from the `DataSource`, checks file types to determine if it's a `.ply` (viewing) or a dataset (training).
5.  **Loading (`brush-dataset`):**
    *   *(Viewing):* `splat_import` loads the `.ply` into `Splats`.
    *   *(Training):* `load_dataset` parses camera poses, images, etc., into `Scene` / `SceneView`.
6.  **Message to UI:** `brush-process` sends `ProcessMessage::ViewSplats` or `ProcessMessage::Dataset` to `brush-app`.
7.  **UI Update:** `brush-app` receives the message, updates `AppContext`, and UI panels redraw (e.g., `ScenePanel` shows splats/views).
8.  **Training Loop (`brush-process` + `brush-train`):**
    *   For each step: Select batch -> Render (`brush-render`) -> Calculate loss -> Backward pass (`brush-render-bwd`) -> Optimizer step -> Density control -> Send `ProcessMessage::TrainStep`.
9.  **Export (`brush-dataset::splat_export`):**
    *   *(Manual):* Triggered by `brush-app` (`ScenePanel`) in a separate task.
    *   *(Automatic):* Triggered by `brush-process` based on `--export-every`.

## UI vs. Background Process

A key architectural pattern is the separation between the main UI thread (`brush-app`) and the background processing thread/task (`brush-process`).

*   **UI Thread (`brush-app`):** Handles user interaction via panels (e.g., `SettingsPanel`, `ScenePanel`), drawing the `egui` UI, and managing UI-relevant state (`AppContext`). Must remain responsive.
*   **Background Task (`brush-process`):** Handles potentially long-running tasks (I/O, training), updating the UI asynchronously via `ProcessMessage`s which populate panels like `StatsPanel` and `DatasetPanel`.
*   **See Also:** [UI Development Guide](./ui.md) for details on `AppContext` and message handling.

## Threading Model

*   **Native:** Uses `tokio` tasks and potentially `std::thread` for background processing.
*   **WASM:** Uses `tokio_with_wasm` which primarily schedules asynchronous tasks onto the main browser event loop. It does *not* automatically use Web Workers based on current configuration; therefore, long-running synchronous computation within spawned tasks could still impact UI responsiveness. <!-- TODO: Verify exact WASM threading/task execution model used (Web Workers?) --> <!-- Resolved: Uses main thread event loop via tokio_with_wasm -->

## Platform Support (`#[cfg]`)

The codebase uses conditional compilation (`#[cfg(...)]`) extensively to handle platform differences:

*   `#[cfg(target_family = "wasm")]` / `#[cfg(not(target_family = "wasm"))]` for web vs. native code (e.g., file dialogs, threading).
*   `#[cfg(target_os = "android")]` for Android-specific logic (`brush-android`, `eframe` features).
*   Platform-specific dependencies (e.g., windowing backends in `eframe`).
*   `wgpu` abstracts backend differences (Vulkan, Metal, DX12, WebGPU), minimizing direct platform-specific graphics code in Brush itself.

## Notes for Porting

Considerations for porting Brush to a new platform (e.g., iOS):

*   **Graphics:** Ensure `wgpu` supports the target graphics API (Metal on iOS).
*   **Windowing/UI:** Ensure `eframe`/`winit` supports the target platform's windowing system and input events.
*   **Build System:** Set up the Rust build target and any platform-specific build tools or SDKs.
*   **Platform-Specific Code:** Add necessary `#[cfg(target_os = "ios")]` blocks for APIs like filesystem access, threading, or platform service integration.
*   **Dependencies:** Check if all dependencies support the target platform.

## Next Steps

*   Explore [Core Technologies](./core-technologies.md) used in Brush.
*   Understand the [Training and Rendering Pipeline](./training-and-rendering.md).
*   Review the [Data Handling](./data-handling.md) process.
*   Learn about [UI Development](./ui.md) if modifying the interface. 