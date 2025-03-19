# Brush - 3D reconstruction for all

https://github.com/user-attachments/assets/b7f55b9c-8632-49f9-b34b-d5de52a7a8b0

Brush is a 3D reconstruction engine using [Gaussian splatting](https://repo-sam.inria.fr/fungraph/3d-gaussian-splatting/). It works on a wide range of systems: **macOS/windows/linux**, **AMD/Nvidia/Intel** cards, **Android**, and in a **browser**. To achieve this, it uses WebGPU compatible tech and the [Burn](https://github.com/tracel-ai/burn) machine learning framework, which has a portable [`wgpu`](https://github.com/gfx-rs/wgpu) backend.

[**Try the (experimental) web demo** <img src="https://cdn-icons-png.flaticon.com/256/888/888846.png" alt="chrome logo" width="24"/>
](https://arthurbrussee.github.io/brush-demo)
_NOTE: Only works on Chrome with WebGPU support enabled. Check [caniuse.com/webgpu](https://caniuse.com/webgpu) for current browser compatibility._

[![](https://dcbadge.limes.pink/api/server/https://discord.gg/TbxJST2BbC)](https://discord.gg/TbxJST2BbC)

https://github.com/user-attachments/assets/4c70f892-cfd2-419f-8098-b0e20dba23c7

Training & Viewing on the web

https://github.com/user-attachments/assets/d6751cb3-ff58-45a4-8321-77d3b0a7b051

Training on a pixel 7

# Why

Machine learning for real time rendering has tons of potential, but most ML tools don't align well with it: Rendering requires realtime interactivity, usually involves dynamic shapes, and it's cumbersome to ship apps with large PyTorch/Jax/CUDA deps. The usual fix is to write a separate training and inference application. Brush on the other hand, written in `rust` using `wgpu` and `burn`, can produce simple dependency free binaries, run on nearly all devices, and doesn't require any cumbersome setup.

# Documentation

For detailed documentation about Brush's architecture, components, and development, please refer to the [documentation index](docs/documentation_index.md) in the `/docs/` folder. The documentation includes:

- [Dataset Handling](docs/dataset_handling.md): How datasets are managed, loaded, and processed
- [Reconstruction Process](docs/reconstruction_process.md): The end-to-end 3D reconstruction pipeline
- [Scene Rendering Pipeline](docs/scene_rendering_pipeline.md): How 3D Gaussian Splats are rendered
- [Export Service](docs/export_service.md): The centralized export functionality
- [Development Environment](docs/development_environment.md): Setting up and working with the codebase
- [Vibe Coding Guide](docs/vibe_coding_guide.md): Coding standards and best practices
- [Browser Tools Setup](docs/browser_tools_setup.md): How to configure Browser Tools MCP for development

# Features

## Training

Brush works with _posed_ image data. It can load COLMAP data or datasets in the Nerfstudio format with a transforms.json. Training is fully supported natively, on mobile, and in a browser*.

It also supports masking images:
- Images with transparency. This will force the final splat to match the transparency of the input.
- A folder of images called 'masks'. This ignores parts of the image that are masked out.

While training you can interact with the scene and see the training dynamics live, and compare the current rendering to training or eval views as the training progresses.

(*To train in your browser, you have to load your dataset a zip).

## Viewer
Brush also works well as a splat viewer, including on the web. It can load normal .ply files. It can also stream in data from a URL (for a web app, simply append `?url=`). There's both orbit and flythrough controls.

Brush also can load .zip of splat files to display them as an animation, or a special ply that includes delta frames. This was used for [cat-4D](https://cat-4d.github.io/) and [Cap4D](https://felixtaubner.github.io/cap4d/)!

## CLI
Brush can be used as a CLI. Run `brush --help` to get an overview. Every CLI command can work with `--with-viewer` which also opens the UI, for easy debugging.

## Rerun

https://github.com/user-attachments/assets/f679fec0-935d-4dd2-87e1-c301db9cdc2c

While training, additional data can be visualized with the excellent [rerun](https://rerun.io/). To install rerun on your machine, please follow their [instructions](https://rerun.io/docs/getting-started/installing-viewer). Open the ./brush_blueprint.rbl in the viewer for best results.

## Building Brush
First install rust 1.82+. You can run tests with `cargo test --all`. Brush uses the wonderful [rerun](https://rerun.io/) for additional visualizations while training, run `cargo install rerun-cli` if you want to use it.

> **Note**: For a comprehensive guide on setting up your development environment, including all required dependencies and tools, see the [Development Environment Setup Guide](docs/development_environment.md).

### Windows/macOS/Linux
Simply `cargo run` or `cargo run --release` from the workspace root. Brush can also be used as a CLI, run `cargo run --release -- --help` to use the CLI directly from source. See the notes about the CLI in the features section.

### Web
This project uses [`trunk`](https://github.com/trunk-rs/trunk) to build for the web. Install trunk, and then run `trunk serve` or `trunk serve --release` to run a development server.

WebGPU is still a new standard, and as such, only browsers with WebGPU support will work. Check [caniuse.com/webgpu](https://caniuse.com/webgpu) for current browser compatibility.

### Android
See the more detailed README instructions at crates/brush-android.

## Results

| Metric | bicycle | garden | stump | room | counter | kitchen | bonsai | Average |
|--------|---------|--------|--------|-------|----------|----------|---------|----------|
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

Numbers taken from [here](https://docs.gsplat.studio/main/tests/eval.html). Note that Brush by default regularizes opacity slightly.

## Benchmarks

Rendering is generally faster than gsplat, while end-to-end training speeds are similar. You can run benchmarks of some of the kernels using `cargo bench`. For additional profiling, you can use [tracy](https://github.com/wolfpld/tracy) and run with `cargo run --release --feature=tracy`.

# Acknowledgements

[**gSplat**](https://github.com/nerfstudio-project/gsplat), for their reference version of the kernels

**Peter Hedman, George Kopanas & Bernhard Kerbl**, for the many discussions & pointers.

**The Burn team**, for help & improvements to Burn along the way

**Raph Levien**, for the [original version](https://github.com/googlefonts/compute-shader-101/pull/31) of the GPU radix sort.

# Disclaimer

This is *not* an official Google product. This repository is a forked public version of [the google-research repository](https://github.com/google-research/google-research/tree/master/brush_splat)

# Development Tools

## Web Debugging

For web development and debugging, Brush includes optional Node.js-based tools:

1. **BrowserTools MCP Server**: Captures console logs, network requests, and other browser information when developing the web version.

### Setup

To set up the development tools:

1. Make sure you have [Node.js and npm](https://nodejs.org/) installed
2. Run the setup script:
   ```bash
   ./setup-dev-tools.sh
   ```

### Usage

Start the BrowserTools MCP server:
```bash
npm run start-mcp
```

Then run the web app with Trunk:
```bash
trunk serve --no-autoreload --open=false
```

These tools are optional and only needed for web debugging.
