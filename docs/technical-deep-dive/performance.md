# 3.5 Performance Considerations

This section provides guidance on profiling Brush and understanding performance characteristics.

## 3.5.1 Profiling with Tracy

Brush integrates with [Tracy Profiler](https://github.com/wolfpld/tracy) for detailed performance analysis.

**Setup:**

1.  Install Tracy:
    ```bash
    # macOS
    brew install tracy
    
    # Linux
    sudo apt install tracy
    ```

2.  Build Brush with Tracy:
    ```bash
    cargo build --release --features=tracy
    ```

3.  Run and Connect:
    ```bash
    # Start Tracy
    tracy
    
    # Run Brush
    cargo run --bin brush_app --release --features=tracy
    ```

**Key Areas to Profile:**

*   **GPU Operations:**
    - `brush-sort`: Radix sort performance
    - `brush-render`: Rasterization time
    - `brush-render-bwd`: Gradient computation
    - Memory bandwidth usage
    - Kernel execution times

*   **CPU Operations:**
    - `brush-process`: Training loop overhead
    - `brush-dataset`: Data loading
    - `brush-ui`: UI responsiveness
    - Memory allocation patterns

## 3.5.2 Common Bottlenecks

Based on Brush's implementation, common performance bottlenecks include:

*   **GPU Sorting:**
    - Radix sort in `brush-sort`
    - Memory bandwidth for large datasets
    - Synchronization overhead
    - Target: < 1ms for sorting 1M splats

*   **Rendering:**
    - Tile-based rasterization in `brush-render`
    - Memory bandwidth for splat data
    - Shader compilation time
    - Target: 60+ FPS, < 16ms frame time

*   **Training:**
    - Gradient computation in `brush-render-bwd`
    - Memory transfers between passes
    - Dataset loading and preprocessing
    - Target: 10-20 iterations/second

## 3.5.3 Hardware Requirements

**Recommended Specifications:**

*   **GPU:**
    - 8GB+ VRAM
    - Modern GPU with good compute capabilities
    - High memory bandwidth (20GB/s+)
    - Support for WebGPU/WGSL

*   **CPU:**
    - 8+ cores
    - 16GB+ RAM
    - SSD storage

*   **Storage:**
    - Fast SSD for dataset loading
    - Sufficient space for intermediate files
    - Target: 100MB/s+ throughput

## 3.5.4 Memory Management

**Key Memory Considerations:**

*   **GPU Memory:**
    - ProjectedSplat structure: 40 bytes per splat
    - Tile size: 16x16
    - Maximum splats: 10M
    - Memory layout optimized for GPU access

*   **CPU Memory:**
    - Efficient buffer reuse
    - Compact data structures
    - Smart memory allocation
    - Automatic cleanup

## 3.5.5 Performance Optimization Tips

*   **Rendering:**
    - Use appropriate tile size (16x16)
    - Monitor GPU memory usage
    - Profile shader performance
    - Optimize memory transfers

*   **Training:**
    - Monitor iteration speed
    - Profile gradient computation
    - Track memory bandwidth
    - Optimize data loading

*   **General:**
    - Enable Tracy profiling
    - Monitor frame times
    - Track memory usage
    - Profile critical paths

See **[Benchmarks](benchmarks.md)** for performance results on specific hardware.

---

## Where to Go Next?

*   See quantitative results: **[Benchmarks](../benchmarks.md)**
*   Understand the rendering steps: **[Rendering Pipeline](rendering-pipeline.md)**
*   Explore the overall structure: **[Architecture Overview](architecture.md)**
*   Learn about the core libraries used: **[Core Technologies Guide](core-technologies.md)** 