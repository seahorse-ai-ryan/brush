# 5.2. Glossary

Definitions of key terms used in the Brush project and documentation.

*(TODO: Define terms specific to the project or relevant technologies.)*

*   **Burn:** A deep learning framework written in Rust, used by Brush for model definition, automatic differentiation, optimization, and GPU computation via its `wgpu` backend. See [Core Technologies](technical-deep-dive/core-technologies.md#343-burn).
*   **Cargo:** The official Rust build tool and package manager. Used to compile Brush, manage dependencies, run tests, and generate documentation.
*   **COLMAP:** A popular open-source Structure-from-Motion (SfM) and Multi-View Stereo (MVS) software. Brush can load datasets processed by COLMAP (which include camera poses).
*   **Crate:** A Rust package, the smallest unit of compilation. Brush is organized into multiple crates within a Cargo workspace.
*   **Eframe:** A framework for the EGUI library that provides backends for running EGUI applications on native platforms and the web. See [Core Technologies](technical-deep-dive/core-technologies.md#344-egui--eframe).
*   **EGUI:** An immediate mode GUI library for Rust, used by Brush to create its user interface (`brush-ui`). See [Core Technologies](technical-deep-dive/core-technologies.md#344-egui--eframe).
*   **Gaussian Splatting (3DGS):** A 3D reconstruction and rendering technique that represents scenes using millions of 3D Gaussians. See [Reconstruction Pipeline](technical-deep-dive/reconstruction-pipeline.md) and [Gaussian Splat Rendering](technical-deep-dive/gaussian-splatting.md).
*   **Nerfstudio Format:** A dataset format popularized by the Nerfstudio project, typically involving a `transforms.json` file defining camera poses and image paths. Brush supports loading datasets in this format.
*   **Rust:** A systems programming language focused on safety, speed, and concurrency, used as the primary language for Brush. See [Core Technologies](technical-deep-dive/core-technologies.md#341-rust).
*   **Structure from Motion (SfM):** A photogrammetry technique used to estimate 3D structure and camera poses from 2D image sequences. Often used as a prerequisite for methods like Gaussian Splatting.
*   **Trunk:** A build tool and asset bundler for Rust WebAssembly applications, used to build the web version of Brush. See [Core Technologies](technical-deep-dive/core-technologies.md#342-webassembly-wasm).
*   **WASM (WebAssembly):** A binary instruction format enabling high-performance code (like Rust) to run in web browsers. See [Core Technologies](technical-deep-dive/core-technologies.md#342-webassembly-wasm).
*   **WGPU:** A modern graphics and compute API specification and its Rust implementation (`wgpu`), providing portable access to GPU capabilities across different platforms and native APIs. See [Core Technologies](technical-deep-dive/core-technologies.md#345-wgpu--wgsl).
*   **WGSL:** WebGPU Shading Language, the language used to write shaders for WGPU. See [Core Technologies](technical-deep-dive/core-technologies.md#345-wgpu--wgsl). 