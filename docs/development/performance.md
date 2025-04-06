# Performance Considerations & Profiling

This document discusses performance aspects of Brush and how to profile the application using the integrated Tracy profiler to identify bottlenecks.

## Performance Goals

Brush aims for performance competitive with other leading Gaussian Splatting implementations, enabling both real-time rendering and reasonably fast training, while maintaining cross-platform compatibility (Desktop, Web, Android).

## Key Performance Areas & Potential Bottlenecks

Performance can be influenced by various parts of the pipeline. Understanding these areas is key for optimization efforts:

1.  **GPU Rendering (`brush-render`):** Directly impacts viewing frame rate.
    *   **Sorting (`brush-sort`):** The GPU radix sort is critical, especially with many splats. Its performance depends on the number of splats and GPU hardware capabilities.
    *   **Rasterization (`brush-kernel`, `brush-wgsl`):** The efficiency of the WGSL rasterization kernels. High overdraw (many splats covering the same pixel) increases computational cost.
    *   **GPU Utilization:** Ensuring the GPU is kept busy and not bottlenecked by CPU synchronization or inefficient kernel dispatch.
2.  **GPU Training (`brush-train`, `brush-render-bwd`):** Impacts training speed (iterations/second).
    *   **Forward/Backward Pass:** The computational cost of the differentiable rendering passes (`brush-render` + `brush-render-bwd`).
    *   **Optimizer (`AdamScaled`):** Efficiency of the parameter update step.
    *   **Density Control (`refine_if_needed`):** Pruning and densification operations involve significant GPU data manipulation (scatter, concatenate, sampling) and introduce periodic overhead.
    *   **Memory Bandwidth:** Training moves large amounts of data (parameters, gradients, intermediate values), making GPU memory bandwidth crucial.
3.  **Data Loading & Preprocessing (`brush-dataset`):** Primarily affects startup time.
    *   **I/O (via VFS):** Reading image files/archives from disk/network (using the Virtual File System abstraction `brush-vfs`).
    *   **Parsing:** Parsing dataset formats (COLMAP, JSON).
    *   **Image Operations:** CPU cost for decoding and resizing images.
4.  **CPU / GPU Synchronization:**
    *   Inefficient synchronization (e.g., frequent readbacks, unnecessary waits) can stall the pipeline.
    *   Minimizing data transfers between CPU and GPU is important.

## Profiling with Tracy

Brush integrates with the [Tracy Profiler](https://github.com/wolfpld/tracy) for detailed, low-overhead performance analysis on native platforms.

1.  **Build with Tracy Feature:** Compile Brush with the `tracy` feature flag enabled. Always use release builds for profiling.
    ```bash
    # Build with Tracy support
    cargo build --release --features=tracy

    # Or run directly with Tracy support
    cargo run --bin brush_app --release --features=tracy
    ```
2.  **Download & Run Tracy:** Get the Tracy profiler GUI application matching the library version used by Brush (check `Cargo.lock` or dependencies) for your OS from the [Tracy releases page](https://github.com/wolfpld/tracy/releases).
3.  **Connect:** Launch the Tracy-enabled Brush application *first*, then launch the Tracy profiler GUI. Tracy should automatically detect and connect to the running Brush instance (listed under "Tracy Profiler").
4.  **Analyze:** Capture a profiling session. Tracy provides detailed timelines for:
    *   **CPU Threads:** See function call durations, identify blocking operations, analyze thread interactions.
    *   **GPU Operations:** Visualize `wgpu` command buffer submissions and execution times (requires driver support). Identify expensive kernels or GPU idle time.
    *   **Synchronization:** Analyze lock contention (`sync-span`) and CPU-GPU wait times.
    *   **Memory:** Track allocations (though detailed GPU memory tracking might require backend-specific tools).

> **Note:** GPU profiling capabilities within Tracy depend heavily on the specific `wgpu` backend (Vulkan, Metal, DX12), graphics driver version, and operating system support for GPU event queries. Verification of behavior across platforms is needed. <!-- Refined: Keeping TODO for manual cross-platform testing -->

## General Optimization Considerations

*   **Target `release` builds:** Always profile and benchmark optimized release builds (`--release`).
*   **Analyze Tracy profiles:** Identify the longest poles in the CPU and GPU timelines.
*   **Shader Optimization (WGSL):** Optimize kernels in `brush-wgsl` for parallelism, register usage, and memory access patterns. Consider tile sizes and workgroup dimensions.
*   **Data Layout & Transfers:** Ensure data structures passed to the GPU (`brush-kernel` bindings) are efficiently packed (`bytemuck`) and minimize unnecessary CPU-GPU transfers.
*   **Batching:** Ensure operations process data in reasonably sized batches to maximize GPU utilization without excessive launch overhead.
*   **Asynchronous Operations:** Use `async`/`await` (`tokio`) effectively to avoid blocking threads, especially for I/O in `brush-dataset` or `brush-process`.
*   **Algorithm Choices:** Consider algorithmic alternatives (e.g., different sorting approaches, culling strategies) if a specific part is fundamentally slow.

## Next Steps

*   Understand the [Training and Rendering Pipeline](./training-and-rendering.md) to see where potential bottlenecks occur.
*   Review the [Core Technologies](./core-technologies.md) involved, especially WGPU and Burn.
*   Consult the [Tracy Profiler documentation](https://github.com/wolfpld/tracy) for advanced usage. 