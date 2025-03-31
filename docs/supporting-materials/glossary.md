# 5.2. Glossary

Definitions of key terms used in the Brush project and documentation.

*   **Autodiff (Automatic Differentiation):** A technique used by frameworks like Burn to automatically compute derivatives (gradients) of functions, typically represented as computation graphs. Essential for training models by calculating how parameter changes affect the loss.
*   **Backend (Burn):** In Burn, refers to the specific computation engine used to run tensor operations (e.g., `Wgpu` for GPU via WebGPU, `NdArray` for CPU). Brush primarily uses the `Wgpu` backend.
*   **`brush` (CLI):** The command-line-only executable produced when specifically building the `brush-cli` crate (e.g., `cargo build -p brush-cli`). It resides at `target/.../brush` and offers core functionality without launching a UI by default.
*   **`brush_app`:** The main executable for Brush, typically built via `cargo build` or run via `cargo run --bin brush_app` in the workspace root. It provides the full Graphical User Interface (UI) and also accepts command-line arguments.
*   **Burn:** A deep learning framework written in Rust, used by Brush for model definition, automatic differentiation, optimization, and GPU computation via its `wgpu` backend. See [Core Technologies](technical-deep-dive/core-technologies.md#343-burn).
*   **Cargo:** The official Rust build tool and package manager. Used to compile Brush, manage dependencies, run tests, and generate documentation.
*   **COLMAP:** A popular open-source Structure-from-Motion (SfM) and Multi-View Stereo (MVS) software. Brush can load datasets processed by COLMAP (which include camera poses).
*   **Crate:** A Rust package, the smallest unit of compilation. Brush is organized into multiple crates within a Cargo workspace.
*   **Densification:** A core step in Gaussian Splatting training where new Gaussians are created (by splitting or cloning existing ones) in areas that are under-reconstructed, helping to add detail.
*   **Eframe:** A framework for the EGUI library that provides backends for running EGUI applications on native platforms and the web. See [Core Technologies](../technical-deep-dive/core-technologies.md#344-egui--eframe).
*   **EGUI:** An immediate mode GUI library for Rust, used by Brush to create its user interface (`brush-ui`). See [Core Technologies](../technical-deep-dive/core-technologies.md#344-egui--eframe).
*   **Gaussian Splatting (3DGS):** A 3D reconstruction and rendering technique that represents scenes using millions of 3D Gaussians. See [Reconstruction Pipeline](../technical-deep-dive/reconstruction-pipeline.md) and [Gaussian Splat Rendering](../technical-deep-dive/rendering-pipeline.md).
*   **Live Update Splats:** A toggle in the UI (Scene Panel) that controls whether the 3D view updates in real-time during training to show the refining splats. Disabling it can sometimes improve performance slightly.
*   **Nerfstudio Format:** A dataset format popularized by the Nerfstudio project, typically involving a `transforms.json` file defining camera poses and image paths. Brush supports loading datasets in this format.
*   **Presets Panel:** A specific panel or tab within the Brush UI dedicated to loading pre-configured example datasets (like MipNeRF or Synthetic Blender scenes) easily.
*   **Pruning:** A core step in Gaussian Splatting training where Gaussians that are deemed insignificant (e.g., very small or almost transparent) are removed to keep the model efficient.
*   **Rerun:** A visualization toolkit for multimodal data, particularly useful for time-series data like training runs. Brush integrates with Rerun (`brush-rerun` crate) to log scene data, splats, training metrics, and memory usage. ([rerun.io](https://rerun.io/))
*   **Rust:** A systems programming language focused on safety, speed, and concurrency, used as the primary language for Brush. See [Core Technologies](technical-deep-dive/core-technologies.md#341-rust).
*   **Spherical Harmonics (SH):** Mathematical functions defined on the surface of a sphere, used in computer graphics to represent view-dependent appearance efficiently. In Gaussian Splatting, SH coefficients are stored per Gaussian to model how their color changes based on viewing direction.
*   **Structure from Motion (SfM):** A photogrammetry technique used to estimate 3D structure and camera poses from 2D image sequences. Often used as a prerequisite for methods like Gaussian Splatting.
*   **Tensor:** A multi-dimensional array, the primary data structure used in machine learning frameworks like Burn to store and manipulate data (e.g., Gaussian parameters, image data, gradients).
*   **Trunk:** A build tool and asset bundler for Rust WebAssembly applications, used to build the web version of Brush. See [Core Technologies](technical-deep-dive/core-technologies.md#342-webassembly-wasm).
*   **Virtual File System (VFS):** An abstraction layer (`brush-dataset::brush_vfs`) used by Brush to handle loading datasets from different sources (local directories, ZIP archives, URLs) with a unified interface.
*   **WASM (WebAssembly):** A binary instruction format enabling high-performance code (like Rust) to run in web browsers. See [Core Technologies](technical-deep-dive/core-technologies.md#342-webassembly-wasm).
*   **WGPU:** A modern graphics and compute API specification and its Rust implementation (`wgpu`), providing portable access to GPU capabilities across different platforms and native APIs. See [Core Technologies](technical-deep-dive/core-technologies.md#345-wgpu--wgsl).
*   **WGSL:** WebGPU Shading Language, the language used to write shaders for WGPU. See [Core Technologies](technical-deep-dive/core-technologies.md#345-wgpu--wgsl).
*   **Zen Mode:** A viewing mode (controllable via URL parameter `zen=true` in the web demo) that maximizes the Scene panel and minimizes or hides other UI elements for an immersive viewing experience. 