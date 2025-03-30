# Brush - 3D reconstruction for all

[![Docs](https://img.shields.io/badge/Documentation-View%20Here-blue)](docs/index.md)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE) <!-- Assuming Apache 2.0 based on Google Research -->
[![Discord](https://dcbadge.limes.pink/api/server/TbxJST2BbC?style=flat)](https://discord.gg/TbxJST2BbC)

## Overview

Brush is an open-source 3D reconstruction engine using Gaussian splatting, built with Rust. It leverages the [Burn](https://github.com/burn-rs/burn) framework and custom WGSL kernels for high portability and performance.

**Key Goals:**

*   **Portable:** Run training and rendering across macOS, Windows, Linux, Web (WASM), and Android.
*   **Flexible:** Support various input dataset formats.
*   **Fast:** Optimized rendering and training pipeline using modern GPU techniques.

> [!NOTE]
> This project originated from Google Research and is currently maintained as a fork with ongoing development. While functional and capable, it serves as a foundation and may not yet implement all the latest extensions to the Gaussian splatting technique.

<video src="https://github.com/user-attachments/assets/b7f55b9c-8632-49f9-b34b-d5de52a7a8b0" controls width="100%"></video>
*Live training view showing the interactive UI, scene reconstruction, and dataset visualization.* 

<video src="https://github.com/user-attachments/assets/4c70f892-cfd2-419f-8098-b0e20dba23c7" controls width="100%"></video>
*Left: Web viewer rendering a pre-trained scene. Right: Web UI training a new scene.* 

<video src="https://github.com/user-attachments/assets/d6751cb3-ff58-45a4-8321-77d3b0a7b051" controls width="100%"></video>
*Brush training live on an Android device (Pixel 7).* 

<video src="https://github.com/user-attachments/assets/f679fec0-935d-4dd2-87e1-c301db9cdc2c" controls width="100%"></video>
*Rerun viewer showing detailed training visualization (losses, splat counts, 3D view) for the LEGO dataset.*

➡️ **Learn More:** [**Introduction & Project Overview**](docs/introduction.md)

## Features

*   Load datasets in **COLMAP** and **Synthetic NeRF** (`transforms.json`) formats.
*   Train Gaussian splatting models from scratch.
*   Real-time rendering of trained models.
*   Visualize training progress live using [Rerun](https://www.rerun.io/).
*   Interactive desktop and web viewer using `egui`.
*   Command-line interface for processing (`brush_app`). <!-- Assuming brush_app is the CLI -->
*   Cross-platform compatibility (Desktop, Web, Android).

➡️ **See More:** [**Full Feature List**](docs/introduction.md#13-key-features) | [**Supported Platforms**](docs/getting_started/user_guide.md#213-hardware--software-requirements)

## Getting Started

### Prerequisites

*   **Rust:** Install Rust 1.78+ via [rustup](https://rustup.rs/).
*   **Platform Dependencies:** See the [Setup Guide](docs/getting_started/developer_guide.md#221-development-environment-setup) for Linux, macOS, or Windows requirements.
*   **(Web):** `trunk` (`cargo install trunk`) and the WASM target (`rustup target add wasm32-unknown-unknown`).
*   **(Visualization):** [Rerun SDK](https://www.rerun.io/docs/getting-started/installing-the-sdk) (Optional, for live training view).

### Quick Start (Desktop)

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/ArthurBrussee/brush.git # Or your fork's URL
    cd brush
    ```
2.  **Run the desktop application:**
    *   Debug: `cargo run --bin brush_app`
    *   Release: `cargo run --bin brush_app --release`
    *   With Rerun: `cargo run --bin brush_app --release --features=rerun`

➡️ **Detailed Guides:** [**Installation**](docs/getting_started/user_guide.md#211-installation) | [**Building for Web/Android**](docs/getting_started/developer_guide.md#222-building-the-project) | [**Basic Workflows**](docs/getting_started/user_guide.md#212-basic-workflows-step-by-step)

> [!WARNING]
> The public web demo is experimental. As of early 2025, it requires Chrome 131+ due to WebGPU and subgroup requirements. Firefox/Safari support may vary. See the User Guide for details.

## Example Data

Example datasets (like `bicycle`, `bonsai`, `counter`, synthetic NeRF scenes, etc.) can be loaded directly via the "Presets" panel within the `brush_app` application.

➡️ **Learn More:** [**Using Presets (User Guide)**](docs/getting_started/user_guide.md#workflow-1-loading-a-dataset) <!-- Adjust link if needed -->

## Technical Deep Dive

Brush employs a multi-crate architecture for modularity. Key technologies include:

*   **Gaussian Splatting:** The core reconstruction and rendering algorithm.
*   **Burn:** ML framework providing tensor operations and abstractions.
*   **wgpu:** Graphics API abstraction for cross-platform GPU access (Vulkan, Metal, DX12, WebGPU).
*   **WGSL:** Custom compute and rendering kernels.
*   **egui:** Immediate-mode GUI for the viewer application.
*   **naga & naga-oil:** Shader processing and management.

The rendering pipeline uses GPU-accelerated sorting (`brush-sort`) and prefix sums (`brush-prefix-sum`) for efficiency. Training involves forward and backward passes with gradient aggregation suitable for WebGPU compatibility.

➡️ **Explore Further:** [**Architecture**](docs/technical_deep_dive/architecture.md) | [**Reconstruction Pipeline**](docs/technical_deep_dive/reconstruction_pipeline.md) | [**Rendering Details**](docs/technical_deep_dive/gaussian_splat_rendering.md) | [**Core Technologies**](docs/technical_deep_dive/core_technologies.md)

## Benchmarks & Performance

Rendering performance aims to be competitive with other leading Gaussian splatting implementations. Training performance is continually improving.

For detailed performance metrics and comparisons (which may evolve), please see:

➡️ **[Benchmarks](docs/benchmarks.md)**

Profiling is possible using `tracy`: run with `cargo run --release --features=tracy`.

## Contributing

We welcome contributions! Please read our [**Contribution Guidelines**](CONTRIBUTING.md) before submitting pull requests or issues.

## Acknowledgements

Inspired by and building upon foundational work:
*   [3D Gaussian Splatting for Real-Time Radiance Field Rendering](https://repo-sam.inria.fr/fungraph/3d-gaussian-splatting/) (INRIA)
*   [Mip-NeRF 360](https://jonbarron.info/mipnerf360/)

Project Contributors & Support:
*   Original Google Research team.
*   **Arthur Brussee** ([@ArthurBrussee](https://github.com/ArthurBrussee)) for significant contributions and maintenance.
*   **Raph Levien** (original GPU radix sort).
*   **Peter Hedman & George Kopanas** (discussion & inspiration).
*   **The Burn team** (support with kernel integration).

## Disclaimer

This is *not* an official Google product. It is an open-source project.
