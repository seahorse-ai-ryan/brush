# 1. Introduction

This section provides a high-level overview of the Brush project.

## 1.1 Project Overview

Brush is an open-source 3D reconstruction engine built with Rust, leveraging the power of **[3D Gaussian Splatting](https://repo-sam.inria.fr/fungraph/3d-gaussian-splatting/)**. Its core purpose is to create 3D scenes from posed image data.

A key goal of Brush is portability. It runs on a wide range of platforms:

*   **Desktop:** macOS, Windows, Linux
*   **Mobile:** Android
*   **Web:** Modern browsers via WebAssembly (WASM)

This cross-platform capability is achieved using WebGPU-compatible technologies (`wgpu`) and the [Burn](https://github.com/tracel-ai/burn) machine learning framework. This approach allows Brush to produce simple, dependency-free binaries, avoiding cumbersome Python/CUDA setups often required by similar tools.

Brush supports several input data formats:

*   Posed image datasets in the **COLMAP** format.
*   Posed image datasets in the **Nerfstudio format** (using `transforms.json`).
*   Images with transparency masks (alpha channel or separate mask files).
*   Standard `.ply` files containing Gaussian Splats (for viewing).
*   ZIP archives containing datasets (required for web training).
*   ZIP archives or specialized `.ply` files containing sequences of splats for animation viewing.

## 1.2 Why Brush?

Machine learning for real-time rendering holds immense potential, but most standard ML tools present challenges:
*   **Real-time Interactivity:** Traditional frameworks aren't always optimized for the demands of interactive rendering.
*   **Complex Graphics Tasks:** Integrating ML with complex graphics tasks like real-time rendering or handling dynamic scenes can be difficult with standard ML tools.
*   **Deployment:** Shipping applications with large dependencies like PyTorch/Jax/CUDA is cumbersome.

Often, this necessitates separate applications for training and inference. Brush, written in Rust using `wgpu` and `burn`, aims to overcome these hurdles. It produces simpler, dependency-free binaries, runs on a wide array of devices (including web and mobile), requires minimal setup, and integrates training and viewing.

## 1.3 Target Audience

Brush is designed for:

*   **Developers:** Individuals looking to contribute to the Brush codebase, integrate its features into other applications, or understand its implementation details.
*   **Researchers:** Academics and practitioners interested in using Brush for 3D reconstruction experiments, exploring its underlying algorithms (like Gaussian Splatting), or extending its capabilities.

## 1.4 Key Features

Brush offers a range of features for both training and viewing 3D Gaussian Splat scenes:

*   **Cross-Platform Training:** Train reconstruction models directly on Desktop, Android, and even in the Web browser (requires zipped dataset).
*   **Interactive Training & Viewing:** Visualize the 3D scene and training dynamics live, comparing the current render against training/evaluation views as the process unfolds.
*   **Masking Support:** Utilize images with transparency or separate mask files to ignore specific regions during training.
*   **Versatile Splat Viewer:** Load and view standard `.ply` splat files, including streaming data directly from a URL in the web version.
*   **Animation Viewing:** Display sequences of splats from ZIP archives or specialized `.ply` files to view dynamic scenes or animations.
*   **Command-Line Interface (CLI):** Perform training and other operations via the `brush-cli` crate, optionally launching the UI alongside (`--with-viewer`) for debugging.
*   **Rerun Integration:** Visualize additional training data and metrics using the [Rerun](https://rerun.io/) visualization tool (requires separate installation).

## 1.5 High-Level Architecture

*(A diagram showing the relationship between key crates like `brush-app`, `brush-process`, `brush-train`, `brush-render`, and `brush-dataset` can be found in the [Architecture Deep Dive](technical-deep-dive/architecture.md#313-data-flow). This diagram provides a visual overview but may be simplified.)*

---

## Where to Go Next?

*   Ready to try Brush? Head to the **[User Guide](getting-started/user-guide.md)**.
*   Want to build the code? See the **[Developer Guide](getting-started/developer-guide.md)**.
*   Curious about the components? Dive into the **[Architecture Overview](technical-deep-dive/architecture.md)**.
*   New to the core concepts? Check the **[Glossary](supporting-materials/glossary.md)**.
*   See the full documentation structure: **[Documentation Index](README.md)**. 