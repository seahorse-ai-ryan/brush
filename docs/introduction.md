# 1. Introduction

This section provides a high-level overview of the Brush project.

## 1.1. Project Overview

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

## 1.2. Target Audience

Brush is designed for:

*   **Developers:** Individuals looking to contribute to the Brush codebase, integrate its features into other applications, or understand its implementation details.
*   **Researchers:** Academics and practitioners interested in using Brush for 3D reconstruction experiments, exploring its underlying algorithms (like Gaussian Splatting), or extending its capabilities.

## 1.3. Key Features

Brush offers a range of features for both training and viewing 3D Gaussian Splat scenes:

*   **Cross-Platform Training:** Train reconstruction models directly on Desktop, Android, and even in the Web browser (requires zipped dataset).
*   **Interactive Training & Viewing:** Visualize the 3D scene and training dynamics live, comparing the current render against training/evaluation views as the process unfolds.
*   **Masking Support:** Utilize images with transparency or separate mask files to ignore specific regions during training.
*   **Versatile Splat Viewer:** Load and view standard `.ply` splat files, including streaming data directly from a URL in the web version.
*   **Animation Viewing:** Display sequences of splats from ZIP archives or specialized `.ply` files to view dynamic scenes or animations.
*   **Command-Line Interface (CLI):** Perform training and other operations via the `brush-cli` crate, optionally launching the UI alongside (`--with-viewer`) for debugging.
*   **Rerun Integration:** Visualize additional training data and metrics using the [Rerun](https://rerun.io/) visualization tool (requires separate installation).

## 1.4. High-Level Architecture Diagram

*(TODO: Refine this diagram to accurately represent dependencies and data flow. Consider replacing with a more detailed image if needed.)*

```mermaid
graph LR;
    subgraph User Interface
        UI[brush-ui (EGUI)]
    end
    subgraph Application Core
        App[brush-app]
    end
    subgraph Reconstruction
        Train[brush-train (Burn)]
        Process[brush-process]
        Dataset[brush-dataset]
        Colmap[colmap-reader]
    end
    subgraph Rendering
        RenderFwd[brush-render]
        RenderBwd[brush-render-bwd]
        Kernel[brush-kernel (WGPU/WGSL)]
        Sort[brush-sort]
        PrefixSum[brush-prefix-sum]
    end
    subgraph Command Line
        CLI[brush-cli]
    end
    subgraph Visualization
        Rerun[brush-rerun]
    end
    subgraph Platform Specific
        Android[brush-android]
    end

    User --> App
    App --> UI
    App --> Train
    App --> RenderFwd
    App --> Process
    App --> Dataset
    App --> CLI

    CLI --> Train
    CLI --> RenderFwd
    CLI --> Process
    CLI --> Dataset

    Train --> Process
    Train --> Dataset
    Train --> RenderFwd
    Train --> RenderBwd
    Train --> Kernel
    Train --> Sort
    Train --> PrefixSum
    Train --> Rerun

    Process --> Dataset
    Dataset --> Colmap

    RenderFwd --> Kernel
    RenderFwd --> Sort
    RenderFwd --> PrefixSum
    RenderBwd --> Kernel

    App --> Android
```

*(Note: This diagram shows potential high-level interactions between crates. Arrows indicate likely dependencies or data flow directions.)* 