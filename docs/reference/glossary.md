# Glossary

Definitions for key terms and technologies used in the Brush project and documentation.

**Navigate:** [3D Recon](#3d-reconstruction--rendering) | [Brush Arch/UI](#brush-architecture--ui) | [Core Tech](#core-technologies)

<!-- TODO: Consider adding A-Z links if glossary grows significantly -->

## 3D Reconstruction & Rendering

*   **Gaussian Splatting (GS / 3DGS):** A rasterization-based technique for novel view synthesis and 3D reconstruction. Represents a scene as a collection of 3D Gaussians.
*   **Gaussian / 3D Gaussian:** A single 3D ellipsoid with associated properties (position, scale, rotation, opacity, SH coefficients) used as the fundamental building block in Gaussian Splatting. A collection of these (often stored in a `.ply` file) represents the scene's radiance field.
*   **Splat:** Technically, the 2D shape resulting from projecting a 3D Gaussian onto the image plane during rendering. Informally (and often in code/comments), used as shorthand for the 3D Gaussians themselves. The final rendered image is formed by blending many overlapping 2D splats.
    > **Note:** A collection of 3D Gaussians is the *model* or *radiance field*; a `.ply` file *stores* this model; a *splat* is the 2D projection for rendering.
*   **Spherical Harmonics (SH):** Mathematical functions used in Brush to represent the view-dependent color of each 3D Gaussian efficiently.
*   **NeRF (Neural Radiance Field):** An alternative scene representation technique using neural networks. Brush can *load datasets* originally prepared for NeRFs (e.g., camera poses from `transforms.json`) but uses Gaussian Splatting for its internal representation and rendering.
*   **COLMAP:** A popular open-source Structure-from-Motion (SfM) pipeline. Brush can *load datasets* derived from COLMAP output (camera poses, sparse points), but does not run COLMAP itself.
*   **Structure-from-Motion (SfM):** The process of estimating 3D structure and camera poses from 2D images. Brush *requires* camera poses as input, typically pre-computed using SfM tools like COLMAP.
*   **Novel View Synthesis:** The process of rendering realistic images of a scene from viewpoints not present in the original input data. In Brush, this is achieved by rendering the trained 3D Gaussians from the desired virtual camera perspective within the interactive `Scene Panel`.
*   **Differentiable Rendering:** A rendering process where gradients can be computed, allowing optimization of scene parameters (like Gaussian properties) based on image loss. Essential for Brush's training process (`brush-render-bwd`).
*   **Rasterization:** The process of converting geometric descriptions (like the 2D projected splats) into pixel colors on the screen grid. Performed on the GPU via WGSL shaders in Brush.
*   **Alpha Blending:** Combining translucent colors based on opacity. Used during splat rasterization to blend overlapping splats correctly.
*   **PLY (`.ply`):** Polygon File Format. Used by Brush to import and export the parameters (position, scale, rotation, opacity, SH coefficients) of the collection of 3D Gaussians that represent the scene.

## Brush Architecture & UI

*   **`brush-app`:** Main application crate handling UI ([`Egui`](#core-technologies)), platform integration ([`Eframe`](#core-technologies)), and interaction orchestration.
*   **`brush-process`:** Background process management, data loading coordination, main training/viewing loop driver.
*   **`brush-dataset`:** Data loading ([COLMAP](#3d-reconstruction--rendering), NeRF JSON), [VFS](#brush-architecture--ui), internal scene representation (`Scene`, `SceneView`), [PLY](#3d-reconstruction--rendering) export/import.
*   **`brush-train`:** Core training logic, optimizer (`AdamScaled`), loss calculation, density control.
*   **`brush-render`:** Forward rendering pass (projection, sorting, rasterization).
*   **`brush-render-bwd`:** Backward pass for [differentiable rendering](#3d-reconstruction--rendering).
*   **`brush-kernel` / `brush-wgsl`:** Low-level [WGSL](#core-technologies) shader implementations.
*   **`brush-sort` / `brush-prefix-sum`:** GPU sorting and prefix sum utilities.
*   **VFS (Virtual File System):** Abstraction layer (`brush-dataset/src/brush_vfs`) for reading data from local files, zip archives, or URLs.
*   **`AppContext`:** Shared state structure within `brush-app` used by UI panels.
*   **`ControlMessage` / `ProcessMessage`:** Enums used for asynchronous communication between `brush-app` and `brush-process`.
*   **Scene Panel:** The main UI panel in `brush-app` displaying the interactive 3D rendering of the Gaussian splats.
*   **Settings Panel:** UI panel in `brush-app` containing controls for model parameters, training settings, process settings, data loading, and Rerun configuration. Includes the `Presets` tab.
*   **Presets Panel:** A tab within the `Settings` panel allowing users to load pre-configured example datasets.
*   **Dataset Panel:** UI panel displaying the input images from the loaded dataset, allowing navigation and switching between training/evaluation sets.
*   **Stats Panel:** UI panel displaying statistics about the current model and training process (splat count, SH degree, training speed, memory usage, etc.).
*   **Zen Mode:** A viewing mode in the web demo (activated via `?zen=true` URL parameter) that maximizes the `Scene Panel` and hides other UI elements.

## Core Technologies

*   **Rust:** The primary programming language used for Brush, chosen for its performance, memory safety, concurrency features, and strong ecosystem support for graphics (WGPU) and web (WASM).
*   **[WGPU](https://wgpu.rs/):** A modern, cross-platform graphics and compute API abstraction layer (over Vulkan, Metal, DX12, WebGPU). Used by Burn and directly by Brush for rendering.
*   **[WGSL](https://www.w3.org/TR/WGSL/):** WebGPU Shading Language. The language used for writing GPU shaders (compute and rendering) in Brush.
*   **[Burn](https://burn-rs.github.io/book/):** A deep learning framework written in Rust, used for tensor operations, automatic differentiation, and GPU backend management (`burn-wgpu`).
*   **[Egui](https://github.com/emilk/egui):** An immediate mode GUI library in Rust used for the application interface (`brush-app`).
*   **[Eframe](https://docs.rs/eframe/latest/eframe/):** The framework used to run `egui` applications on native platforms and the web.
*   **[`egui_tiles`](https://github.com/emilk/egui_tiles):** A crate providing dockable panel management for `egui`.
*   **[Tokio](https://tokio.rs/):** Asynchronous runtime for Rust, used for managing background tasks and communication.
*   **[WASM (WebAssembly)](https://webassembly.org/):** A binary instruction format allowing code compiled from languages like Rust to run in web browsers. Brush uses WASM for its web demo/application.
*   **[Trunk](https://trunkrs.dev/):** A build tool for packaging Rust WASM applications for the web.
*   **[Tracy Profiler](https://github.com/wolfpld/tracy):** A performance profiler used for analyzing native builds (`--features=tracy`).
*   **[Rerun](https://www.rerun.io/):** A visualization tool used for live logging and viewing of training progress (`--features=rerun`).
*   **[Clap](https://docs.rs/clap/latest/clap/):** Library used for parsing command-line arguments.
*   **[Serde](https://serde.rs/):** Framework for serializing and deserializing Rust data structures.

## Next Steps

*   See where these terms are applied in the [Configuration Options Reference](./config-options.md).
*   See practical usage examples in the [Training a Scene](../guides/training-a-scene.md) and [Viewing Scenes](../guides/viewing-scenes.md) guides.
*   Understand how components fit together in the [Project Architecture](../development/architecture.md). 