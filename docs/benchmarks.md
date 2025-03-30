# Benchmarks

This page contains performance benchmark results for Brush, comparing it against other Gaussian Splatting implementations on standard datasets. These results may change as the project evolves.

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

*   Rendering performance is generally expected to be competitive or faster than `gsplat`. 
*   End-to-end training speeds are expected to be similar to `gsplat`.
*   You can run benchmarks of some specific kernels using `cargo bench`.

## Profiling

> [!TIP]
> For detailed performance analysis, you can profile Brush using `tracy`.
>
> 1.  Build and run with the `tracy` feature enabled:
>     ```bash
>     cargo run --bin brush_app --release --features=tracy
>     ```
> 2.  Connect the [Tracy profiler](https://github.com/wolfpld/tracy) UI to the running application.
>
> The application UI will have options related to GPU synchronization when the `tracy` feature is enabled, which can help in obtaining more accurate GPU timings.

---

➡️ **Where to Go Next?**

*   [Technical Deep Dive: Reconstruction Pipeline](technical_deep_dive/reconstruction_pipeline.md)
*   [Technical Deep Dive: Rendering Details](technical_deep_dive/gaussian_splat_rendering.md)
*   [Back to Main README](../README.md) 