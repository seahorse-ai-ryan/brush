# 3.4 Core Technologies Guide

This section documents the core technologies used in Brush, with specific implementation details and version requirements.

## 3.4.1 Rust

Brush is built with Rust, leveraging its performance, safety guarantees, and modern tooling.

*   **Version Requirements:** Rust 1.85.0 (see `rust-toolchain.toml`)
*   **Key Features Used:**
    - Ownership/borrowing for memory safety in GPU operations
    - Async/await for data loading and processing
    - Custom derive macros for shader bindings
    - Workspace-based crate organization
*   **Dependencies:**
    - glam 0.28 for 3D math
    - serde 1.0.215 for serialization
    - tokio 1.42.0 for async runtime
    - wgpu for GPU operations
*   [Official Website](https://www.rust-lang.org/)
*   [The Rust Book](https://doc.rust-lang.org/book/)

## 3.4.2 WebAssembly (WASM)

Brush supports web deployment through WebAssembly, enabling GPU-accelerated 3D reconstruction in browsers.

*   **Implementation Details:**
    - Uses `wasm-bindgen` for JavaScript interop
    - `Trunk` for asset bundling and WASM compilation
    - WebGPU backend for GPU acceleration
*   **Build Configuration:** See `Trunk.toml` for WASM-specific settings
*   **Dependencies:**
    - tokio_with_wasm 0.8.2 for WASM-compatible async
    - reqwest with rustls-tls for web-safe networking
*   [WebAssembly Official Website](https://webassembly.org/)
*   [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
*   [Trunk Documentation](https://trunkrs.dev/)

## 3.4.3 Burn

Brush uses Burn for GPU-accelerated deep learning operations.

*   **Integration Details:**
    - Custom `AdamScaled` optimizer implementation
    - WGSL backend for GPU compute
    - Memory-efficient tensor operations
*   **Key Components:**
    - `brush-burn`: Burn integration crate
    - Custom WGSL kernels for Gaussian operations
    - Fusion optimization for compute kernels
*   **Features Used:**
    - Automatic differentiation
    - GPU-accelerated tensor operations
    - Custom backend implementation
*   [Burn GitHub Repository](https://github.com/tracel-ai/burn)
*   [Burn Book (Documentation)](https://burn-rs.github.io/book/)

## 3.4.4 EGUI / Eframe

Brush's UI is built with EGUI and Eframe for cross-platform support.

*   **Implementation:**
    - Custom panels for training control and visualization
    - GPU-accelerated rendering via `wgpu`
    - Real-time performance monitoring
*   **Key Features Used:**
    - Immediate mode UI for responsive controls
    - Custom widgets for splat visualization
    - Platform-specific optimizations
*   **Dependencies:**
    - egui for UI components
    - eframe for window management
    - wgpu for rendering
*   [EGUI GitHub Repository](https://github.com/emilk/egui)
*   [Eframe Documentation](https://docs.rs/eframe/)

## 3.4.5 WGPU / WGSL

Brush's GPU operations are implemented using WGPU and WGSL.

*   **Implementation Details:**
    - Custom WGSL shaders in `brush-wgsl`
    - GPU-accelerated sorting and prefix sum
    - Efficient memory management for splat data
*   **Key Components:**
    - `brush-kernel`: WGSL kernel management
    - `brush-render`: Forward rendering pipeline
    - `brush-render-bwd`: Backward pass implementation
*   **Technical Specifications:**
    - 16x16 tile size for rendering
    - ProjectedSplat structure (10 floats, 40 bytes)
    - Custom memory layout for GPU operations
*   [wgpu-rs Repository](https://github.com/gfx-rs/wgpu)
*   [wgpu Official Website](https://wgpu.rs/)
*   [WGSL Specification](https://www.w3.org/TR/WGSL/)

## 3.4.6 Rerun

Brush integrates with Rerun for training visualization.

*   **Implementation:**
    - Optional dependency via `brush-rerun`
    - Training progress and metrics logging
    - Splat evolution visualization
*   **Usage:**
    - Enable via feature flag: `rerun`
    - Configure logging in `brush-process`
*   **Features:**
    - Real-time training visualization
    - Performance metrics tracking
    - Memory usage monitoring
*   [Rerun Official Website](https://rerun.io/)

---

## Where to Go Next?

*   See how these technologies fit into the project: **[Architecture Overview](architecture.md)**.
*   Understand the core algorithms built with these tools: **[Reconstruction Pipeline](reconstruction-pipeline.md)** and **[Rendering Pipeline](rendering-pipeline.md)**.
*   Get started building the project: **[Developer Guide](../getting-started/developer-guide.md)**.
*   Look up definitions for terms: **[Glossary](../supporting-materials/glossary.md)**. 