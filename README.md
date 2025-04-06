# Brush - 3D reconstruction for all

[![Docs](https://img.shields.io/badge/Documentation-View%20Here-blue)](docs/README.md)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Discord](https://dcbadge.limes.pink/api/server/TbxJST2BbC?style=flat)](https://discord.gg/TbxJST2BbC)

## Overview

Brush is an open-source 3D reconstruction engine using Gaussian splatting, built with Rust. It leverages the [Burn](https://github.com/burn-rs/burn) framework and custom WGSL kernels for high portability and performance across desktop (Windows, macOS, Linux), web (WASM), and Android.

> **Note:** This repository is an experimental fork (derived from [ArthurBrussee/brush](https://github.com/ArthurBrussee/brush), originating from [Google Research](https://github.com/google-research/google-research/tree/master/brush_splat)) representing ongoing development and exploration. While functional, it may not yet implement all the latest extensions to the Gaussian splatting technique.

## Visual Showcase

<!-- TODO: Replace placeholders with actual hosted video/gif URLs -->

**Desktop Training:**

<video src="https://github.com/user-attachments/assets/b7f55b9c-8632-49f9-b34b-d5de52a7a8b0" controls width="100%"></video>
*Live training view showing the interactive UI, scene reconstruction, and dataset visualization.*

**Web Viewer & Training:**

<video src="https://github.com/user-attachments/assets/4c70f892-cfd2-419f-8098-b0e20dba23c7" controls width="100%"></video>
*Left: Web viewer rendering a pre-trained scene. Right: Web UI training a new scene.*

<!-- Optional: Add Android / Rerun videos here or link to docs sections containing them -->

## Core Features

*   Load datasets in **COLMAP** and **Synthetic NeRF** (`transforms.json`) formats.
*   Train Gaussian Splatting models from scratch via UI or CLI.
*   Real-time, cross-platform viewing (Desktop, Web, Android).
*   Visualize training progress live using [Rerun](https://www.rerun.io/) integration (`--features=rerun`).

## Getting Started

*   **Try the Web Demo:** [**arthurbrussee.github.io/brush-demo/**](https://arthurbrussee.github.io/brush-demo/)
    > (Requires modern browser with WebGPU support)
*   **Run Natively (Basic):**
    ```bash
    # Clone the repo (replace with your fork URL if needed)
    git clone https://github.com/ArthurBrussee/brush.git
    cd brush
    # Build and run (Release mode recommended for performance)
    cargo run --bin brush_app --release
    ```
*   **Explore the Full Documentation:** ➡️ [**docs/README.md**](./docs/README.md) ⬅️

## Community

Join the discussion on the [Brush Discord Server](https://discord.gg/TbxJST2BbC).

## Acknowledgements

Inspired by and building upon foundational work:
*   [3D Gaussian Splatting for Real-Time Radiance Field Rendering](https://repo-sam.inria.fr/fungraph/3d-gaussian-splatting/) (INRIA)
*   Original Google Research team & paper contributors.
*   **Arthur Brussee** ([@ArthurBrussee](https://github.com/ArthurBrussee)) for significant contributions and maintenance of the upstream fork.
*   The **Burn** team ([@burn-rs](https://github.com/burn-rs)) for the ML framework and support.
*   The **Rerun** team ([@rerun-io](https://github.com/rerun-io)) for the visualization tool and stewardship of Egui.

## Disclaimer

This is *not* an official Google product. It is an open-source project.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](./LICENSE) file.
