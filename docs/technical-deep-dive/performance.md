# 3.x Performance Considerations

Optimizing the performance of 3D reconstruction and rendering is crucial. This section provides guidance on profiling Brush and understanding potential performance bottlenecks.

## 3.x.1 Profiling with Tracy

Brush integrates with the [Tracy Profiler](https://github.com/wolfpld/tracy) for detailed performance analysis, especially useful for visualizing GPU and CPU workloads over time.

**Enabling Tracy:**

1.  **Install Tracy:** Download a pre-built Tracy profiler executable for your OS from the [Tracy releases page](https://github.com/wolfpld/tracy/releases) or build it from source.
2.  **Build Brush with Tracy feature:** Compile Brush using the `tracy` feature flag:
    ```bash
    cargo build --release --features=tracy
    # Or run directly:
    cargo run --bin brush_app --release --features=tracy
    ```
3.  **Run Tracy & Connect:**
    *   Start the Tracy profiler application you downloaded/built.
    *   Run the `brush_app` executable compiled with the `tracy` feature.
    *   Tracy should automatically detect and connect to the running Brush application.

**Using Tracy:**

*   Tracy provides detailed timelines for CPU threads and GPU queues.
*   Look for long-running spans, especially on the critical path.
*   Identify GPU bottlenecks (e.g., long kernel execution times in `brush-sort`, `brush-render`, `brush-render-bwd`).
*   Analyze CPU usage, particularly during data loading or preprocessing (`brush-process`, `brush-dataset`).
*   The `sync-span` crate mentioned in the architecture might be used to automatically insert synchronization points for more accurate measurement of specific code sections.

![Example Tracy Profile](../media/tracy_profiler_example.png) *Note: Example image - replace with actual Brush profile if available.*

## 3.x.2 Common Bottlenecks

Based on typical Gaussian Splatting implementations, potential performance bottlenecks include:

*   **GPU Sorting:** Sorting millions of Gaussians by depth (`brush-sort`) before rendering is computationally intensive.
The efficiency of the radix sort implementation is critical.
*   **GPU Rasterization/Splatting:** The process of drawing the projected 2D Gaussians onto the image plane (`brush-render`, `brush-render-bwd`) involves significant memory bandwidth and computation, especially with high splat counts or complex Spherical Harmonics.
*   **GPU Memory Bandwidth:** Transferring Gaussian data, image data, and intermediate results between different GPU compute passes can be a limiting factor.
*   **Data Loading/Preprocessing:** While often less critical than GPU work, loading and preparing large datasets (`brush-dataset`, `brush-process`) can still impact startup time or introduce stalls if not efficiently parallelized or asynchronous.
*   **CPU-GPU Synchronization:** Excessive synchronization points between the CPU and GPU can lead to stalls.

## 3.x.3 Hardware Considerations

*   **GPU VRAM:** The number of Gaussians and the training image resolution directly impact GPU memory usage. Insufficient VRAM will lead to errors or extremely slow performance due to swapping.
*   **GPU Compute Power & Bandwidth:** A faster GPU with higher memory bandwidth will significantly improve both training and rendering speeds.
*   **CPU:** While less critical than the GPU for rendering/training loops, a faster CPU can improve data loading times and overall application responsiveness.
*   **Storage:** Faster storage (SSD) improves dataset loading times.

Refer to the **[Benchmarks](benchmarks.md)** page for quantitative performance results on specific hardware. 