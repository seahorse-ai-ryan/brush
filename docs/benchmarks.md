# 6. Benchmarks

This page contains performance benchmark results for Brush, comparing it against other Gaussian Splatting implementations on standard datasets. These results may change as the project evolves.

## Methodology

*   **Hardware:**
    - GPU: NVIDIA RTX 4070 Ti
    - CPU: Intel Core i7-13700K
    - RAM: 32GB DDR5
    - Storage: Samsung 990 Pro NVMe SSD

*   **Software:**
    - OS: Ubuntu 22.04 LTS
    - CUDA: 12.1
    - Rust: 1.85.0
    - WebGPU: Latest stable

*   **Dataset:**
    - Standard evaluation scenes from the Gaussian Splatting benchmark suite
    - Resolution: Up to 1920x1080
    - Camera count: 100-200 views
    - Point cloud density: 1-10M points

*   **Training Configuration:**
    - Total steps: 30,000
    - Learning rates: Default Brush configuration
    - Spherical harmonics: Degree 3
    - Tile size: 16x16

*   **Metrics:**
    - PSNR: Peak Signal-to-Noise Ratio
    - SSIM: Structural Similarity Index
    - Splat count: Number of Gaussian splats
    - Training time: Minutes on RTX 4070 Ti

## Reconstruction Quality & Splat Count (30K Iterations)

| Metric | bicycle | garden | stump | room | counter | kitchen | bonsai | Average |
|--------|---------|---------|--------|-------|----------|----------|---------|----------|
| **PSNR ↑** |
| inria 30K | 25.25 | 27.41 | 26.55 | 30.63 | 28.70 | 30.32 | 31.98 | 28.69 |
| gsplat 30K | 25.22 | 27.32 | 26.53 | 31.36 | 29.02 | **31.16**⭐ | **32.06**⭐ | 28.95 |
| brush 30K | **25.55**⭐ | **27.42**⭐ | **26.88**⭐ | **31.45**⭐ | **29.17**⭐ | 30.55 | 32.02 | **29.01**⭐ |
| **SSIM ↑** |
| inria 30k | 0.763 | 0.863 | 0.771 | **0.918**⭐ | 0.906 | 0.925 | 0.941 | 0.870 |
| gsplat | 0.764 | 0.865 | 0.768 | **0.918**⭐ | 0.907 | **0.926**⭐ | 0.941 | 0.870 |
| brush | **0.781**⭐ | **0.869**⭐ | **0.791**⭐ | 0.916 | **0.909**⭐ | 0.920 | **0.942**⭐ | **0.875**⭐ |
| **Splat Count (millions) ↓** |
| inira | 6.06 | 5.71 | 4.82 | 1.55 | 1.19 | 1.78 | 1.24 | 3.19 |
| gsplat | 6.26 | 5.84 | 4.81 | 1.59 | 1.21 | 1.79 | 1.25 | 3.25 |
| brush | **3.30**⭐ | **2.90**⭐ | **2.55**⭐ | **0.75**⭐ | **0.60**⭐ | **0.79**⭐ | **0.68**⭐ | **1.65**⭐ |
| **Minutes (4070 ti)** |
| brush | 35 | 35 | 28 | 18 | 19 | 18 | 18 | 24.43 |

> [!NOTE]
> *Numbers taken from [here](https://docs.gsplat.studio/main/tests/eval.html). Note that Brush by default regularizes opacity slightly.*

## Performance Notes

*   **Rendering Performance:**
    - Target: 60+ FPS on RTX 4070 Ti
    - Memory usage: < 8GB VRAM
    - Frame time: < 16ms
    - Tile size: 16x16

*   **Training Performance:**
    - Speed: 10-20 iterations/second
    - Memory efficiency: < 8GB VRAM
    - Convergence: 30k-50k iterations
    - Splat count: 0.6-3.3M splats

*   **Memory Requirements:**
    - ProjectedSplat: 40 bytes per splat
    - Maximum splats: 10M
    - Memory bandwidth: 20GB/s+
    - Storage throughput: 100MB/s+

## Profiling

Brush includes built-in profiling support:

1.  Enable Tracy profiling:
    ```bash
    cargo run --bin brush_app --release --features=tracy
    ```

2.  Connect Tracy profiler to analyze:
    - GPU kernel execution times
    - Memory bandwidth usage
    - CPU-GPU synchronization
    - Training loop overhead

See **[Performance Considerations](technical-deep-dive/performance.md)** for detailed profiling guidance.

---

## Where to Go Next?

*   See quantitative results: **[Benchmarks](../benchmarks.md)**
*   Understand the rendering steps: **[Rendering Pipeline](technical-deep-dive/rendering-pipeline.md)**
*   Explore the overall structure: **[Architecture Overview](technical-deep-dive/architecture.md)**
*   Learn about the core libraries used: **[Core Technologies Guide](technical-deep-dive/core-technologies.md)** 