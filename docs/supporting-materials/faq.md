# 5.1. FAQ

Frequently Asked Questions about Brush.

### General

*   **What is 3D Gaussian Splatting?**
    *   It's a technique representing 3D scenes using millions of tiny, colored, semi-transparent ellipsoids (Gaussians). It allows for fast, high-quality rendering directly from data learned from images. See the [Gaussian Splatting Deep Dive](technical-deep-dive/rendering-pipeline.md#331-conceptual-overview) for more details.
*   **Can I use Brush for commercial projects?**
    *   Yes, Brush is licensed under the Apache License, Version 2.0. This permissive license generally allows commercial use, modification, and distribution, subject to its terms (like retaining copyright and license notices). Please consult the `LICENSE` file in the root of the repository for the full legal text.

### Installation & Hardware

*   **Can I download pre-compiled binaries?**
    *   Yes! Pre-built binaries for Linux (x86_64), macOS (Apple Silicon), and Windows (x86_64) are typically available for tagged releases. Check the **[GitHub Releases page](https://github.com/ArthurBrussee/brush/releases)** under "Assets".
*   **What hardware do I need?**
    *   You need a GPU compatible with WebGPU (`wgpu`), which includes modern GPUs from AMD, Nvidia, and Intel on desktop/laptop systems, and mobile GPUs on Android. A high-end GPU will significantly speed up training. See [Hardware & Software Requirements](getting-started/user-guide.md#213-hardware--software-requirements) for details.
*   **Is CUDA required?**
    *   No, CUDA is **not** required. Brush uses `wgpu`, which leverages native GPU APIs like Metal, Vulkan, or DirectX 12. This allows Brush to run on a wider range of hardware (including AMD and Intel GPUs) without needing specific Nvidia drivers or libraries.
*   **What are the WebGPU limitations?**
    *   WebGPU is a relatively new standard. Currently (early 2025), browser support is best in **Chrome 131+**. Firefox and Safari support is experimental and may vary.
    *   Performance in the browser is generally lower than the native desktop application, especially for training.
    *   Due to an issue in the underlying Burn library ([tracel-ai/burn#2901](https://github.com/tracel-ai/burn/issues/2901)), training new datasets directly in the web version may not function correctly, though viewing `.ply` files works.

### Data & Formats

*   **Which dataset formats can Brush reconstruct?**
    *   Brush primarily supports:
        *   **COLMAP format:** Reads `cameras.txt`/`.bin`, `images.txt`/`.bin`, `points3D.txt`/`.bin`.
        *   **Nerfstudio format:** Reads `transforms.json` along with images in an `images/` subfolder.
    *   Datasets can be provided as an unzipped folder (native only) or a `.zip` archive (native and web).
*   **What camera models are supported for training images?**
    *   Brush can parse various camera models defined in COLMAP data (like `PINHOLE`, `OPENCV`, `OPENCV_FISHEYE`).
    *   However, the actual training and rendering process currently **treats all cameras as simple pinhole models**, deriving focal length and field-of-view from the parameters but ignoring distortion coefficients. Complex models like fisheye or 360-degree cameras are not explicitly optimized for.
*   **What are the file export options?**
    *   Brush exports trained models to the standard Gaussian Splatting **`.ply`** file format. This format typically includes properties for each Gaussian: `x`, `y`, `z` (position), `scale_0`, `scale_1`, `scale_2` (log scale), `opacity` (logit opacity), `rot_0`..`rot_3` (rotation quaternion), `f_dc_0`..`f_dc_2` (DC component of Spherical Harmonics), and `f_rest_0`..`f_rest_N` (higher-order SH coefficients).
*   **Are the exported `.ply` files compatible with other viewers/tools?**
    *   Generally, yes. The exported `.ply` files adhere to the common format used by many Gaussian Splatting viewers and tools (like SuperSplat, Polycam viewer, Luma WebGL viewer, etc.). Brush can also import `.ply` files from other sources, including the compressed format used by SuperSplat.

### Features & Capabilities

*   **How does the Brush reconstruction algorithm compare to other options?**
    *   Brush implements the core 3D Gaussian Splatting algorithm. Compared to **NeRF**, it often offers faster training and significantly faster rendering. Compared to **COLMAP**, Brush uses COLMAP *output* (poses) to perform *reconstruction* via Gaussian Splatting, whereas COLMAP itself performs SfM/MVS.
    *   Compared to **other Gaussian Splatting implementations** (like the original Inria version or `gsplat`), Brush prioritizes cross-platform portability (Rust, `wgpu`) over CUDA dependency. Performance aims to be competitive (see Benchmarks). It may not implement all the very latest research extensions to the core algorithm.
*   **Can Brush handle dynamic scenes (4D)?**
    *   Not directly in the core training loop. However, Brush *can view* animated sequences if they are provided as a `.zip` archive of sequential `.ply` files or a special `.ply` with delta frames, as used by projects like [cat-4D](https://cat-4d.github.io/) and [Cap4D](https://felixtaubner.github.io/cap4d/).
*   **Does Brush support large-scale scenes and progressive loading?**
    *   The core implementation loads the entire scene/splat data into memory. There is currently no built-in support for out-of-core rendering, progressive loading, or level-of-detail systems specifically designed for extremely large scenes that exceed GPU memory limits.
*   **Are semantics for scene objects identified, masked, or labeled?**
    *   No, Brush focuses on geometric and appearance reconstruction. It does not perform semantic segmentation or object labeling.
*   **Does Brush support editing scenes?**
    *   No, there are no built-in tools for editing the reconstructed Gaussian Splats (e.g., deleting, moving, or recoloring splats).
*   **Can Brush scenes be cropped?**
    *   No, there is no built-in cropping tool within the application.
*   **Is Brush VR compatible for desktop or mobile XR?**
    *   No, there is currently no built-in support for rendering to Virtual Reality (VR) or Augmented/Mixed Reality (AR/XR) devices.
*   **Can Brush compare two splats side-by-side?**
    *   The UI does not have a specific feature to load and render two different `.ply` files side-by-side for direct comparison.
*   **Is there integration with Jupyter Notebooks or Google Colab?**
    *   No, Brush is a standalone Rust application and library. There are no official Python bindings or integrations for Jupyter/Colab environments.

### Troubleshooting

*   **Common Build/Runtime Issues:**
    *   > [!IMPORTANT]
    >   **Missing Linux Dependencies:** Ensure required development libraries are installed (e.g., Wayland/X11 libs, `fontconfig`). See [Developer Guide](getting-started/developer-guide.md#221-development-environment-setup).
    *   > [!WARNING]
    >   **WebGPU Browser Issues:** Verify browser support, update browser/drivers, check flags (`chrome://flags/#enable-unsafe-webgpu`), and check the developer console (F12) for errors.
    *   **Missing Poses/Transforms:** Check dataset integrity (COLMAP files, `transforms.json`).
    *   **Dataset Format Errors:** Double-check file paths and JSON/COLMAP format validity.
    *   **Out of Memory (OOM):** Reduce image resolution, dataset size, or use a GPU with more VRAM.
*   **Does anyone host Brush in the cloud for remote processing?**
    *   Not currently known. Brush is designed to run locally on your machine (desktop, web browser using local resources, or Android). You can try the **[Live Web Demo](https://arthurbrussee.github.io/brush-demo)**, but it runs entirely within your browser. 