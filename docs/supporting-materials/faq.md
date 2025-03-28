# 5.1. FAQ

Frequently Asked Questions about Brush.

*   **What are 3D Gaussian Splats?**
    *   They are a representation for 3D scenes used in the "3D Gaussian Splatting" rendering and reconstruction technique. Instead of triangles or voxels, the scene is modeled as a collection of 3D ellipsoids (Gaussians) with properties like position, shape, color, and opacity. See the [Gaussian Splatting Deep Dive](technical-deep-dive/gaussian-splatting.md#331-conceptual-overview) for more details.
*   **What hardware do I need?**
    *   You need a GPU compatible with WebGPU (`wgpu`), which includes modern GPUs from AMD, Nvidia, and Intel. A high-end GPU will significantly speed up training. CUDA is *not* required. For the web version, a recent browser supporting WebGPU is needed. See [Hardware & Software Requirements](getting-started/user-guide.md#213-hardware--software-requirements).
*   **Can I use Brush commercially?**
    *   Yes, Brush is licensed under the Apache License, Version 2.0. This permissive license generally allows commercial use, modification, and distribution, subject to its terms (like retaining copyright and license notices). Please consult the `LICENSE` file in the root of the repository for the full legal text.
*   **How does Brush compare to NeRF/COLMAP/Other Methods?**
    *   **NeRF:** Neural Radiance Fields (NeRF) typically use neural networks to represent scenes volumetrically. Training and rendering NeRFs can be slow. Gaussian Splatting often achieves faster rendering and competitive quality.
    *   **COLMAP:** COLMAP is primarily a Structure from Motion (SfM) and Multi-View Stereo (MVS) tool, often used to generate the camera poses and sparse/dense point clouds that can serve as input or initialization for methods like Gaussian Splatting. Brush uses COLMAP *format* data but performs reconstruction using Gaussian Splatting.
    *   **Brush vs Other GS implementations:** Brush focuses on portability (Rust, WGPU, cross-platform) and aims for efficient performance without CUDA dependencies. Results comparisons can be found in the main `README.md`.
*   **Common Build/Runtime Issues:**
    *   *(TODO: Populate with common issues encountered during setup or use, e.g., WebGPU browser flags, missing system dependencies for Linux, specific dataset format errors.)*
    *   **WebGPU Flags:** If the web version doesn't work, ensure your browser supports WebGPU and you have enabled necessary flags (like `chrome://flags/#enable-unsafe-webgpu` in Chrome) if not using an origin trial.
    *   **Missing Poses:** Ensure your input dataset (COLMAP/Nerfstudio) contains accurate camera pose information. 