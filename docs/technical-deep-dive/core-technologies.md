# 3.4. Core Technologies Guide

This section provides brief explanations and links related to the core technologies used in Brush.

## 3.4.1. Rust

Rust is a modern systems programming language focused on safety, speed, and concurrency. Brush leverages Rust for its performance, strong type system, excellent tooling (Cargo), and growing ecosystem.

*   Key features used: Ownership/borrowing for memory safety, Cargo for build system and package management, async/await for concurrency, comprehensive standard library.
*   [Official Website](https://www.rust-lang.org/)
*   [The Rust Book](https://doc.rust-lang.org/book/)

## 3.4.2. WebAssembly (WASM)

WebAssembly is a binary instruction format for a stack-based virtual machine. It enables high-performance code (like Rust) to run in web browsers alongside JavaScript.

*   **`wasm-bindgen`**: Facilitates high-level interactions between WASM modules (Rust) and JavaScript.
*   **`Trunk`**: An asset bundler and build tool specifically designed for Rust WASM applications, simplifying development and deployment.
*   [WebAssembly Official Website](https://webassembly.org/)
*   [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
*   [Trunk Documentation](https://trunkrs.dev/)

## 3.4.3. Burn

Burn is a flexible deep learning framework built with Rust, designed for research and production. It emphasizes flexibility, performance, and portability.

*   Brush uses Burn for defining the Gaussian Splatting model, handling automatic differentiation, managing optimization, and executing computations on the GPU via its `wgpu` backend.
*   [Burn GitHub Repository](https://github.com/tracel-ai/burn)
*   [Burn Book (Documentation)](https://burn-rs.github.io/book/)

## 3.4.4. EGUI / Eframe

EGUI is an immediate mode GUI library in Rust, known for its simplicity and ease of use. Eframe is the framework used to run EGUI applications on native platforms and the web.

*   Brush uses EGUI (`brush-ui`) to create its cross-platform graphical user interface.
*   Eframe provides the backends to render the EGUI interface using `wgpu` on both desktop and web.
*   [EGUI GitHub Repository](https://github.com/emilk/egui)
*   [Eframe Documentation](https://docs.rs/eframe/)

## 3.4.5. WGPU / WGSL

WGPU is a modern graphics and compute API specification designed for safety, performance, and portability across platforms (Vulkan, Metal, DirectX, OpenGL, WebGPU). `wgpu` is the Rust implementation.

WGSL is the WebGPU Shading Language, used to write shaders that run on the GPU.

*   Brush uses `wgpu` as its primary interface to the GPU for both rendering (via Eframe/`brush-render`) and compute kernels (`brush-kernel`, Burn backend).
*   Custom GPU logic for tasks like Gaussian rasterization, sorting, and prefix sums are likely implemented using WGSL shaders within `brush-kernel` or `brush-wgsl`.
*   [wgpu-rs Repository](https://github.com/gfx-rs/wgpu)
*   [wgpu Official Website](https://wgpu.rs/)
*   [WGSL Specification](https://www.w3.org/TR/WGSL/)

## 3.4.6. Rerun

Rerun is an SDK and visualization tool for multimodal data, particularly useful for time-series data and understanding complex systems. It helps visualize data streams and logs.

*   Brush integrates with Rerun (`brush-rerun` crate) as an optional dependency to log and visualize training progress, splat evolution, metrics, and memory usage over time, providing deeper insights into the reconstruction process.
*   [Rerun Official Website](https://rerun.io/)

---

## Where to Go Next?

*   See how these technologies fit into the project: **[Architecture Overview](architecture.md)**.
*   Understand the core algorithms built with these tools: **[Reconstruction Pipeline](reconstruction-pipeline.md)** and **[Rendering Pipeline](rendering-pipeline.md)**.
*   Get started building the project: **[Developer Guide](../getting-started/developer-guide.md)**.
*   Look up definitions for terms: **[Glossary](../supporting-materials/glossary.md)**. 