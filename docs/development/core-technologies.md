# Core Technologies

This document provides a deeper look into the key external libraries and technologies used by Brush and how they are integrated. Understanding these is helpful for modifying core functionality or debugging issues related to specific dependencies.

## Burn

[Burn](https://github.com/burn-rs/burn) is the machine learning framework providing the foundation for tensor operations and GPU compute orchestration in Brush.

*   **Tensor Operations:** Provides the core `Tensor<Backend, Rank>` type for multi-dimensional arrays and numerous mathematical operations (linear algebra, activations, etc.) executed on the GPU via the WGPU backend.
*   **GPU Backend (`burn-wgpu`):** Manages interaction with the `wgpu` API, handling device selection, buffer allocation, kernel execution (including custom WGSL via integration points or potentially `burn-cubecl`), and synchronization.
*   **Automatic Differentiation (`Autodiff<Wgpu>`):** Enables gradient calculation for training. Brush wraps the `Wgpu` backend with `Autodiff`, allowing Burn to automatically track operations on `Splats` parameters (`means`, `log_scales`, etc.) and compute gradients during the backward pass, integrating with the custom gradients from `brush-render-bwd`.
*   **Compute Kernel Integration (`burn-cubecl`, `burn-wgpu`):** While Brush implements most custom kernels directly in WGSL (managed via `brush-kernel`), it relies on types and concepts from `burn-cubecl` (like `CubeTensor`, `CubeCount`) provided through the `burn-wgpu` backend for interacting with the GPU, managing kernel dispatches, and handling tensor data.
*   **Modules & Parameters:** Brush defines the trainable `Splats` struct (`brush-render`) and manages its `Tensor` fields wrapped in `Param` directly. It interacts with Burn's optimizer and autodiff system at this level, rather than defining custom nested `Module` structs using `burn::module::Module`.
*   **See Also:** [Training & Rendering](./training-and-rendering.md), `crates/brush-train/`

## WGPU / WGSL

[WGPU](https://wgpu.rs/) is the cornerstone for graphics and compute operations, providing a modern, cross-platform API over native graphics APIs (Vulkan, Metal, DirectX 12) and the web (WebGPU).

*   **API Abstraction:** `wgpu` allows Brush to write GPU code once (primarily in WGSL) and run it across diverse hardware and operating systems.
*   **WGSL Shaders (`brush-wgsl`, `brush-kernel`):** Brush implements performance-critical parts of the pipeline using custom [WGSL](https://www.w3.org/TR/WGSL/) shaders:
    *   Forward rasterization (splatting)
    *   Backward pass gradient calculation (`brush-render-bwd`)
    *   GPU sorting (`brush-sort`)
    *   Prefix sum (`brush-prefix-sum`).
*   **Shader Management (`naga_oil`, `brush-kernel`):** `naga_oil` preprocesses WGSL, handling `#include`-like directives for modularity. `brush-kernel` provides Rust structs and functions to bind data and dispatch these custom compute/render pipelines.
*   **Integration:** `burn-wgpu` uses `wgpu` internally. Brush also interacts directly with `wgpu` via `brush-kernel` to execute its custom pipelines.
*   **Feature Usage:** Brush leverages modern GPU features via `wgpu` for performance. Notably, **atomic operations** (`atomicAdd`) are used within the WGSL shaders for the GPU radix sort (`brush-sort`) to efficiently build histograms.
*   **See Also:** [Training & Rendering](./training-and-rendering.md), `crates/brush-kernel/`, `crates/brush-wgsl/`, `crates/brush-sort/`

## Egui / Eframe

[Egui](https://github.com/emilk/egui) is an immediate mode GUI library used for the desktop and web application interface (`brush-app`). [Eframe](https://github.com/emilk/egui/tree/master/crates/eframe) is the framework used to run `egui` applications on various platforms.

*   **UI Toolkit:** `egui` provides the widgets (buttons, sliders, etc.) and layout system used in the UI panels.
*   **Application Framework:** `brush-app` implements `eframe::App` for the main application loop.
*   **Panel Management:** [`egui_tiles`](https://github.com/emilk/egui_tiles) is used for the dockable panel layout.
*   **WGPU Integration:** `eframe` initializes WGPU and provides `egui_wgpu::Renderer` for integrating `egui` rendering with custom WGPU drawing (like the splat view in `ScenePanel`).
*   **See Also:** [UI Development Guide](./ui.md), `crates/brush-app/`

## Other Key Technologies

*   **`tokio` / `tokio_with_wasm`:** Asynchronous runtime used for managing the background process (`brush-process`) and communication channels.
*   **`clap`:** Parses command-line arguments in `brush-app`.
*   **`serde`:** Handles serialization/deserialization (e.g., `transforms.json`, config files).
*   **`tracing` / `tracy` / `rerun`:** Ecosystem for logging, performance profiling (`tracy` feature), and live visualization (`rerun` feature).
*   **`naga_oil`:** WGSL shader preprocessing.
*   **`ply-rs` (forked):** Reads/writes `.ply` files.
*   **`glam`:** Provides math types (Vec3, Quat, Mat4).
*   **`rrfd`:** Cross-platform native file dialog abstraction.
*   **`colmap-reader`:** Parses COLMAP sparse reconstruction output.

## Next Steps

*   Review the overall [Project Architecture](./architecture.md).
*   See how these technologies are used in the [Training and Rendering Pipeline](./training-and-rendering.md).
*   Understand specific implementation details by browsing the linked crates and external documentation. 