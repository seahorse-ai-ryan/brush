# 5.2 Glossary

Definitions of key terms used in the Brush project and documentation.

**Index:** [A](#a) | [B](#b) | [C](#c) | [D](#d) | [E](#e) | [G](#g) | [L](#l) | [N](#n) | [O](#o) | [P](#p) | [R](#r) | [S](#s) | [T](#t) | [V](#v) | [W](#w) | [Z](#z)

---

### A {#a}

**Autodiff (Automatic Differentiation)**
: A technique used by frameworks like Burn to automatically compute derivatives (gradients) of functions, typically represented as computation graphs. Essential for training models by calculating how parameter changes affect the loss.

---

### B {#b}

**Backend (Burn)**
: In Burn, refers to the specific computation engine used to run tensor operations (e.g., `Wgpu` for GPU via WebGPU, `NdArray` for CPU). Brush primarily uses the `Wgpu` backend.

**`brush` (CLI)**
: The command-line-only executable produced when specifically building the `brush-cli` crate (e.g., `cargo build -p brush-cli`). It resides at `target/.../brush` and offers core functionality without launching a UI by default.

**`brush_app`**
: The main executable for Brush, typically built via `cargo build` or run via `cargo run --bin brush_app` in the workspace root. It provides the full Graphical User Interface (UI) and also accepts command-line arguments.

**Burn**
: A deep learning framework written in Rust, used by Brush for model definition, automatic differentiation, optimization, and GPU computation via its `wgpu` backend. See [Core Technologies](technical-deep-dive/core-technologies.md#343-burn).

---

### C {#c}

**Cargo**
: The official Rust build tool and package manager. Used to compile Brush, manage dependencies, run tests, and generate documentation.

**Checkpoint (Brush)**
: A snapshot of the state of the 3D Gaussians saved during the optimization (training) process, typically stored as a `.ply` file. Checkpoints allow training progress to be saved periodically (controlled by the `export_every` setting) and can be used to view intermediate results or potentially resume training by loading the saved Gaussians.

**COLMAP**
: A popular open-source Structure-from-Motion (SfM) and Multi-View Stereo (MVS) software. Brush can load datasets processed by COLMAP (which include camera poses).

**Crate**
: A Rust package, the smallest unit of compilation. Brush is organized into multiple crates within a Cargo workspace.

---

### D {#d}

**Densification**
: A core step in Gaussian Splatting training where new Gaussians are created (by splitting or cloning existing ones) in areas that are under-reconstructed, helping to add detail.

---

### E {#e}

**Eframe**
: A framework for the EGUI library that provides backends for running EGUI applications on native platforms and the web. See [Core Technologies](../technical-deep-dive/core-technologies.md#344-egui--eframe).

**EGUI**
: An immediate mode GUI library for Rust, used by Brush to create its user interface (`brush-ui`). See [Core Technologies](../technical-deep-dive/core-technologies.md#344-egui--eframe).

---

### G {#g}

**Gaussian**
: The fundamental element used in the Gaussian Splatting technique. Represents a point in 3D space with properties like position, shape (defined by covariance), color (often using Spherical Harmonics), and opacity. The 3D scene is reconstructed as a collection of millions of these Gaussians.

**Gaussian Splatting (3DGS)**
: A 3D reconstruction and rendering technique that represents scenes using millions of 3D Gaussians. See [Reconstruction Pipeline](../technical-deep-dive/reconstruction-pipeline.md) and [Gaussian Splat Rendering](../technical-deep-dive/rendering-pipeline.md).

---

### L {#l}

**Live Update Splats**
: A toggle in the UI (Scene Panel) that controls whether the 3D view updates in real-time during training to show the refining splats. Disabling it can sometimes improve performance slightly.

**Loss Function**
: In the context of training Brush models, a mathematical function that measures the difference (error) between the images rendered from the current set of 3D Gaussians and the input ground truth images. The goal of optimization is to adjust Gaussian parameters to minimize this loss.

---

### N {#n}

**Native**
: Refers to the version of an application compiled to run directly on a specific operating system (like Windows, macOS, Linux) without needing a web browser. Brush provides native executables for desktop platforms.

**NeRF (Neural Radiance Field)**
: A method for novel view synthesis that represents a 3D scene implicitly using a neural network. While related in goal to Gaussian Splatting, NeRF uses a different underlying representation and typically has different performance characteristics.

**Nerfstudio Format**
: A dataset format popularized by the Nerfstudio project, typically involving a `transforms.json` file defining camera poses and image paths. Brush supports loading datasets in this format.

---

### O {#o}

**Optimization**
: The iterative process used in Brush to refine the parameters of the 3D Gaussians (e.g., position, shape, color, opacity). It uses techniques like gradient descent (guided by the loss function and autodiff) to improve the match between the rendered scene and the input images.

---

### P {#p}

**PLY File**
: A standard 3D data format (`.ply`) commonly used for storing geometric data like point clouds or polygons. In Brush, `.ply` files are the primary format for saving and loading the collection of trained 3D Gaussians and their associated parameters (position, rotation, scale, color/SH, opacity). These files represent the captured scene's radiance field and can be used for viewing, further training (as checkpoints), or potentially in other applications compatible with the Gaussian Splatting PLY structure. See the [User Guide](../getting-started/user-guide.md#workflow-4-exporting-results) for export details.

**Presets Panel**
: A specific panel or tab within the Brush UI dedicated to loading pre-configured example datasets (like MipNeRF or Synthetic Blender scenes) easily.

**Pruning**
: A core step in Gaussian Splatting training where Gaussians that are deemed insignificant (e.g., very small or almost transparent) are removed to keep the model efficient.

---

### R {#r}

**Radiance Field**
: A function describing the flow of light energy at every point in 3D space and in every direction. Techniques like NeRF and 3D Gaussian Splatting aim to learn and render representations of these fields from input images.

**Real-time Rendering**
: The process of generating and displaying images (frames) from a 3D scene at a high enough frame rate (e.g., 30-60 frames per second) to create the perception of smooth, interactive motion. Brush aims for real-time rendering of the Gaussian splat scenes.

**Rerun**
: A visualization toolkit for multimodal data, particularly useful for time-series data like training runs. Brush integrates with Rerun (`brush-rerun` crate) to log scene data, splats, training metrics, and memory usage. ([rerun.io](https://rerun.io/))

**Rust**
: A systems programming language focused on safety, speed, and concurrency, used as the primary language for Brush. See [Core Technologies](technical-deep-dive/core-technologies.md#341-rust).

**`rust-toolchain.toml`**
: A configuration file used in Rust projects to specify the exact version of the Rust compiler and associated tools (`rustup` toolchain) required to build the project. Ensures build consistency for all developers.

---

### S {#s}

**Scene Panel**
: The main viewport component within the Brush Graphical User Interface (UI) where the 3D reconstructed scene, composed of Gaussian splats, is displayed and can be interactively navigated (rotated, panned, zoomed).

**Shader**
: A type of program that runs on the Graphics Processing Unit (GPU). Shaders are used in graphics pipelines to perform calculations for rendering, such as determining the final color of pixels (fragment/pixel shader) or the position of geometry vertices (vertex shader). Brush uses shaders written in WGSL for rendering Gaussian splats via WGPU.

**Spherical Harmonics (SH)**
: Mathematical functions defined on the surface of a sphere, used in computer graphics to represent view-dependent appearance efficiently. In Gaussian Splatting, SH coefficients are stored per Gaussian to model how their color changes based on viewing direction.

**Splat**
: The 2D projection of a single 3D Gaussian onto the image plane (screen) during the rendering process. The final rendered image is created by blending (compositing) potentially millions of these overlapping 2D splats based on their color and opacity. *Note: This term refers specifically to the 2D projected primitive, not the collection of 3D Gaussians representing the trained scene (which is often stored in a PLY file and represents a radiance field).*

**Structure from Motion (SfM)**
: A photogrammetry technique used to estimate 3D structure and camera poses from 2D image sequences. Often used as a prerequisite for methods like Gaussian Splatting.

---

### T {#t}

**Tensor**
: A multi-dimensional array, the primary data structure used in machine learning frameworks like Burn to store and manipulate data (e.g., Gaussian parameters, image data, gradients).

**Trunk**
: A build tool and asset bundler for Rust WebAssembly applications, used to build the web version of Brush. See [Core Technologies](technical-deep-dive/core-technologies.md#342-webassembly-wasm).

---

### V {#v}

**Virtual File System (VFS)**
: An abstraction layer (`brush-dataset::brush_vfs`) used by Brush to handle loading datasets from different sources (local directories, ZIP archives, URLs) with a unified interface.

---

### W {#w}

**WASM (WebAssembly)**
: A binary instruction format enabling high-performance code (like Rust) to run in web browsers. See [Core Technologies](technical-deep-dive/core-technologies.md#342-webassembly-wasm).

**WGPU**
: A modern graphics and compute API specification and its Rust implementation (`wgpu`), providing portable access to GPU capabilities across different platforms and native APIs. See [Core Technologies](technical-deep-dive/core-technologies.md#345-wgpu--wgsl).

**WGSL**
: WebGPU Shading Language, the language used to write shaders for WGPU. See [Core Technologies](technical-deep-dive/core-technologies.md#345-wgpu--wgsl).

---

### Z {#z}

**Zen Mode**
: A viewing mode (controllable via URL parameter `zen=true` in the web demo) that maximizes the Scene panel and minimizes or hides other UI elements for an immersive viewing experience. 